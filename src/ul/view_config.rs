use crate::ul::ffi::{ULViewConfig, ulCreateViewConfig, ulDestroyViewConfig, ulViewConfigSetDisplayId, 
                ulViewConfigSetIsAccelerated, ulViewConfigSetIsTransparent, 
                ulViewConfigSetInitialDeviceScale, ulViewConfigSetInitialFocus, 
                ulViewConfigSetEnableImages, ulViewConfigSetEnableJavaScript, 
                ulViewConfigSetFontFamilyStandard, ulViewConfigSetFontFamilyFixed, 
                ulViewConfigSetFontFamilySerif, ulViewConfigSetFontFamilySansSerif, 
                ulViewConfigSetUserAgent};
use crate::ul::string::String;

/// A safe wrapper around Ultralight's ULViewConfig type.
pub struct ViewConfig {
    raw: ULViewConfig,
}

impl ViewConfig {
    /// Create a new view config with default values.
    pub fn new() -> Self {
        unsafe {
            let raw = ulCreateViewConfig();
            Self { raw }
        }
    }
    
    /// Get a reference to the raw ULViewConfig.
    pub fn raw(&self) -> ULViewConfig {
        self.raw
    }
    
    /// Set the display ID that the View will be shown on.
    pub fn set_display_id(&mut self, display_id: u32) -> &mut Self {
        unsafe {
            ulViewConfigSetDisplayId(self.raw, display_id);
        }
        self
    }
    
    /// Set whether to use GPU rendering.
    pub fn set_is_accelerated(&mut self, is_accelerated: bool) -> &mut Self {
        unsafe {
            ulViewConfigSetIsAccelerated(self.raw, is_accelerated);
        }
        self
    }
    
    /// Set whether the View should be transparent.
    pub fn set_is_transparent(&mut self, is_transparent: bool) -> &mut Self {
        unsafe {
            ulViewConfigSetIsTransparent(self.raw, is_transparent);
        }
        self
    }
    
    /// Set the initial device scale.
    pub fn set_initial_device_scale(&mut self, scale: f64) -> &mut Self {
        unsafe {
            ulViewConfigSetInitialDeviceScale(self.raw, scale);
        }
        self
    }
    
    /// Set whether the View should initially have input focus.
    pub fn set_initial_focus(&mut self, has_focus: bool) -> &mut Self {
        unsafe {
            ulViewConfigSetInitialFocus(self.raw, has_focus);
        }
        self
    }
    
    /// Set whether images should be enabled.
    pub fn set_enable_images(&mut self, enabled: bool) -> &mut Self {
        unsafe {
            ulViewConfigSetEnableImages(self.raw, enabled);
        }
        self
    }
    
    /// Set whether JavaScript should be enabled.
    pub fn set_enable_javascript(&mut self, enabled: bool) -> &mut Self {
        unsafe {
            ulViewConfigSetEnableJavaScript(self.raw, enabled);
        }
        self
    }
    
    /// Set the default font family for standard fonts.
    pub fn set_font_family_standard(&mut self, font_name: &str) -> &mut Self {
        let font_name_str = String::from_str(font_name);
        unsafe {
            ulViewConfigSetFontFamilyStandard(self.raw, font_name_str.raw());
        }
        self
    }
    
    /// Set the default font family for fixed fonts.
    pub fn set_font_family_fixed(&mut self, font_name: &str) -> &mut Self {
        let font_name_str = String::from_str(font_name);
        unsafe {
            ulViewConfigSetFontFamilyFixed(self.raw, font_name_str.raw());
        }
        self
    }
    
    /// Set the default font family for serif fonts.
    pub fn set_font_family_serif(&mut self, font_name: &str) -> &mut Self {
        let font_name_str = String::from_str(font_name);
        unsafe {
            ulViewConfigSetFontFamilySerif(self.raw, font_name_str.raw());
        }
        self
    }
    
    /// Set the default font family for sans-serif fonts.
    pub fn set_font_family_sans_serif(&mut self, font_name: &str) -> &mut Self {
        let font_name_str = String::from_str(font_name);
        unsafe {
            ulViewConfigSetFontFamilySansSerif(self.raw, font_name_str.raw());
        }
        self
    }
    
    /// Set the user agent string.
    pub fn set_user_agent(&mut self, agent_string: &str) -> &mut Self {
        let agent_string_str = String::from_str(agent_string);
        unsafe {
            ulViewConfigSetUserAgent(self.raw, agent_string_str.raw());
        }
        self
    }
}

impl Default for ViewConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ViewConfig {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyViewConfig(self.raw);
            }
        }
    }
}