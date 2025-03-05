use std::marker::PhantomData;
use std::ptr;
use std::slice;
use crate::ul::ffi::{ULBitmap, ULBitmapFormat, ulCreateEmptyBitmap, ulCreateBitmap, 
                ulCreateBitmapFromPixels, ulCreateBitmapFromCopy, ulDestroyBitmap, 
                ulBitmapGetWidth, ulBitmapGetHeight, ulBitmapGetFormat, ulBitmapGetBpp, 
                ulBitmapGetRowBytes, ulBitmapGetSize, ulBitmapOwnsPixels, ulBitmapLockPixels, 
                ulBitmapUnlockPixels, ulBitmapRawPixels, ulBitmapIsEmpty, ulBitmapErase, 
                ulBitmapWritePNG, ulBitmapSwapRedBlueChannels};
use crate::ul::error::Error;

pub use crate::ul::ffi::ULBitmapFormat as BitmapFormat;

/// A safe wrapper around Ultralight's ULBitmap type.
pub struct Bitmap {
    raw: ULBitmap,
    owned: bool,
}

/// A locked bitmap pixels wrapper that automatically unlocks the pixels when dropped.
pub struct LockedPixels<'a> {
    bitmap: &'a Bitmap,
    pixels: *mut u8,
    size: usize,
    _phantom: PhantomData<&'a mut [u8]>,
}

impl<'a> LockedPixels<'a> {
    /// Get a slice reference to the locked pixels.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.pixels as *const u8, self.size) }
    }
    
    /// Get a mutable slice reference to the locked pixels.
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.pixels as *mut u8, self.size) }
    }
}

impl<'a> Drop for LockedPixels<'a> {
    fn drop(&mut self) {
        unsafe {
            ulBitmapUnlockPixels(self.bitmap.raw);
        }
    }
}

impl Bitmap {
    /// Create a new bitmap from a raw ULBitmap pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULBitmap created by the Ultralight API.
    pub unsafe fn from_raw(raw: ULBitmap, owned: bool) -> Self {
        Self { raw, owned }
    }
    
    /// Create a new empty bitmap.
    pub fn empty() -> Self {
        unsafe {
            let raw = ulCreateEmptyBitmap();
            Self { raw, owned: true }
        }
    }
    
    /// Create a new bitmap with the specified dimensions and format.
    pub fn new(width: u32, height: u32, format: BitmapFormat) -> Self {
        unsafe {
            let raw = ulCreateBitmap(width, height, format);
            Self { raw, owned: true }
        }
    }
    
    /// Create a new bitmap from existing pixel data.
    pub fn from_pixels(width: u32, height: u32, format: BitmapFormat, row_bytes: u32, 
                        pixels: &[u8], should_copy: bool) -> Self {
        unsafe {
            let raw = ulCreateBitmapFromPixels(
                width, 
                height, 
                format, 
                row_bytes, 
                pixels.as_ptr() as *const _, 
                pixels.len(),
                should_copy
            );
            Self { raw, owned: true }
        }
    }
    
    /// Create a copy of another bitmap.
    pub fn from_copy(other: &Self) -> Self {
        unsafe {
            let raw = ulCreateBitmapFromCopy(other.raw);
            Self { raw, owned: true }
        }
    }
    
    /// Get a reference to the raw ULBitmap.
    pub fn raw(&self) -> ULBitmap {
        self.raw
    }
    
    /// Get the width of the bitmap in pixels.
    pub fn width(&self) -> u32 {
        unsafe { ulBitmapGetWidth(self.raw) }
    }
    
    /// Get the height of the bitmap in pixels.
    pub fn height(&self) -> u32 {
        unsafe { ulBitmapGetHeight(self.raw) }
    }
    
    /// Get the pixel format of the bitmap.
    pub fn format(&self) -> BitmapFormat {
        unsafe { ulBitmapGetFormat(self.raw) }
    }
    
    /// Get the bytes per pixel.
    pub fn bpp(&self) -> u32 {
        unsafe { ulBitmapGetBpp(self.raw) }
    }
    
    /// Get the number of bytes per row.
    pub fn row_bytes(&self) -> u32 {
        unsafe { ulBitmapGetRowBytes(self.raw) }
    }
    
    /// Get the size of the bitmap in bytes.
    pub fn size(&self) -> usize {
        unsafe { ulBitmapGetSize(self.raw) }
    }
    
    /// Check if the bitmap owns its pixel buffer.
    pub fn owns_pixels(&self) -> bool {
        unsafe { ulBitmapOwnsPixels(self.raw) }
    }
    
    /// Lock the pixel buffer for reading/writing.
    pub fn lock_pixels(&self) -> Result<LockedPixels, Error> {
        unsafe {
            let pixels = ulBitmapLockPixels(self.raw);
            if pixels.is_null() {
                return Err(Error::NullReference("Failed to lock bitmap pixels"));
            }
            
            Ok(LockedPixels {
                bitmap: self,
                pixels: pixels as *mut u8,
                size: self.size(),
                _phantom: PhantomData,
            })
        }
    }
    
    /// Get the raw pixel buffer without locking.
    ///
    /// # Safety
    ///
    /// The bitmap must already be locked before calling this function.
    pub unsafe fn raw_pixels(&self) -> *mut u8 {
        ulBitmapRawPixels(self.raw) as *mut u8
    }
    
    /// Check if the bitmap is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { ulBitmapIsEmpty(self.raw) }
    }
    
    /// Reset the bitmap pixels to 0.
    pub fn erase(&self) {
        unsafe { ulBitmapErase(self.raw) }
    }
    
    /// Write the bitmap to a PNG file.
    pub fn write_png(&self, path: &str) -> bool {
        use std::ffi::CString;
        let c_path = CString::new(path).unwrap();
        unsafe { ulBitmapWritePNG(self.raw, c_path.as_ptr()) }
    }
    
    /// Swap the red and blue channels in the bitmap.
    pub fn swap_red_blue_channels(&self) {
        unsafe { ulBitmapSwapRedBlueChannels(self.raw) }
    }
}

impl Clone for Bitmap {
    fn clone(&self) -> Self {
        Self::from_copy(self)
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        if !self.raw.is_null() && self.owned {
            unsafe {
                ulDestroyBitmap(self.raw);
            }
        }
    }
}