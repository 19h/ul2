use crate::ul::ffi::{ULRect, ULIntRect, ulRectIsEmpty, ulRectMakeEmpty, ulIntRectIsEmpty, ulIntRectMakeEmpty};

/// A rectangle with floating-point coordinates.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

/// A rectangle with integer coordinates.
#[derive(Debug, Clone, Copy)]
pub struct IntRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    /// Create a new rectangle with the specified coordinates.
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self { left, top, right, bottom }
    }
    
    /// Create a new empty rectangle.
    pub fn empty() -> Self {
        unsafe {
            Self::from_raw(ulRectMakeEmpty())
        }
    }
    
    /// Create a rectangle from a raw ULRect.
    pub fn from_raw(raw: ULRect) -> Self {
        Self {
            left: raw.left,
            top: raw.top,
            right: raw.right,
            bottom: raw.bottom,
        }
    }
    
    /// Convert the rectangle to a raw ULRect.
    pub fn into_raw(self) -> ULRect {
        ULRect {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
    
    /// Check if the rectangle is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { ulRectIsEmpty(self.into_raw()) }
    }
    
    /// Get the width of the rectangle.
    pub fn width(&self) -> f32 {
        self.right - self.left
    }
    
    /// Get the height of the rectangle.
    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }
}

impl IntRect {
    /// Create a new rectangle with the specified coordinates.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self { left, top, right, bottom }
    }
    
    /// Create a new empty rectangle.
    pub fn empty() -> Self {
        unsafe {
            Self::from_raw(ulIntRectMakeEmpty())
        }
    }
    
    /// Create a rectangle from a raw ULIntRect.
    pub fn from_raw(raw: ULIntRect) -> Self {
        Self {
            left: raw.left,
            top: raw.top,
            right: raw.right,
            bottom: raw.bottom,
        }
    }
    
    /// Convert the rectangle to a raw ULIntRect.
    pub fn into_raw(self) -> ULIntRect {
        ULIntRect {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
    
    /// Check if the rectangle is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { ulIntRectIsEmpty(self.into_raw()) }
    }
    
    /// Get the width of the rectangle.
    pub fn width(&self) -> i32 {
        self.right - self.left
    }
    
    /// Get the height of the rectangle.
    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
}