use crate::ul::config::Config;
use crate::ul::events::{GamepadAxisEvent, GamepadButtonEvent, GamepadEvent};
use crate::ul::ffi::{
    ULRenderer, ulCreateRenderer, ulDestroyRenderer, ulFireGamepadAxisEvent,
    ulFireGamepadButtonEvent, ulFireGamepadEvent, ulLogMemoryUsage, ulPurgeMemory,
    ulRefreshDisplay, ulRender, ulSetGamepadDetails, ulStartRemoteInspectorServer, ulUpdate,
};
use crate::ul::session::Session;
use crate::ul::string::String;
use std::ffi::CString;

/// A safe wrapper around Ultralight's ULRenderer type.
pub struct Renderer {
    raw: ULRenderer,
    owned: bool,
}

impl Renderer {
    /// Create a new renderer with the specified configuration.
    pub fn new(config: Config) -> Self {
        unsafe {
            let raw = ulCreateRenderer(config.raw());
            Self { raw, owned: true }
        }
    }

    /// Create a renderer from a raw ULRenderer pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULRenderer created by the Ultralight API.
    pub unsafe fn from_raw(raw: ULRenderer, owned: bool) -> Self {
        Self { raw, owned }
    }

    /// Get a reference to the raw ULRenderer.
    pub fn raw(&self) -> ULRenderer {
        self.raw
    }

    /// Update timers and dispatch internal callbacks.
    pub fn update(&self) {
        unsafe {
            ulUpdate(self.raw);
        }
    }

    /// Notify the renderer that a display has refreshed.
    pub fn refresh_display(&self, display_id: u32) {
        unsafe {
            ulRefreshDisplay(self.raw, display_id);
        }
    }

    /// Render all active Views.
    pub fn render(&self) {
        unsafe {
            ulRender(self.raw);
        }
    }

    /// Attempt to release as much memory as possible.
    pub fn purge_memory(&self) {
        unsafe {
            ulPurgeMemory(self.raw);
        }
    }

    /// Print detailed memory usage statistics to the log.
    pub fn log_memory_usage(&self) {
        unsafe {
            ulLogMemoryUsage(self.raw);
        }
    }

    /// Start the remote inspector server.
    pub fn start_remote_inspector_server(&self, address: &str, port: u16) -> bool {
        let c_address = CString::new(address).unwrap();
        unsafe { ulStartRemoteInspectorServer(self.raw, c_address.as_ptr(), port) }
    }

    /// Describe the details of a gamepad.
    pub fn set_gamepad_details(&self, index: u32, id: &str, axis_count: u32, button_count: u32) {
        let id_str = String::from_str(id);
        unsafe {
            ulSetGamepadDetails(self.raw, index, id_str.raw(), axis_count, button_count);
        }
    }

    /// Fire a gamepad event.
    pub fn fire_gamepad_event(&self, event: &GamepadEvent) {
        unsafe {
            ulFireGamepadEvent(self.raw, event.raw());
        }
    }

    /// Fire a gamepad axis event.
    pub fn fire_gamepad_axis_event(&self, event: &GamepadAxisEvent) {
        unsafe {
            ulFireGamepadAxisEvent(self.raw, event.raw());
        }
    }

    /// Fire a gamepad button event.
    pub fn fire_gamepad_button_event(&self, event: &GamepadButtonEvent) {
        unsafe {
            ulFireGamepadButtonEvent(self.raw, event.raw());
        }
    }

    /// Create a new session.
    pub fn create_session(&self, is_persistent: bool, name: &str) -> Session {
        Session::new(self, is_persistent, name)
    }

    /// Get the default session.
    pub fn default_session(&self) -> Session {
        Session::default(self)
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        if !self.raw.is_null() && self.owned {
            unsafe {
                ulDestroyRenderer(self.raw);
            }
        }
    }
}
