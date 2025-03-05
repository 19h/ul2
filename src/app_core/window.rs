use crate::app_core::ffi::{
    ULWindow, ulCreateWindow, ulDestroyWindow, ulWindowClose, ulWindowGetHeight,
    ulWindowGetNativeHandle, ulWindowGetPositionX, ulWindowGetPositionY, ulWindowGetScale,
    ulWindowGetScreenHeight, ulWindowGetScreenWidth, ulWindowGetWidth, ulWindowHide,
    ulWindowIsFullscreen, ulWindowIsVisible, ulWindowMoveTo, ulWindowMoveToCenter,
    ulWindowPixelsToScreen, ulWindowScreenToPixels, ulWindowSetCloseCallback, ulWindowSetCursor,
    ulWindowSetResizeCallback, ulWindowSetTitle, ulWindowShow,
};
use crate::app_core::error::Error;
use crate::app_core::monitor::Monitor;
use crate::ul::Cursor;
use bitflags::bitflags;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};

bitflags! {
    /// Window creation flags.
    #[repr(C)]
    pub struct WindowFlags: u32 {
        const BORDERLESS = 1 << 0;
        const TITLED = 1 << 1;
        const RESIZABLE = 1 << 2;
        const MAXIMIZABLE = 1 << 3;
        const HIDDEN = 1 << 4;
    }
}

/// Callback for window close events.
pub trait CloseCallback: Send {
    fn on_close(&self, window: &Window);
}

/// Callback for window resize events.
pub trait ResizeCallback: Send {
    fn on_resize(&self, window: &Window, width: u32, height: u32);
}

// Callback wrappers for the C API
extern "C" fn close_callback_wrapper<T: CloseCallback>(
    user_data: *mut c_void,
    window_ptr: ULWindow,
) {
    unsafe {
        if !user_data.is_null() && !window_ptr.is_null() {
            let callback = &*(user_data as *const T);
            let window = Window::from_raw_temp(window_ptr);
            callback.on_close(&window);
        }
    }
}

extern "C" fn resize_callback_wrapper<T: ResizeCallback>(
    user_data: *mut c_void,
    window_ptr: ULWindow,
    width: u32,
    height: u32,
) {
    unsafe {
        if !user_data.is_null() && !window_ptr.is_null() {
            let callback = &*(user_data as *const T);
            let window = Window::from_raw_temp(window_ptr);
            callback.on_resize(&window, width, height);
        }
    }
}

// Thread-safe callback storage
struct CallbackStorage<T: ?Sized> {
    data: Option<Box<T>>,
}

impl<T> CallbackStorage<T> {
    fn new() -> Self {
        Self { data: None }
    }

    fn set(&mut self, data: T) -> *mut c_void {
        self.data = Some(Box::new(data));
        self.data.as_ref().unwrap().as_ref() as *const T as *mut c_void
    }

    fn clear(&mut self) {
        self.data = None;
    }
}

/// A window for displaying content.
pub struct Window {
    raw: ULWindow,
    close_callback: Arc<Mutex<CallbackStorage<dyn CloseCallback>>>,
    resize_callback: Arc<Mutex<CallbackStorage<dyn ResizeCallback>>>,
}

impl Window {
    /// Create a new window.
    ///
    /// # Arguments
    ///
    /// * `monitor` - The monitor to create the window on
    /// * `width` - The width (in screen coordinates)
    /// * `height` - The height (in screen coordinates)
    /// * `fullscreen` - Whether or not the window is fullscreen
    /// * `window_flags` - Various window flags
    ///
    /// # Returns
    ///
    /// A Result containing the Window if successful, or an Error if window creation failed.
    pub fn new(
        monitor: &Monitor,
        width: u32,
        height: u32,
        fullscreen: bool,
        window_flags: WindowFlags,
    ) -> Result<Self, Error> {
        unsafe {
            let raw = ulCreateWindow(
                monitor.raw(),
                width,
                height,
                fullscreen,
                window_flags.bits(),
            );
            if raw.is_null() {
                return Err(Error::CreationFailed("Failed to create window"));
            }
            
            Ok(Self { 
                raw,
                close_callback: Arc::new(Mutex::new(CallbackStorage::new())),
                resize_callback: Arc::new(Mutex::new(CallbackStorage::new())),
            })
        }
    }

    /// Create a Window from a raw ULWindow pointer for permanent usage.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULWindow created by the AppCore API.
    /// This function does not verify if the pointer is valid.
    ///
    /// # Returns
    ///
    /// A Window instance.
    pub unsafe fn from_raw(raw: ULWindow) -> Self {
        Self { 
            raw,
            close_callback: Arc::new(Mutex::new(CallbackStorage::new())),
            resize_callback: Arc::new(Mutex::new(CallbackStorage::new())),
        }
    }

    /// Create a temporary Window from a raw ULWindow pointer.
    /// This is used in callback wrappers and does not take ownership.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULWindow created by the AppCore API.
    /// This function does not verify if the pointer is valid.
    ///
    /// # Returns
    ///
    /// A Window instance that doesn't own the raw pointer.
    unsafe fn from_raw_temp(raw: ULWindow) -> Self {
        Self { 
            raw,
            close_callback: Arc::new(Mutex::new(CallbackStorage::new())),
            resize_callback: Arc::new(Mutex::new(CallbackStorage::new())),
        }
    }

    /// Get a reference to the raw ULWindow.
    pub fn raw(&self) -> ULWindow {
        self.raw
    }

    /// Set a callback to be notified when the window closes.
    ///
    /// # Returns
    ///
    /// A Result containing Ok(()) if successful, or an Error if callback registration failed.
    pub fn set_close_callback<T: 'static + CloseCallback>(&self, callback: T) -> Result<(), Error> {
        unsafe {
            let mut close_cb = match self.close_callback.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(Error::CallbackRegistrationFailed("Failed to acquire close callback lock")),
            };
            
            let user_data = close_cb.set(callback);
            
            ulWindowSetCloseCallback(
                self.raw,
                Some(close_callback_wrapper::<T>),
                user_data,
            );
            
            Ok(())
        }
    }

    /// Clear the close callback.
    ///
    /// # Returns
    ///
    /// A Result containing Ok(()) if successful, or an Error if callback clearing failed.
    pub fn clear_close_callback(&self) -> Result<(), Error> {
        unsafe {
            let mut close_cb = match self.close_callback.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(Error::InvalidOperation("Failed to acquire close callback lock")),
            };
            
            close_cb.clear();
            ulWindowSetCloseCallback(self.raw, None, std::ptr::null_mut());
            
            Ok(())
        }
    }

    /// Set a callback to be notified when the window resizes.
    ///
    /// # Returns
    ///
    /// A Result containing Ok(()) if successful, or an Error if callback registration failed.
    pub fn set_resize_callback<T: 'static + ResizeCallback>(&self, callback: T) -> Result<(), Error> {
        unsafe {
            let mut resize_cb = match self.resize_callback.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(Error::CallbackRegistrationFailed("Failed to acquire resize callback lock")),
            };
            
            let user_data = resize_cb.set(callback);
            
            ulWindowSetResizeCallback(
                self.raw,
                Some(resize_callback_wrapper::<T>),
                user_data,
            );
            
            Ok(())
        }
    }

    /// Clear the resize callback.
    ///
    /// # Returns
    ///
    /// A Result containing Ok(()) if successful, or an Error if callback clearing failed.
    pub fn clear_resize_callback(&self) -> Result<(), Error> {
        unsafe {
            let mut resize_cb = match self.resize_callback.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(Error::InvalidOperation("Failed to acquire resize callback lock")),
            };
            
            resize_cb.clear();
            ulWindowSetResizeCallback(self.raw, None, std::ptr::null_mut());
            
            Ok(())
        }
    }

    /// Get window width (in screen coordinates).
    pub fn screen_width(&self) -> u32 {
        unsafe { ulWindowGetScreenWidth(self.raw) }
    }

    /// Get window width (in pixels).
    pub fn width(&self) -> u32 {
        unsafe { ulWindowGetWidth(self.raw) }
    }

    /// Get window height (in screen coordinates).
    pub fn screen_height(&self) -> u32 {
        unsafe { ulWindowGetScreenHeight(self.raw) }
    }

    /// Get window height (in pixels).
    pub fn height(&self) -> u32 {
        unsafe { ulWindowGetHeight(self.raw) }
    }

    /// Move the window to a new position (in screen coordinates) relative to
    /// the top-left of the monitor area.
    pub fn move_to(&self, x: i32, y: i32) {
        unsafe {
            ulWindowMoveTo(self.raw, x, y);
        }
    }

    /// Move the window to the center of the monitor.
    pub fn move_to_center(&self) {
        unsafe {
            ulWindowMoveToCenter(self.raw);
        }
    }

    /// Get the x-position of the window (in screen coordinates) relative to
    /// the top-left of the monitor area.
    pub fn position_x(&self) -> i32 {
        unsafe { ulWindowGetPositionX(self.raw) }
    }

    /// Get the y-position of the window (in screen coordinates) relative to
    /// the top-left of the monitor area.
    pub fn position_y(&self) -> i32 {
        unsafe { ulWindowGetPositionY(self.raw) }
    }

    /// Check if the window is fullscreen.
    pub fn is_fullscreen(&self) -> bool {
        unsafe { ulWindowIsFullscreen(self.raw) }
    }

    /// Get the DPI scale of the window.
    pub fn scale(&self) -> f64 {
        unsafe { ulWindowGetScale(self.raw) }
    }

    /// Set the window title.
    ///
    /// # Arguments
    ///
    /// * `title` - The new window title
    ///
    /// # Returns
    ///
    /// A Result containing Ok(()) if successful, or an Error if title setting failed.
    pub fn set_title(&self, title: &str) -> Result<(), Error> {
        let c_title = match CString::new(title) {
            Ok(s) => s,
            Err(_) => return Err(Error::InvalidArgument("Title contains null bytes")),
        };
        
        unsafe {
            ulWindowSetTitle(self.raw, c_title.as_ptr());
        }
        
        Ok(())
    }

    /// Set the cursor for the window.
    pub fn set_cursor(&self, cursor: Cursor) {
        unsafe {
            ulWindowSetCursor(self.raw, cursor);
        }
    }

    /// Show the window (if it was previously hidden).
    pub fn show(&self) {
        unsafe {
            ulWindowShow(self.raw);
        }
    }

    /// Hide the window.
    pub fn hide(&self) {
        unsafe {
            ulWindowHide(self.raw);
        }
    }

    /// Check if the window is currently visible (not hidden).
    pub fn is_visible(&self) -> bool {
        unsafe { ulWindowIsVisible(self.raw) }
    }

    /// Close the window.
    pub fn close(&self) {
        unsafe {
            ulWindowClose(self.raw);
        }
    }

    /// Convert screen coordinates to pixels using the current DPI scale.
    pub fn screen_to_pixels(&self, val: i32) -> i32 {
        unsafe { ulWindowScreenToPixels(self.raw, val) }
    }

    /// Convert pixels to screen coordinates using the current DPI scale.
    pub fn pixels_to_screen(&self, val: i32) -> i32 {
        unsafe { ulWindowPixelsToScreen(self.raw, val) }
    }

    /// Get the underlying native window handle.
    ///
    /// # Returns
    ///
    /// * On Windows: HWND
    /// * On macOS: NSWindow*
    /// * On Linux: GLFWwindow*
    pub fn native_handle(&self) -> *mut c_void {
        unsafe { ulWindowGetNativeHandle(self.raw) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyWindow(self.raw);
            }
        }
    }
}