use crate::app_core::ffi::{
    ulEnableDefaultLogger, ulEnablePlatformFileSystem, ulEnablePlatformFontLoader,
};
use crate::ul::String;

/// Initialize the platform font loader.
///
/// This is only needed if you are not calling App::new().
/// It initializes the platform font loader and sets it as the current FontLoader.
pub fn enable_platform_font_loader() {
    unsafe {
        ulEnablePlatformFontLoader();
    }
}

/// Initialize the platform file system.
///
/// This is only needed if you are not calling App::new().
/// It initializes the platform file system (needed for loading file:/// URLs)
/// and sets it as the current FileSystem.
///
/// # Arguments
///
/// * `base_dir` - A base directory path to resolve relative paths against
pub fn enable_platform_file_system(base_dir: &str) {
    let base_dir_string = String::from_str(base_dir);
    unsafe {
        ulEnablePlatformFileSystem(base_dir_string.raw());
    }
}

/// Initialize the default logger.
///
/// This is only needed if you are not calling App::new().
/// It initializes the default logger (writes the log to a file).
///
/// # Arguments
///
/// * `log_path` - A writable log path to write the log to (e.g., "./ultralight.log")
pub fn enable_default_logger(log_path: &str) {
    let log_path_string = String::from_str(log_path);
    unsafe {
        ulEnableDefaultLogger(log_path_string.raw());
    }
}