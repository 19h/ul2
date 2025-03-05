use crate::app_core::ffi::{
    ULApp, ulAppGetMainMonitor, ulAppGetRenderer, ulAppIsRunning, ulAppQuit, ulAppRun,
    ulAppSetUpdateCallback, ulCreateApp, ulDestroyApp,
};
use crate::app_core::monitor::Monitor;
use crate::app_core::settings::Settings;
use crate::app_core::error::Error;
use crate::ul::{Config, Renderer};
use std::os::raw::c_void;
use std::cell::RefCell;

/// Callback for app updates.
pub trait UpdateCallback: Send {
    fn on_update(&self);
}

// Thread-local storage for the active callback
thread_local! {
    static ACTIVE_UPDATE_CALLBACK: RefCell<Option<Box<dyn FnMut()>>> = RefCell::new(None);
}

extern "C" fn update_callback_trampoline(_user_data: *mut c_void) {
    ACTIVE_UPDATE_CALLBACK.with(|cell| {
        if let Some(callback) = cell.borrow_mut().as_mut() {
            callback();
        }
    });
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
            
            Ok(Self { raw })
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
    ///
    /// # Returns
    ///
    /// A Result containing Ok(()) if successful, or an Error if callback setting failed.
    pub fn set_update_callback<F>(&self, callback: F) -> Result<(), Error>
    where
        F: FnMut() + 'static,
    {
        // Store the callback in thread-local storage
        ACTIVE_UPDATE_CALLBACK.with(|cell| {
            *cell.borrow_mut() = Some(Box::new(callback));
        });
        
        unsafe {
            ulAppSetUpdateCallback(
                self.raw,
                update_callback_trampoline,
                std::ptr::null_mut(),
            );
        }
        
        Ok(())
    }

    /// Clear the update callback.
    ///
    /// # Returns
    ///
    /// A Result containing Ok(()) if successful, or an Error if callback clearing failed.
    pub fn clear_update_callback(&self) -> Result<(), Error> {
        // Clear the callback from thread-local storage
        ACTIVE_UPDATE_CALLBACK.with(|cell| {
            *cell.borrow_mut() = None;
        });
        
        unsafe {
            // Define a no-op callback
            extern "C" fn no_op(_: *mut c_void) {}
            
            ulAppSetUpdateCallback(
                self.raw,
                no_op,
                std::ptr::null_mut(),
            );
        }
        
        Ok(())
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
        // Clear the update callback to avoid dangling references
        let _ = self.clear_update_callback();
        
        if !self.raw.is_null() {
            unsafe {
                ulDestroyApp(self.raw);
            }
        }
    }
}