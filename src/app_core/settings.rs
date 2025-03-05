use crate::app_core::ffi::{ULSettings, ulCreateSettings, ulDestroySettings, ulSettingsSetDeveloperName,
                    ulSettingsSetAppName, ulSettingsSetFileSystemPath, 
                    ulSettingsSetLoadShadersFromFileSystem, ulSettingsSetForceCPURenderer};
use crate::ul::String as ULString;

/// Settings used to customize AppCore runtime behavior.
pub struct Settings {
    raw: ULSettings,
}

impl Settings {
    /// Create a new Settings instance with default values.
    pub fn new() -> Self {
        unsafe {
            let raw = ulCreateSettings();
            Self { raw }
        }
    }
    
    /// Get a reference to the raw ULSettings.
    pub fn raw(&self) -> ULSettings {
        self.raw
    }
    
    /// Set the name of the developer of this app.
    ///
    /// This is used to generate a unique path to store local application data
    /// on the user's machine.
    ///
    /// Default is "MyCompany".
    pub fn set_developer_name(&mut self, name: &str) -> &mut Self {
        let ul_name = ULString::from_str(name);
        unsafe {
            ulSettingsSetDeveloperName(self.raw, ul_name);
        }
        self
    }
    
    /// Set the name of this app.
    ///
    /// This is used to generate a unique path to store local application data
    /// on the user's machine.
    ///
    /// Default is "MyApp".
    pub fn set_app_name(&mut self, name: &str) -> &mut Self {
        let ul_name = ULString::from_str(name);
        unsafe {
            ulSettingsSetAppName(self.raw, ul_name);
        }
        self
    }
    
    /// Set the root file path for the file system.
    ///
    /// This will be used to resolve all file URLs, e.g., file:///page.html
    ///
    /// The default path is "./assets/".
    ///
    /// This relative path is resolved using the following logic:
    /// - Windows: relative to the executable path
    /// - Linux: relative to the executable path
    /// - macOS: relative to YourApp.app/Contents/Resources/
    pub fn set_file_system_path(&mut self, path: &str) -> &mut Self {
        let ul_path = ULString::from_str(path);
        unsafe {
            ulSettingsSetFileSystemPath(self.raw, ul_path);
        }
        self
    }
    
    /// Set whether or not to load and compile shaders from the file system.
    ///
    /// When enabled, shaders will be loaded from the /shaders/ path (relative 
    /// to file_system_path).
    ///
    /// If this is false (the default), pre-compiled shaders will be loaded
    /// from memory which speeds up application startup time.
    pub fn set_load_shaders_from_file_system(&mut self, enabled: bool) -> &mut Self {
        unsafe {
            ulSettingsSetLoadShadersFromFileSystem(self.raw, enabled);
        }
        self
    }
    
    /// Force the engine to always use the CPU renderer.
    ///
    /// By default, the GPU renderer is used when a compatible GPU is detected.
    pub fn set_force_cpu_renderer(&mut self, force_cpu: bool) -> &mut Self {
        unsafe {
            ulSettingsSetForceCPURenderer(self.raw, force_cpu);
        }
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroySettings(self.raw);
            }
        }
    }
}