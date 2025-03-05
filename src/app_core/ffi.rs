#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};

use crate::ul::String as ULString;
use crate::ul::ffi::{ULConfig, ULCursor, ULRenderer, ULView};

// Opaque struct types
pub enum C_Settings {}
pub enum C_App {}
pub enum C_Window {}
pub enum C_Monitor {}
pub enum C_Overlay {}

// Type aliases
pub type ULSettings = *mut C_Settings;
pub type ULApp = *mut C_App;
pub type ULWindow = *mut C_Window;
pub type ULMonitor = *mut C_Monitor;
pub type ULOverlay = *mut C_Overlay;

/// Window creation flags.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULWindowFlags {
    kWindowFlags_Borderless = 1 << 0,
    kWindowFlags_Titled = 1 << 1,
    kWindowFlags_Resizable = 1 << 2,
    kWindowFlags_Maximizable = 1 << 3,
    kWindowFlags_Hidden = 1 << 4,
}

pub type ULUpdateCallback = extern "C" fn(user_data: *mut c_void);
pub type ULCloseCallback = extern "C" fn(user_data: *mut c_void, window: ULWindow);
pub type ULResizeCallback =
    extern "C" fn(user_data: *mut c_void, window: ULWindow, width: c_uint, height: c_uint);

// Function definitions

// Settings functions
unsafe extern "C" {
    pub fn ulCreateSettings() -> ULSettings;
    pub fn ulDestroySettings(settings: ULSettings);
    pub fn ulSettingsSetDeveloperName(settings: ULSettings, name: ULString);
    pub fn ulSettingsSetAppName(settings: ULSettings, name: ULString);
    pub fn ulSettingsSetFileSystemPath(settings: ULSettings, path: ULString);
    pub fn ulSettingsSetLoadShadersFromFileSystem(settings: ULSettings, enabled: bool);
    pub fn ulSettingsSetForceCPURenderer(settings: ULSettings, force_cpu: bool);
}

// App functions
unsafe extern "C" {
    pub fn ulCreateApp(settings: ULSettings, config: ULConfig) -> ULApp;
    pub fn ulDestroyApp(app: ULApp);
    pub fn ulAppSetUpdateCallback(app: ULApp, callback: ULUpdateCallback, user_data: *mut c_void);
    pub fn ulAppIsRunning(app: ULApp) -> bool;
    pub fn ulAppGetMainMonitor(app: ULApp) -> ULMonitor;
    pub fn ulAppGetRenderer(app: ULApp) -> ULRenderer;
    pub fn ulAppRun(app: ULApp);
    pub fn ulAppQuit(app: ULApp);
}

// Monitor functions
unsafe extern "C" {
    pub fn ulMonitorGetScale(monitor: ULMonitor) -> c_double;
    pub fn ulMonitorGetWidth(monitor: ULMonitor) -> c_uint;
    pub fn ulMonitorGetHeight(monitor: ULMonitor) -> c_uint;
}

// Window functions
unsafe extern "C" {
    pub fn ulCreateWindow(
        monitor: ULMonitor,
        width: c_uint,
        height: c_uint,
        fullscreen: bool,
        window_flags: c_uint,
    ) -> ULWindow;
    pub fn ulDestroyWindow(window: ULWindow);
    pub fn ulWindowSetCloseCallback(
        window: ULWindow,
        callback: ULCloseCallback,
        user_data: *mut c_void,
    );
    pub fn ulWindowSetResizeCallback(
        window: ULWindow,
        callback: ULResizeCallback,
        user_data: *mut c_void,
    );
    pub fn ulWindowGetScreenWidth(window: ULWindow) -> c_uint;
    pub fn ulWindowGetWidth(window: ULWindow) -> c_uint;
    pub fn ulWindowGetScreenHeight(window: ULWindow) -> c_uint;
    pub fn ulWindowGetHeight(window: ULWindow) -> c_uint;
    pub fn ulWindowMoveTo(window: ULWindow, x: c_int, y: c_int);
    pub fn ulWindowMoveToCenter(window: ULWindow);
    pub fn ulWindowGetPositionX(window: ULWindow) -> c_int;
    pub fn ulWindowGetPositionY(window: ULWindow) -> c_int;
    pub fn ulWindowIsFullscreen(window: ULWindow) -> bool;
    pub fn ulWindowGetScale(window: ULWindow) -> c_double;
    pub fn ulWindowSetTitle(window: ULWindow, title: *const c_char);
    pub fn ulWindowSetCursor(window: ULWindow, cursor: ULCursor);
    pub fn ulWindowShow(window: ULWindow);
    pub fn ulWindowHide(window: ULWindow);
    pub fn ulWindowIsVisible(window: ULWindow) -> bool;
    pub fn ulWindowClose(window: ULWindow);
    pub fn ulWindowScreenToPixels(window: ULWindow, val: c_int) -> c_int;
    pub fn ulWindowPixelsToScreen(window: ULWindow, val: c_int) -> c_int;
    pub fn ulWindowGetNativeHandle(window: ULWindow) -> *mut c_void;
}

// Overlay functions
unsafe extern "C" {
    pub fn ulCreateOverlay(
        window: ULWindow,
        width: c_uint,
        height: c_uint,
        x: c_int,
        y: c_int,
    ) -> ULOverlay;
    pub fn ulCreateOverlayWithView(window: ULWindow, view: ULView, x: c_int, y: c_int)
    -> ULOverlay;
    pub fn ulDestroyOverlay(overlay: ULOverlay);
    pub fn ulOverlayGetView(overlay: ULOverlay) -> ULView;
    pub fn ulOverlayGetWidth(overlay: ULOverlay) -> c_uint;
    pub fn ulOverlayGetHeight(overlay: ULOverlay) -> c_uint;
    pub fn ulOverlayGetX(overlay: ULOverlay) -> c_int;
    pub fn ulOverlayGetY(overlay: ULOverlay) -> c_int;
    pub fn ulOverlayMoveTo(overlay: ULOverlay, x: c_int, y: c_int);
    pub fn ulOverlayResize(overlay: ULOverlay, width: c_uint, height: c_uint);
    pub fn ulOverlayIsHidden(overlay: ULOverlay) -> bool;
    pub fn ulOverlayHide(overlay: ULOverlay);
    pub fn ulOverlayShow(overlay: ULOverlay);
    pub fn ulOverlayHasFocus(overlay: ULOverlay) -> bool;
    pub fn ulOverlayFocus(overlay: ULOverlay);
    pub fn ulOverlayUnfocus(overlay: ULOverlay);
}

// Platform functions
unsafe extern "C" {
    pub fn ulEnablePlatformFontLoader();
    pub fn ulEnablePlatformFileSystem(base_dir: ULString);
    pub fn ulEnableDefaultLogger(log_path: ULString);
}
