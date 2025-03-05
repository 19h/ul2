use crate::ul::String;
use crate::ul::ffi::{
    ULConfig, ULFaceWinding, ULFontHinting, ulConfigSetAnimationTimerDelay,
    ulConfigSetBitmapAlignment, ulConfigSetCachePath, ulConfigSetFaceWinding, ulConfigSetFontGamma,
    ulConfigSetFontHinting, ulConfigSetForceRepaint, ulConfigSetMaxUpdateTime,
    ulConfigSetMemoryCacheSize, ulConfigSetMinLargeHeapSize, ulConfigSetMinSmallHeapSize,
    ulConfigSetNumRendererThreads, ulConfigSetOverrideRAMSize, ulConfigSetPageCacheSize,
    ulConfigSetRecycleDelay, ulConfigSetResourcePathPrefix, ulConfigSetScrollTimerDelay,
    ulConfigSetUserStylesheet, ulCreateConfig, ulDestroyConfig,
};

/// A safe wrapper around Ultralight's ULConfig type.
pub struct Config {
    raw: ULConfig,
}

impl Config {
    /// Create a new config with default values.
    pub fn new() -> Self {
        unsafe {
            let raw = ulCreateConfig();
            Self { raw }
        }
    }

    /// Get a reference to the raw ULConfig.
    pub fn raw(&self) -> ULConfig {
        self.raw
    }

    /// Set the cache path for persistent Session data.
    pub fn set_cache_path(&mut self, path: &str) -> &mut Self {
        let path_str = String::from_str(path);
        unsafe {
            ulConfigSetCachePath(self.raw, path_str.raw());
        }
        self
    }

    /// Set the relative path to the resources folder.
    pub fn set_resource_path_prefix(&mut self, prefix: &str) -> &mut Self {
        let prefix_str = String::from_str(prefix);
        unsafe {
            ulConfigSetResourcePathPrefix(self.raw, prefix_str.raw());
        }
        self
    }

    /// Set the winding order for front-facing triangles.
    pub fn set_face_winding(&mut self, winding: ULFaceWinding) -> &mut Self {
        unsafe {
            ulConfigSetFaceWinding(self.raw, winding);
        }
        self
    }

    /// Set the font hinting algorithm.
    pub fn set_font_hinting(&mut self, hinting: ULFontHinting) -> &mut Self {
        unsafe {
            ulConfigSetFontHinting(self.raw, hinting);
        }
        self
    }

    /// Set the gamma to use when composing font glyphs.
    pub fn set_font_gamma(&mut self, gamma: f64) -> &mut Self {
        unsafe {
            ulConfigSetFontGamma(self.raw, gamma);
        }
        self
    }

    /// Set the global user-defined CSS string.
    pub fn set_user_stylesheet(&mut self, css: &str) -> &mut Self {
        let css_str = String::from_str(css);
        unsafe {
            ulConfigSetUserStylesheet(self.raw, css_str.raw());
        }
        self
    }

    /// Set whether to continuously repaint Views.
    pub fn set_force_repaint(&mut self, enabled: bool) -> &mut Self {
        unsafe {
            ulConfigSetForceRepaint(self.raw, enabled);
        }
        self
    }

    /// Set the delay between ticks of a CSS animation.
    pub fn set_animation_timer_delay(&mut self, delay: f64) -> &mut Self {
        unsafe {
            ulConfigSetAnimationTimerDelay(self.raw, delay);
        }
        self
    }

    /// Set the delay between ticks of a smooth scroll animation.
    pub fn set_scroll_timer_delay(&mut self, delay: f64) -> &mut Self {
        unsafe {
            ulConfigSetScrollTimerDelay(self.raw, delay);
        }
        self
    }

    /// Set the delay between calls to the recycler.
    pub fn set_recycle_delay(&mut self, delay: f64) -> &mut Self {
        unsafe {
            ulConfigSetRecycleDelay(self.raw, delay);
        }
        self
    }

    /// Set the size of WebCore's memory cache in bytes.
    pub fn set_memory_cache_size(&mut self, size: u32) -> &mut Self {
        unsafe {
            ulConfigSetMemoryCacheSize(self.raw, size);
        }
        self
    }

    /// Set the number of pages to keep in the cache.
    pub fn set_page_cache_size(&mut self, size: u32) -> &mut Self {
        unsafe {
            ulConfigSetPageCacheSize(self.raw, size);
        }
        self
    }

    /// Set the system's physical RAM size in bytes.
    pub fn set_override_ram_size(&mut self, size: u32) -> &mut Self {
        unsafe {
            ulConfigSetOverrideRAMSize(self.raw, size);
        }
        self
    }

    /// Set the minimum size of large VM heaps in JavaScriptCore.
    pub fn set_min_large_heap_size(&mut self, size: u32) -> &mut Self {
        unsafe {
            ulConfigSetMinLargeHeapSize(self.raw, size);
        }
        self
    }

    /// Set the minimum size of small VM heaps in JavaScriptCore.
    pub fn set_min_small_heap_size(&mut self, size: u32) -> &mut Self {
        unsafe {
            ulConfigSetMinSmallHeapSize(self.raw, size);
        }
        self
    }

    /// Set the number of threads to use in the Renderer.
    pub fn set_num_renderer_threads(&mut self, num_threads: u32) -> &mut Self {
        unsafe {
            ulConfigSetNumRendererThreads(self.raw, num_threads);
        }
        self
    }

    /// Set the max amount of time to allow repeating timers to run.
    pub fn set_max_update_time(&mut self, max_time: f64) -> &mut Self {
        unsafe {
            ulConfigSetMaxUpdateTime(self.raw, max_time);
        }
        self
    }

    /// Set the alignment in bytes of the BitmapSurface.
    pub fn set_bitmap_alignment(&mut self, alignment: u32) -> &mut Self {
        unsafe {
            ulConfigSetBitmapAlignment(self.raw, alignment);
        }
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyConfig(self.raw);
            }
        }
    }
}
