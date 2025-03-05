use crate::ul::ffi::{
    ULClipboard, ULFileSystem, ULFontLoader, ULGPUDriver, ULLogger, ULString, ULSurfaceDefinition,
    ulPlatformSetClipboard, ulPlatformSetFileSystem, ulPlatformSetFontLoader,
    ulPlatformSetGPUDriver, ulPlatformSetLogger, ulPlatformSetSurfaceDefinition,
};
use crate::ul::string::String;

/// Static methods for configuring the platform.
pub struct Platform;

impl Platform {
    /// Set a custom logger implementation.
    pub fn set_logger(logger: ULLogger) {
        unsafe {
            ulPlatformSetLogger(logger);
        }
    }

    /// Set a custom file system implementation.
    pub fn set_file_system(file_system: ULFileSystem) {
        unsafe {
            ulPlatformSetFileSystem(file_system);
        }
    }

    /// Set a custom font loader implementation.
    pub fn set_font_loader(font_loader: ULFontLoader) {
        unsafe {
            ulPlatformSetFontLoader(font_loader);
        }
    }

    /// Set a custom surface definition.
    pub fn set_surface_definition(surface_definition: ULSurfaceDefinition) {
        unsafe {
            ulPlatformSetSurfaceDefinition(surface_definition);
        }
    }

    /// Set a custom GPU driver implementation.
    pub fn set_gpu_driver(gpu_driver: ULGPUDriver) {
        unsafe {
            ulPlatformSetGPUDriver(gpu_driver);
        }
    }

    /// Set a custom clipboard implementation.
    pub fn set_clipboard(clipboard: ULClipboard) {
        unsafe {
            ulPlatformSetClipboard(clipboard);
        }
    }

    /// Enable the default logger (requires AppCore).
    pub fn enable_default_logger() {
        unsafe extern "C" {
            fn ulEnableDefaultLogger();
        }

        unsafe {
            ulEnableDefaultLogger();
        }
    }

    /// Enable the platform file system (requires AppCore).
    pub fn enable_platform_file_system(base_dir: &str) {
        unsafe extern "C" {
            fn ulEnablePlatformFileSystem(base_dir: ULString);
        }

        let base_dir_str = String::from_str(base_dir);
        unsafe {
            ulEnablePlatformFileSystem(base_dir_str.raw());
        }
    }

    /// Enable the platform font loader (requires AppCore).
    pub fn enable_platform_font_loader() {
        unsafe extern "C" {
            fn ulEnablePlatformFontLoader();
        }

        unsafe {
            ulEnablePlatformFontLoader();
        }
    }
}
