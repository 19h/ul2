use std::marker::PhantomData;
use std::os::raw::{c_void};
use std::ptr;
use std::slice;
use crate::ul::ffi::{ULSurface, ULBitmapSurface, ULSurfaceDefinition, ULIntRect,
                ulSurfaceGetWidth, ulSurfaceGetHeight, ulSurfaceGetRowBytes, ulSurfaceGetSize,
                ulSurfaceLockPixels, ulSurfaceUnlockPixels, ulSurfaceResize, ulSurfaceSetDirtyBounds,
                ulSurfaceGetDirtyBounds, ulSurfaceClearDirtyBounds, ulSurfaceGetUserData,
                ulBitmapSurfaceGetBitmap};
use crate::ul::bitmap::Bitmap;
use crate::ul::geometry::IntRect;

/// A locked surface pixels wrapper that automatically unlocks the pixels when dropped.
pub struct LockedPixels<'a> {
    surface: &'a Surface,
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
            ulSurfaceUnlockPixels(self.surface.raw);
        }
    }
}

/// A safe wrapper around Ultralight's ULSurface type.
pub struct Surface {
    raw: ULSurface,
}

/// A safe wrapper for bitmap surfaces.
pub struct BitmapSurface {
    surface: Surface,
}

/// Trait for implementing a custom surface.
pub trait SurfaceDefinition {
    /// Create a new surface with the specified dimensions.
    fn create(width: u32, height: u32) -> *mut c_void;
    
    /// Destroy the surface.
    fn destroy(user_data: *mut c_void);
    
    /// Get the width of the surface in pixels.
    fn get_width(user_data: *mut c_void) -> u32;
    
    /// Get the height of the surface in pixels.
    fn get_height(user_data: *mut c_void) -> u32;
    
    /// Get the number of bytes per row.
    fn get_row_bytes(user_data: *mut c_void) -> u32;
    
    /// Get the size of the pixel buffer in bytes.
    fn get_size(user_data: *mut c_void) -> usize;
    
    /// Lock the pixel buffer for reading/writing.
    fn lock_pixels(user_data: *mut c_void) -> *mut c_void;
    
    /// Unlock the pixel buffer.
    fn unlock_pixels(user_data: *mut c_void);
    
    /// Resize the pixel buffer.
    fn resize(user_data: *mut c_void, width: u32, height: u32);
    
    /// Convert the trait to a raw ULSurfaceDefinition.
    fn to_raw() -> ULSurfaceDefinition where Self: Sized {
        extern "C" fn create_callback<T: SurfaceDefinition>(width: u32, height: u32) -> *mut c_void {
            T::create(width, height)
        }
        
        extern "C" fn destroy_callback<T: SurfaceDefinition>(user_data: *mut c_void) {
            T::destroy(user_data)
        }
        
        extern "C" fn get_width_callback<T: SurfaceDefinition>(user_data: *mut c_void) -> u32 {
            T::get_width(user_data)
        }
        
        extern "C" fn get_height_callback<T: SurfaceDefinition>(user_data: *mut c_void) -> u32 {
            T::get_height(user_data)
        }
        
        extern "C" fn get_row_bytes_callback<T: SurfaceDefinition>(user_data: *mut c_void) -> u32 {
            T::get_row_bytes(user_data)
        }
        
        extern "C" fn get_size_callback<T: SurfaceDefinition>(user_data: *mut c_void) -> usize {
            T::get_size(user_data)
        }
        
        extern "C" fn lock_pixels_callback<T: SurfaceDefinition>(user_data: *mut c_void) -> *mut c_void {
            T::lock_pixels(user_data)
        }
        
        extern "C" fn unlock_pixels_callback<T: SurfaceDefinition>(user_data: *mut c_void) {
            T::unlock_pixels(user_data)
        }
        
        extern "C" fn resize_callback<T: SurfaceDefinition>(user_data: *mut c_void, width: u32, height: u32) {
            T::resize(user_data, width, height)
        }
        
        ULSurfaceDefinition {
            create: create_callback::<Self>,
            destroy: destroy_callback::<Self>,
            get_width: get_width_callback::<Self>,
            get_height: get_height_callback::<Self>,
            get_row_bytes: get_row_bytes_callback::<Self>,
            get_size: get_size_callback::<Self>,
            lock_pixels: lock_pixels_callback::<Self>,
            unlock_pixels: unlock_pixels_callback::<Self>,
            resize: resize_callback::<Self>,
        }
    }
}

impl Surface {
    /// Create a new surface from a raw ULSurface pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULSurface created by the Ultralight API.
    pub unsafe fn from_raw(raw: ULSurface) -> Self {
        Self { raw }
    }
    
    /// Get a reference to the raw ULSurface.
    pub fn raw(&self) -> ULSurface {
        self.raw
    }
    
    /// Get the width of the surface in pixels.
    pub fn width(&self) -> u32 {
        unsafe { ulSurfaceGetWidth(self.raw) }
    }
    
    /// Get the height of the surface in pixels.
    pub fn height(&self) -> u32 {
        unsafe { ulSurfaceGetHeight(self.raw) }
    }
    
    /// Get the number of bytes per row.
    pub fn row_bytes(&self) -> u32 {
        unsafe { ulSurfaceGetRowBytes(self.raw) }
    }
    
    /// Get the size of the surface in bytes.
    pub fn size(&self) -> usize {
        unsafe { ulSurfaceGetSize(self.raw) }
    }
    
    /// Lock the pixel buffer for reading/writing.
    pub fn lock_pixels(&self) -> Result<LockedPixels, ()> {
        unsafe {
            let pixels = ulSurfaceLockPixels(self.raw);
            if pixels.is_null() {
                return Err(());
            }
            
            Ok(LockedPixels {
                surface: self,
                pixels: pixels as *mut u8,
                size: self.size(),
                _phantom: PhantomData,
            })
        }
    }
    
    /// Resize the surface to the specified dimensions.
    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            ulSurfaceResize(self.raw, width, height);
        }
    }
    
    /// Set the dirty bounds of the surface.
    pub fn set_dirty_bounds(&self, bounds: IntRect) {
        unsafe {
            ulSurfaceSetDirtyBounds(self.raw, bounds.into_raw());
        }
    }
    
    /// Get the dirty bounds of the surface.
    pub fn dirty_bounds(&self) -> IntRect {
        unsafe {
            let raw = ulSurfaceGetDirtyBounds(self.raw);
            IntRect::from_raw(raw)
        }
    }
    
    /// Clear the dirty bounds of the surface.
    pub fn clear_dirty_bounds(&self) {
        unsafe {
            ulSurfaceClearDirtyBounds(self.raw);
        }
    }
    
    /// Get the user data pointer for custom surface implementations.
    pub fn user_data(&self) -> *mut c_void {
        unsafe { ulSurfaceGetUserData(self.raw) }
    }
    
    /// Try to cast this surface to a BitmapSurface.
    pub fn as_bitmap_surface(&self) -> Option<BitmapSurface> {
        if self.user_data().is_null() {
            Some(BitmapSurface { surface: Surface { raw: self.raw } })
        } else {
            None
        }
    }
}

impl BitmapSurface {
    /// Create a new bitmap surface from a raw ULBitmapSurface pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULBitmapSurface created by the Ultralight API.
    pub unsafe fn from_raw(raw: ULBitmapSurface) -> Self {
        Self { surface: Surface { raw } }
    }
    
    /// Get a reference to the underlying Surface.
    pub fn surface(&self) -> &Surface {
        &self.surface
    }
    
    /// Get a reference to the raw ULBitmapSurface.
    pub fn raw(&self) -> ULBitmapSurface {
        self.surface.raw
    }
    
    /// Get the underlying bitmap.
    pub fn bitmap(&self) -> Bitmap {
        unsafe {
            let bitmap = ulBitmapSurfaceGetBitmap(self.raw());
            Bitmap::from_raw(bitmap, false)
        }
    }
}