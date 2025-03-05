use crate::app_core::ffi::{
    ULApp, ulAppGetMainMonitor, ulAppGetRenderer, ulAppIsRunning, ulAppQuit, ulAppRun,
    ulAppSetUpdateCallback, ulCreateApp, ulDestroyApp,
};
use crate::app_core::monitor::Monitor;
use crate::app_core::settings::Settings;
use crate::app_core::error::Error;
use crate::ul::{Config, Renderer};
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};

/// Callback for app updates.
pub trait UpdateCallback: Send {
    fn on_update(&self);
}

// Callback wrapper for the C API
extern "C" fn update_callback_wrapper<T: UpdateCallback>(user_data: *mut c_void) {
    unsafe {
        if !user_data.is_null() {
            let callback = &*(user_data as *const T);
            callback.on_update();
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

/// The main application class for AppCore, responsible for managing the renderer,
/// run loop, windows, and platform-specific operations.
pub struct App {
    raw: ULApp,
    // Store the callback to ensure it lives as long as the app
    update_callback: Arc<Mutex<CallbackStorage<dyn UpdateCallback>>>,
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
    ///
    /// # Returns
    ///
    /// A Result containing the App if successful, or an Error if app creation failed.
    pub fn new(settings: &Settings, config: &Config) -> Result<Self, Error> {
        unsafe {
            let raw = ulCreateApp(settings.raw(), config.raw());
            if raw.is_null() {
                return Err(Error::CreationFailed("Failed to create app instance"));
            }
            
            Ok(Self { 
                raw,
                update_callback: Arc::new(Mutex::new(CallbackStorage::new())),
            })
        }
    }

    /// Create a new App instance with default settings and config.
    ///
    /// # Returns
    ///
    /// A Result containing the App if successful, or an Error if app creation failed.
    pub fn with_defaults() -> Result<Self, Error> {
        let settings = Settings::new()?;
        let config = Config::new();
        Self::new(&settings, &config)
    }

    /// Get a reference to the raw ULApp.
    pub fn raw(&self) -> ULApp {
        self.raw
    }

    /// Set a callback for app updates.
    ///
    /// This event is fired right before the run loop calls
    /// Renderer::Update and Renderer::Render.
    ///
    /// # Arguments
    ///
    /// * `callback` - The callback to invoke on update
    pub fn set_update_callback<T: 'static + UpdateCallback>(&self, callback: T) -> Result<(), Error> {
        unsafe {
            let mut update_callback = match self.update_callback.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(Error::CallbackRegistrationFailed("Failed to acquire callback lock")),
            };
            
            let user_data = update_callback.set(callback);
            
            ulAppSetUpdateCallback(
                self.raw,
                Some(update_callback_wrapper::<T>),
                user_data,
            );
            
            Ok(())
        }
    }

    /// Clear the update callback.
    pub fn clear_update_callback(&self) -> Result<(), Error> {
        unsafe {
            let mut update_callback = match self.update_callback.lock() {
                Ok(guard) => guard,
                Err(_) => return Err(Error::InvalidOperation("Failed to acquire callback lock")),
            };
            
            update_callback.clear();
            ulAppSetUpdateCallback(self.raw, None, std::ptr::null_mut());
            
            Ok(())
        }
    }

    /// Check if the app is running.
    pub fn is_running(&self) -> bool {
        unsafe { ulAppIsRunning(self.raw) }
    }

    /// Get the main monitor.
    ///
    /// # Returns
    ///
    /// A Result containing the Monitor if successful, or an Error if monitor retrieval failed.
    pub fn main_monitor(&self) -> Result<Monitor, Error> {
        unsafe {
            let monitor = ulAppGetMainMonitor(self.raw);
            if monitor.is_null() {
                return Err(Error::NullReference("Failed to get main monitor"));
            }
            Ok(Monitor::from_raw(monitor))
        }
    }

    /// Get the underlying Renderer instance.
    ///
    /// # Returns
    ///
    /// A Result containing the Renderer if successful, or an Error if renderer retrieval failed.
    pub fn renderer(&self) -> Result<Renderer, Error> {
        unsafe {
            let renderer = ulAppGetRenderer(self.raw);
            if renderer.is_null() {
                return Err(Error::NullReference("Failed to get renderer"));
            }
            Ok(Renderer::from_raw(renderer, false))
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