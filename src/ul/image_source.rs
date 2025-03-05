use crate::ul::bitmap::Bitmap;
use crate::ul::ffi::{
    ULImageSource, ulCreateImageSourceFromBitmap, ulCreateImageSourceFromTexture,
    ulDestroyImageSource, ulImageSourceInvalidate, ulImageSourceProviderAddImageSource,
    ulImageSourceProviderRemoveImageSource,
};
use crate::ul::geometry::Rect;
use crate::ul::string::String;

/// A safe wrapper around Ultralight's ULImageSource type.
pub struct ImageSource {
    raw: ULImageSource,
}

impl ImageSource {
    /// Create a new image source from a GPU texture.
    pub fn from_texture(
        width: u32,
        height: u32,
        texture_id: u32,
        texture_uv: Rect,
        bitmap: Option<&Bitmap>,
    ) -> Self {
        unsafe {
            let bitmap_ptr = match bitmap {
                Some(b) => b.raw(),
                None => std::ptr::null_mut(),
            };

            let raw = ulCreateImageSourceFromTexture(
                width,
                height,
                texture_id,
                texture_uv.into_raw(),
                bitmap_ptr,
            );

            Self { raw }
        }
    }

    /// Create a new image source from a bitmap.
    pub fn from_bitmap(bitmap: &Bitmap) -> Self {
        unsafe {
            let raw = ulCreateImageSourceFromBitmap(bitmap.raw());
            Self { raw }
        }
    }

    /// Get a reference to the raw ULImageSource.
    pub fn raw(&self) -> ULImageSource {
        self.raw
    }

    /// Invalidate the image source, notifying the library that the image has changed.
    pub fn invalidate(&self) {
        unsafe {
            ulImageSourceInvalidate(self.raw);
        }
    }

    /// Add an image source to the provider.
    pub fn add_to_provider(id: &str, image_source: &ImageSource) {
        let id_str = String::from_str(id);
        unsafe {
            ulImageSourceProviderAddImageSource(id_str.raw(), image_source.raw());
        }
    }

    /// Remove an image source from the provider.
    pub fn remove_from_provider(id: &str) {
        let id_str = String::from_str(id);
        unsafe {
            ulImageSourceProviderRemoveImageSource(id_str.raw());
        }
    }
}

impl Drop for ImageSource {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyImageSource(self.raw);
            }
        }
    }
}
