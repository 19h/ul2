use std::marker::PhantomData;
use std::os::raw::{c_void};
use std::ptr;
use std::slice;
use crate::ul::ffi::{ULBuffer, ULDestroyBufferCallback, ulCreateBuffer, ulCreateBufferFromCopy, 
                ulDestroyBuffer, ulBufferGetData, ulBufferGetSize, ulBufferGetUserData, 
                ulBufferOwnsData};
use crate::ul::error::Error;

/// A safe wrapper around Ultralight's ULBuffer type.
pub struct Buffer {
    raw: ULBuffer,
    owned: bool,
}

extern "C" fn noop_buffer_destroy_callback(_: *mut c_void, _: *mut c_void) {}

impl Buffer {
    /// Create a new buffer from a raw ULBuffer pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULBuffer created by the Ultralight API.
    pub unsafe fn from_raw(raw: ULBuffer, owned: bool) -> Self {
        Self { raw, owned }
    }
    
    /// Create a new buffer that wraps the given data without copying it.
    pub fn new(data: &[u8]) -> Self {
        unsafe {
            let raw = ulCreateBuffer(
                data.as_ptr() as *mut c_void,
                data.len(),
                ptr::null_mut(),
                std::mem::transmute(noop_buffer_destroy_callback as extern "C" fn(*mut c_void, *mut c_void))
            );
            Self { raw, owned: true }
        }
    }
    
    /// Create a new buffer with a copy of the given data.
    pub fn from_copy(data: &[u8]) -> Self {
        unsafe {
            let raw = ulCreateBufferFromCopy(data.as_ptr() as *const c_void, data.len());
            Self { raw, owned: true }
        }
    }
    
    /// Get a reference to the raw ULBuffer.
    pub fn raw(&self) -> ULBuffer {
        self.raw
    }
    
    /// Get a slice of the buffer data.
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let data = ulBufferGetData(self.raw);
            let size = ulBufferGetSize(self.raw);
            slice::from_raw_parts(data as *const u8, size)
        }
    }
    
    /// Get the size of the buffer in bytes.
    pub fn size(&self) -> usize {
        unsafe { ulBufferGetSize(self.raw) }
    }
    
    /// Check if the buffer owns its data.
    pub fn owns_data(&self) -> bool {
        unsafe { ulBufferOwnsData(self.raw) }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if !self.raw.is_null() && self.owned {
            unsafe {
                ulDestroyBuffer(self.raw);
            }
        }
    }
}