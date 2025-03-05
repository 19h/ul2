use crate::app_core::ffi::{
    ULApp, ulAppGetMainMonitor, ulAppGetRenderer, ulAppIsRunning, ulAppQuit, ulAppRun,
    ulAppSetUpdateCallback, ulCreateApp, ulDestroyApp,
};
use crate::app_core::monitor::Monitor;
use crate::app_core::settings::Settings;
use crate::ul::{Config, Renderer};
use std::os::raw::c_void;

/// Callback for app updates.
pub trait UpdateCallback: Send {
    fn on_update(&self);
}

// Callback wrapper for the C API
extern "C" fn update_callback_wrapper<T: UpdateCallback>(user_data: *mut c_void) {
    unsafe {
        let callback = &*(user_data as *const T);
        callback.on_update();
    }
}

/// A structure that holds callback data and keeps it alive.
struct CallbackData<T: ?Sized> {
    data: Box<T>,
}

impl<T> CallbackData<T> {
    fn new(data: T) -> *mut c_void {
        let data = Box::new(CallbackData {
            data: Box::new(data),
        });
        Box::into_raw(data) as *mut c_void
    }

    unsafe fn drop(ptr: *mut c_void) {
        unsafe {
            if !ptr.is_null() {
                let _ = Box::from_raw(ptr as *mut CallbackData<T>);
            }
        }
    }
}

/// The main application class for AppCore, responsible for managing the renderer,
/// run loop, windows, and platform-specific operations.
pub struct App {
    raw: ULApp,
}

impl App {
    /// Create a new App instance with the specified settings and config.
    ///
    /// You should only create one App instance per application lifetime.
    ///
    /// # Arguments
    ///
    /// * `settings` - Settings to customize App runtime behavior
    /// * `config` - Config options for the Ultralight renderer
    pub fn new(settings: &Settings, config: &Config) -> Self {
        unsafe {
            let raw = ulCreateApp(settings.raw(), config.raw());
            Self { raw }
        }
    }

    /// Create a new App instance with default settings and config.
    pub fn with_defaults() -> Self {
        let settings = Settings::new();
        let config = Config::new();
        unsafe {
            let raw = ulCreateApp(settings.raw(), config.raw());
            Self { raw }
        }
    }

    /// Get a reference to the raw ULApp.
    pub fn raw(&self) -> ULApp {
        self.raw
    }

    /// Set a callback for app updates.
    ///
    /// This event is fired right before the run loop calls
    /// Renderer::Update and Renderer::Render.
    pub fn set_update_callback<T: 'static + UpdateCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulAppSetUpdateCallback(
                self.raw,
                std::mem::transmute(update_callback_wrapper::<T> as extern "C" fn(*mut c_void)),
                user_data,
            );
        }
    }

    /// Check if the app is running.
    pub fn is_running(&self) -> bool {
        unsafe { ulAppIsRunning(self.raw) }
    }

    /// Get the main monitor.
    pub fn main_monitor(&self) -> Monitor {
        unsafe {
            let monitor = ulAppGetMainMonitor(self.raw);
            Monitor::from_raw(monitor)
        }
    }

    /// Get the underlying Renderer instance.
    pub fn renderer(&self) -> Renderer {
        unsafe {
            let renderer = ulAppGetRenderer(self.raw);
            Renderer::from_raw(renderer, false)
        }
    }

    /// Run the main loop.
    ///
    /// This function will block until the app is quit.
    pub fn run(&self) {
        unsafe {
            ulAppRun(self.raw);
        }
    }

    /// Quit the application.
    pub fn quit(&self) {
        unsafe {
            ulAppQuit(self.raw);
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyApp(self.raw);
            }
        }
    }
}
