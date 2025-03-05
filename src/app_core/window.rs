use std::os::raw::c_void;
use std::ffi::CString;
use bitflags::bitflags;
use crate::app_core::ffi::{ULWindow, ULMonitor, ulCreateWindow, ulDestroyWindow, 
                          ulWindowSetCloseCallback, ulWindowSetResizeCallback, ulWindowGetScreenWidth,
                          ulWindowGetWidth, ulWindowGetScreenHeight, ulWindowGetHeight,
                          ulWindowMoveTo, ulWindowMoveToCenter, ulWindowGetPositionX, ulWindowGetPositionY,
                          ulWindowIsFullscreen, ulWindowGetScale, ulWindowSetTitle, ulWindowSetCursor,
                          ulWindowShow, ulWindowHide, ulWindowIsVisible, ulWindowClose,
                          ulWindowScreenToPixels, ulWindowPixelsToScreen, ulWindowGetNativeHandle};
use crate::app_core::monitor::Monitor;
use crate::ul::Cursor;

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
extern "C" fn close_callback_wrapper<T: CloseCallback>(user_data: *mut c_void, window_ptr: ULWindow) {
    unsafe {
        let callback = &*(user_data as *const T);
        let window = Window::from_raw(window_ptr);
        callback.on_close(&window);
        
        // Prevent drop of window to avoid deallocation
        std::mem::forget(window);
    }
}

extern "C" fn resize_callback_wrapper<T: ResizeCallback>(
    user_data: *mut c_void, 
    window_ptr: ULWindow, 
    width: u32, 
    height: u32
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let window = Window::from_raw(window_ptr);
        callback.on_resize(&window, width, height);
        
        // Prevent drop of window to avoid deallocation
        std::mem::forget(window);
    }
}

/// A structure that holds callback data and keeps it alive.
struct CallbackData<T: ?Sized> {
    data: Box<T>,
}

impl<T> CallbackData<T> {
    fn new(data: T) -> *mut c_void {
        let data = Box::new(CallbackData { data: Box::new(data) });
        Box::into_raw(data) as *mut c_void
    }
    
    unsafe fn drop(ptr: *mut c_void) {
        if !ptr.is_null() {
            let _ = Box::from_raw(ptr as *mut CallbackData<T>);
        }
    }
}

/// A window for displaying content.
pub struct Window {
    raw: ULWindow,
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
    pub fn new(monitor: &Monitor, width: u32, height: u32, fullscreen: bool, window_flags: WindowFlags) -> Self {
        unsafe {
            let raw = ulCreateWindow(monitor.raw(), width, height, fullscreen, window_flags.bits());
            Self { raw }
        }
    }
    
    /// Create a Window from a raw ULWindow pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULWindow created by the AppCore API.
    pub unsafe fn from_raw(raw: ULWindow) -> Self {
        Self { raw }
    }
    
    /// Get a reference to the raw ULWindow.
    pub fn raw(&self) -> ULWindow {
        self.raw
    }
    
    /// Set a callback to be notified when the window closes.
    pub fn set_close_callback<T: 'static + CloseCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulWindowSetCloseCallback(
                self.raw,
                std::mem::transmute(close_callback_wrapper::<T> as extern "C" fn(*mut c_void, *mut _)),
                user_data
            );
        }
    }
    
    /// Set a callback to be notified when the window resizes.
    pub fn set_resize_callback<T: 'static + ResizeCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulWindowSetResizeCallback(
                self.raw,
                std::mem::transmute(resize_callback_wrapper::<T> as extern "C" fn(*mut c_void, *mut _, u32, u32)),
                user_data
            );
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
    pub fn set_title(&self, title: &str) {
        let c_title = CString::new(title).unwrap();
        unsafe {
            ulWindowSetTitle(self.raw, c_title.as_ptr());
        }
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