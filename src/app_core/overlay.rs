use crate::app_core::ffi::{
    ULOverlay, ulCreateOverlay, ulCreateOverlayWithView, ulDestroyOverlay, ulOverlayFocus,
    ulOverlayGetHeight, ulOverlayGetView, ulOverlayGetWidth, ulOverlayGetX, ulOverlayGetY,
    ulOverlayHasFocus, ulOverlayHide, ulOverlayIsHidden, ulOverlayMoveTo, ulOverlayResize,
    ulOverlayShow, ulOverlayUnfocus,
};
use crate::app_core::error::Error;
use crate::app_core::window::Window;
use crate::ul::View;

/// An overlay for displaying web content in a portion of a window.
pub struct Overlay {
    raw: ULOverlay,
}

impl Overlay {
    /// Create a new overlay in a window.
    ///
    /// # Arguments
    ///
    /// * `window` - The window to create the overlay in
    /// * `width` - The width in pixels
    /// * `height` - The height in pixels
    /// * `x` - The x-position (offset from the left of the window), in pixels
    /// * `y` - The y-position (offset from the top of the window), in pixels
    ///
    /// # Returns
    ///
    /// A Result containing the Overlay if successful, or an Error if overlay creation failed.
    pub fn new(window: &Window, width: u32, height: u32, x: i32, y: i32) -> Result<Self, Error> {
        unsafe {
            let raw = ulCreateOverlay(window.raw(), width, height, x, y);
            if raw.is_null() {
                return Err(Error::CreationFailed("Failed to create overlay"));
            }
            Ok(Self { raw })
        }
    }

    /// Create a new overlay in a window, wrapping an existing view.
    ///
    /// # Arguments
    ///
    /// * `window` - The window to create the overlay in
    /// * `view` - The view to wrap (will use its width and height)
    /// * `x` - The x-position (offset from the left of the window), in pixels
    /// * `y` - The y-position (offset from the top of the window), in pixels
    ///
    /// # Returns
    ///
    /// A Result containing the Overlay if successful, or an Error if overlay creation failed.
    pub fn with_view(window: &Window, view: &View, x: i32, y: i32) -> Result<Self, Error> {
        unsafe {
            let raw = ulCreateOverlayWithView(window.raw(), view.raw(), x, y);
            if raw.is_null() {
                return Err(Error::CreationFailed("Failed to create overlay with view"));
            }
            Ok(Self { raw })
        }
    }

    /// Create an Overlay from a raw ULOverlay pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULOverlay created by the AppCore API.
    /// This function does not verify if the pointer is valid.
    ///
    /// # Returns
    ///
    /// An Overlay instance.
    pub unsafe fn from_raw(raw: ULOverlay) -> Self {
        Self { raw }
    }

    /// Get a reference to the raw ULOverlay.
    pub fn raw(&self) -> ULOverlay {
        self.raw
    }

    /// Get the underlying view.
    ///
    /// # Returns
    ///
    /// A Result containing the View if successful, or an Error if view retrieval failed.
    pub fn view(&self) -> Result<View, Error> {
        unsafe {
            let view = ulOverlayGetView(self.raw);
            if view.is_null() {
                return Err(Error::NullReference("Failed to get view from overlay"));
            }
            Ok(View::from_raw(view))
        }
    }

    /// Get the width (in pixels).
    pub fn width(&self) -> u32 {
        unsafe { ulOverlayGetWidth(self.raw) }
    }

    /// Get the height (in pixels).
    pub fn height(&self) -> u32 {
        unsafe { ulOverlayGetHeight(self.raw) }
    }

    /// Get the x-position (offset from the left of the window), in pixels.
    pub fn x(&self) -> i32 {
        unsafe { ulOverlayGetX(self.raw) }
    }

    /// Get the y-position (offset from the top of the window), in pixels.
    pub fn y(&self) -> i32 {
        unsafe { ulOverlayGetY(self.raw) }
    }

    /// Move the overlay to a new position (in pixels).
    pub fn move_to(&self, x: i32, y: i32) {
        unsafe {
            ulOverlayMoveTo(self.raw, x, y);
        }
    }

    /// Resize the overlay (and underlying view), dimensions should be specified
    /// in pixels.
    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            ulOverlayResize(self.raw, width, height);
        }
    }

    /// Check if the overlay is hidden (not drawn).
    pub fn is_hidden(&self) -> bool {
        unsafe { ulOverlayIsHidden(self.raw) }
    }

    /// Hide the overlay (will no longer be drawn).
    pub fn hide(&self) {
        unsafe {
            ulOverlayHide(self.raw);
        }
    }

    /// Show the overlay.
    pub fn show(&self) {
        unsafe {
            ulOverlayShow(self.raw);
        }
    }

    /// Check if the overlay has keyboard focus.
    pub fn has_focus(&self) -> bool {
        unsafe { ulOverlayHasFocus(self.raw) }
    }

    /// Grant this overlay exclusive keyboard focus.
    pub fn focus(&self) {
        unsafe {
            ulOverlayFocus(self.raw);
        }
    }

    /// Remove keyboard focus.
    pub fn unfocus(&self) {
        unsafe {
            ulOverlayUnfocus(self.raw);
        }
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyOverlay(self.raw);
            }
        }
    }
}