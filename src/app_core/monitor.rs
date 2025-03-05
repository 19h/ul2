use crate::app_core::ffi::{ULMonitor, ulMonitorGetHeight, ulMonitorGetScale, ulMonitorGetWidth};
use crate::app_core::error::Error;

/// A representation of a display monitor.
pub struct Monitor {
    raw: ULMonitor,
}

impl Monitor {
    /// Create a Monitor from a raw ULMonitor pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULMonitor created by the AppCore API.
    /// This function does not verify if the pointer is valid.
    ///
    /// # Returns
    ///
    /// A Monitor instance.
    pub unsafe fn from_raw(raw: ULMonitor) -> Self {
        Self { raw }
    }

    /// Get a reference to the raw ULMonitor.
    pub fn raw(&self) -> ULMonitor {
        self.raw
    }

    /// Get the monitor's DPI scale (1.0 = 100%).
    pub fn scale(&self) -> f64 {
        unsafe { ulMonitorGetScale(self.raw) }
    }

    /// Get the width of the monitor (in pixels).
    pub fn width(&self) -> u32 {
        unsafe { ulMonitorGetWidth(self.raw) }
    }

    /// Get the height of the monitor (in pixels).
    pub fn height(&self) -> u32 {
        unsafe { ulMonitorGetHeight(self.raw) }
    }
}