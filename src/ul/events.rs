use std::os::raw::{c_int, c_uint, c_double};
use crate::ul::ffi::{ULKeyEvent, ULKeyEventType, ULMouseEvent, ULMouseEventType, ULMouseButton,
                ULScrollEvent, ULScrollEventType, ULGamepadEvent, ULGamepadEventType,
                ULGamepadAxisEvent, ULGamepadButtonEvent, ulCreateKeyEvent, ulDestroyKeyEvent,
                ulCreateMouseEvent, ulDestroyMouseEvent, ulCreateScrollEvent, ulDestroyScrollEvent,
                ulCreateGamepadEvent, ulDestroyGamepadEvent, ulCreateGamepadAxisEvent, 
                ulDestroyGamepadAxisEvent, ulCreateGamepadButtonEvent, ulDestroyGamepadButtonEvent};
use crate::ul::string::String;

pub use crate::ul::ffi::{
    ULKeyEventType as KeyEventType,
    ULMouseEventType as MouseEventType,
    ULMouseButton as MouseButton,
    ULScrollEventType as ScrollEventType,
    ULGamepadEventType as GamepadEventType,
};

/// A safe wrapper around Ultralight's ULKeyEvent type.
pub struct KeyEvent {
    raw: ULKeyEvent,
}

/// A safe wrapper around Ultralight's ULMouseEvent type.
pub struct MouseEvent {
    raw: ULMouseEvent,
}

/// A safe wrapper around Ultralight's ULScrollEvent type.
pub struct ScrollEvent {
    raw: ULScrollEvent,
}

/// A safe wrapper around Ultralight's ULGamepadEvent type.
pub struct GamepadEvent {
    raw: ULGamepadEvent,
}

/// A safe wrapper around Ultralight's ULGamepadAxisEvent type.
pub struct GamepadAxisEvent {
    raw: ULGamepadAxisEvent,
}

/// A safe wrapper around Ultralight's ULGamepadButtonEvent type.
pub struct GamepadButtonEvent {
    raw: ULGamepadButtonEvent,
}

impl KeyEvent {
    /// Create a new key event.
    pub fn new(
        event_type: KeyEventType,
        modifiers: u32,
        virtual_key_code: i32,
        native_key_code: i32,
        text: &str,
        unmodified_text: &str,
        is_keypad: bool,
        is_auto_repeat: bool,
        is_system_key: bool,
    ) -> Self {
        let text_str = String::from_str(text);
        let unmodified_text_str = String::from_str(unmodified_text);
        
        unsafe {
            let raw = ulCreateKeyEvent(
                event_type,
                modifiers,
                virtual_key_code,
                native_key_code,
                text_str.raw(),
                unmodified_text_str.raw(),
                is_keypad,
                is_auto_repeat,
                is_system_key,
            );
            
            Self { raw }
        }
    }
    
    /// Get a reference to the raw ULKeyEvent.
    pub fn raw(&self) -> ULKeyEvent {
        self.raw
    }
}

impl Drop for KeyEvent {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyKeyEvent(self.raw);
            }
        }
    }
}

impl MouseEvent {
    /// Create a new mouse event.
    pub fn new(event_type: MouseEventType, x: i32, y: i32, button: MouseButton) -> Self {
        unsafe {
            let raw = ulCreateMouseEvent(event_type, x, y, button);
            Self { raw }
        }
    }
    
    /// Get a reference to the raw ULMouseEvent.
    pub fn raw(&self) -> ULMouseEvent {
        self.raw
    }
}

impl Drop for MouseEvent {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyMouseEvent(self.raw);
            }
        }
    }
}

impl ScrollEvent {
    /// Create a new scroll event.
    pub fn new(event_type: ScrollEventType, delta_x: i32, delta_y: i32) -> Self {
        unsafe {
            let raw = ulCreateScrollEvent(event_type, delta_x, delta_y);
            Self { raw }
        }
    }
    
    /// Get a reference to the raw ULScrollEvent.
    pub fn raw(&self) -> ULScrollEvent {
        self.raw
    }
}

impl Drop for ScrollEvent {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyScrollEvent(self.raw);
            }
        }
    }
}

impl GamepadEvent {
    /// Create a new gamepad event.
    pub fn new(index: u32, event_type: GamepadEventType) -> Self {
        unsafe {
            let raw = ulCreateGamepadEvent(index, event_type);
            Self { raw }
        }
    }
    
    /// Get a reference to the raw ULGamepadEvent.
    pub fn raw(&self) -> ULGamepadEvent {
        self.raw
    }
}

impl Drop for GamepadEvent {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyGamepadEvent(self.raw);
            }
        }
    }
}

impl GamepadAxisEvent {
    /// Create a new gamepad axis event.
    pub fn new(index: u32, axis_index: u32, value: f64) -> Self {
        unsafe {
            let raw = ulCreateGamepadAxisEvent(index, axis_index, value);
            Self { raw }
        }
    }
    
    /// Get a reference to the raw ULGamepadAxisEvent.
    pub fn raw(&self) -> ULGamepadAxisEvent {
        self.raw
    }
}

impl Drop for GamepadAxisEvent {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyGamepadAxisEvent(self.raw);
            }
        }
    }
}

impl GamepadButtonEvent {
    /// Create a new gamepad button event.
    pub fn new(index: u32, button_index: u32, value: f64) -> Self {
        unsafe {
            let raw = ulCreateGamepadButtonEvent(index, button_index, value);
            Self { raw }
        }
    }
    
    /// Get a reference to the raw ULGamepadButtonEvent.
    pub fn raw(&self) -> ULGamepadButtonEvent {
        self.raw
    }
}

impl Drop for GamepadButtonEvent {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyGamepadButtonEvent(self.raw);
            }
        }
    }
}