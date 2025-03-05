#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::{
    c_char, c_double, c_float, c_int, c_uchar, c_uint, c_ulonglong, c_ushort, c_void,
};

// Re-export JSContextRef from JavaScriptCore
pub type JSContextRef = *mut c_void;

// Opaque struct types
pub enum C_Config {}
pub enum C_Renderer {}
pub enum C_Session {}
pub enum C_ViewConfig {}
pub enum C_View {}
pub enum C_Bitmap {}
pub enum C_String {}
pub enum C_Buffer {}
pub enum C_KeyEvent {}
pub enum C_MouseEvent {}
pub enum C_ScrollEvent {}
pub enum C_GamepadEvent {}
pub enum C_GamepadAxisEvent {}
pub enum C_GamepadButtonEvent {}
pub enum C_Surface {}
pub enum C_FontFile {}
pub enum C_ImageSource {}

// Type aliases
pub type ULConfig = *mut C_Config;
pub type ULRenderer = *mut C_Renderer;
pub type ULSession = *mut C_Session;
pub type ULViewConfig = *mut C_ViewConfig;
pub type ULView = *mut C_View;
pub type ULBitmap = *mut C_Bitmap;
pub type ULString = *mut C_String;
pub type ULBuffer = *mut C_Buffer;
pub type ULKeyEvent = *mut C_KeyEvent;
pub type ULMouseEvent = *mut C_MouseEvent;
pub type ULScrollEvent = *mut C_ScrollEvent;
pub type ULGamepadEvent = *mut C_GamepadEvent;
pub type ULGamepadAxisEvent = *mut C_GamepadAxisEvent;
pub type ULGamepadButtonEvent = *mut C_GamepadButtonEvent;
pub type ULSurface = *mut C_Surface;
pub type ULBitmapSurface = *mut C_Surface;
pub type ULFontFile = *mut C_FontFile;
pub type ULImageSource = *mut C_ImageSource;
pub type ULChar16 = c_ushort;

// Enums
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULMessageSource {
    kMessageSource_XML = 0,
    kMessageSource_JS,
    kMessageSource_Network,
    kMessageSource_ConsoleAPI,
    kMessageSource_Storage,
    kMessageSource_AppCache,
    kMessageSource_Rendering,
    kMessageSource_CSS,
    kMessageSource_Security,
    kMessageSource_ContentBlocker,
    kMessageSource_Media,
    kMessageSource_MediaSource,
    kMessageSource_WebRTC,
    kMessageSource_ITPDebug,
    kMessageSource_PrivateClickMeasurement,
    kMessageSource_PaymentRequest,
    kMessageSource_Other,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULMessageLevel {
    kMessageLevel_Log = 0,
    kMessageLevel_Warning,
    kMessageLevel_Error,
    kMessageLevel_Debug,
    kMessageLevel_Info,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULCursor {
    kCursor_Pointer = 0,
    kCursor_Cross,
    kCursor_Hand,
    kCursor_IBeam,
    kCursor_Wait,
    kCursor_Help,
    kCursor_EastResize,
    kCursor_NorthResize,
    kCursor_NorthEastResize,
    kCursor_NorthWestResize,
    kCursor_SouthResize,
    kCursor_SouthEastResize,
    kCursor_SouthWestResize,
    kCursor_WestResize,
    kCursor_NorthSouthResize,
    kCursor_EastWestResize,
    kCursor_NorthEastSouthWestResize,
    kCursor_NorthWestSouthEastResize,
    kCursor_ColumnResize,
    kCursor_RowResize,
    kCursor_MiddlePanning,
    kCursor_EastPanning,
    kCursor_NorthPanning,
    kCursor_NorthEastPanning,
    kCursor_NorthWestPanning,
    kCursor_SouthPanning,
    kCursor_SouthEastPanning,
    kCursor_SouthWestPanning,
    kCursor_WestPanning,
    kCursor_Move,
    kCursor_VerticalText,
    kCursor_Cell,
    kCursor_ContextMenu,
    kCursor_Alias,
    kCursor_Progress,
    kCursor_NoDrop,
    kCursor_Copy,
    kCursor_None,
    kCursor_NotAllowed,
    kCursor_ZoomIn,
    kCursor_ZoomOut,
    kCursor_Grab,
    kCursor_Grabbing,
    kCursor_Custom,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULBitmapFormat {
    kBitmapFormat_A8_UNORM,
    kBitmapFormat_BGRA8_UNORM_SRGB,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULKeyEventType {
    kKeyEventType_KeyDown,
    kKeyEventType_KeyUp,
    kKeyEventType_RawKeyDown,
    kKeyEventType_Char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULMouseEventType {
    kMouseEventType_MouseMoved,
    kMouseEventType_MouseDown,
    kMouseEventType_MouseUp,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULMouseButton {
    kMouseButton_None = 0,
    kMouseButton_Left,
    kMouseButton_Middle,
    kMouseButton_Right,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULScrollEventType {
    kScrollEventType_ScrollByPixel,
    kScrollEventType_ScrollByPage,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULGamepadEventType {
    kGamepadEventType_Connected,
    kGamepadEventType_Disconnected,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULFaceWinding {
    kFaceWinding_Clockwise,
    kFaceWinding_CounterClockwise,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULFontHinting {
    kFontHinting_Smooth,
    kFontHinting_Normal,
    kFontHinting_Monochrome,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULRect {
    pub left: c_float,
    pub top: c_float,
    pub right: c_float,
    pub bottom: c_float,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULIntRect {
    pub left: c_int,
    pub top: c_int,
    pub right: c_int,
    pub bottom: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULRenderTarget {
    pub is_empty: bool,
    pub width: c_uint,
    pub height: c_uint,
    pub texture_id: c_uint,
    pub texture_width: c_uint,
    pub texture_height: c_uint,
    pub texture_format: ULBitmapFormat,
    pub uv_coords: ULRect,
    pub render_buffer_id: c_uint,
}

// Callback types
pub type ULClipboardClearCallback = extern "C" fn();
pub type ULClipboardReadPlainTextCallback = extern "C" fn(result: ULString);
pub type ULClipboardWritePlainTextCallback = extern "C" fn(text: ULString);
pub type ULDestroyBufferCallback = extern "C" fn(user_data: *mut c_void, data: *mut c_void);

// Struct definitions
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULClipboard {
    pub clear: ULClipboardClearCallback,
    pub read_plain_text: ULClipboardReadPlainTextCallback,
    pub write_plain_text: ULClipboardWritePlainTextCallback,
}

// Logger related
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULLogLevel {
    kLogLevel_Error = 0,
    kLogLevel_Warning,
    kLogLevel_Info,
}

pub type ULLoggerLogMessageCallback = extern "C" fn(log_level: ULLogLevel, message: ULString);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULLogger {
    pub log_message: ULLoggerLogMessageCallback,
}

// FileSystem related
pub type ULFileSystemFileExistsCallback = extern "C" fn(path: ULString) -> bool;
pub type ULFileSystemGetFileMimeTypeCallback = extern "C" fn(path: ULString) -> ULString;
pub type ULFileSystemGetFileCharsetCallback = extern "C" fn(path: ULString) -> ULString;
pub type ULFileSystemOpenFileCallback = extern "C" fn(path: ULString) -> ULBuffer;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULFileSystem {
    pub file_exists: ULFileSystemFileExistsCallback,
    pub get_file_mime_type: ULFileSystemGetFileMimeTypeCallback,
    pub get_file_charset: ULFileSystemGetFileCharsetCallback,
    pub open_file: ULFileSystemOpenFileCallback,
}

// FontLoader related
pub type ULFontLoaderGetFallbackFont = extern "C" fn() -> ULString;
pub type ULFontLoaderGetFallbackFontForCharacters =
    extern "C" fn(characters: ULString, weight: c_int, italic: bool) -> ULString;
pub type ULFontLoaderLoad =
    extern "C" fn(family: ULString, weight: c_int, italic: bool) -> ULFontFile;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULFontLoader {
    pub get_fallback_font: ULFontLoaderGetFallbackFont,
    pub get_fallback_font_for_characters: ULFontLoaderGetFallbackFontForCharacters,
    pub load: ULFontLoaderLoad,
}

// Surface related
pub type ULSurfaceDefinitionCreateCallback =
    extern "C" fn(width: c_uint, height: c_uint) -> *mut c_void;
pub type ULSurfaceDefinitionDestroyCallback = extern "C" fn(user_data: *mut c_void);
pub type ULSurfaceDefinitionGetWidthCallback = extern "C" fn(user_data: *mut c_void) -> c_uint;
pub type ULSurfaceDefinitionGetHeightCallback = extern "C" fn(user_data: *mut c_void) -> c_uint;
pub type ULSurfaceDefinitionGetRowBytesCallback = extern "C" fn(user_data: *mut c_void) -> c_uint;
pub type ULSurfaceDefinitionGetSizeCallback = extern "C" fn(user_data: *mut c_void) -> usize;
pub type ULSurfaceDefinitionLockPixelsCallback =
    extern "C" fn(user_data: *mut c_void) -> *mut c_void;
pub type ULSurfaceDefinitionUnlockPixelsCallback = extern "C" fn(user_data: *mut c_void);
pub type ULSurfaceDefinitionResizeCallback =
    extern "C" fn(user_data: *mut c_void, width: c_uint, height: c_uint);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULSurfaceDefinition {
    pub create: ULSurfaceDefinitionCreateCallback,
    pub destroy: ULSurfaceDefinitionDestroyCallback,
    pub get_width: ULSurfaceDefinitionGetWidthCallback,
    pub get_height: ULSurfaceDefinitionGetHeightCallback,
    pub get_row_bytes: ULSurfaceDefinitionGetRowBytesCallback,
    pub get_size: ULSurfaceDefinitionGetSizeCallback,
    pub lock_pixels: ULSurfaceDefinitionLockPixelsCallback,
    pub unlock_pixels: ULSurfaceDefinitionUnlockPixelsCallback,
    pub resize: ULSurfaceDefinitionResizeCallback,
}

// View callbacks
pub type ULChangeTitleCallback =
    extern "C" fn(user_data: *mut c_void, caller: ULView, title: ULString);
pub type ULChangeURLCallback = extern "C" fn(user_data: *mut c_void, caller: ULView, url: ULString);
pub type ULChangeTooltipCallback =
    extern "C" fn(user_data: *mut c_void, caller: ULView, tooltip: ULString);
pub type ULChangeCursorCallback =
    extern "C" fn(user_data: *mut c_void, caller: ULView, cursor: ULCursor);
pub type ULAddConsoleMessageCallback = extern "C" fn(
    user_data: *mut c_void,
    caller: ULView,
    source: ULMessageSource,
    level: ULMessageLevel,
    message: ULString,
    line_number: c_uint,
    column_number: c_uint,
    source_id: ULString,
);
pub type ULCreateChildViewCallback = extern "C" fn(
    user_data: *mut c_void,
    caller: ULView,
    opener_url: ULString,
    target_url: ULString,
    is_popup: bool,
    popup_rect: ULIntRect,
) -> ULView;
pub type ULCreateInspectorViewCallback = extern "C" fn(
    user_data: *mut c_void,
    caller: ULView,
    is_local: bool,
    inspected_url: ULString,
) -> ULView;
pub type ULBeginLoadingCallback = extern "C" fn(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
);
pub type ULFinishLoadingCallback = extern "C" fn(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
);
pub type ULFailLoadingCallback = extern "C" fn(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
    description: ULString,
    error_domain: ULString,
    error_code: c_int,
);
pub type ULWindowObjectReadyCallback = extern "C" fn(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
);
pub type ULDOMReadyCallback = extern "C" fn(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
);
pub type ULUpdateHistoryCallback = extern "C" fn(user_data: *mut c_void, caller: ULView);

// GPU Driver related
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULRenderBuffer {
    pub texture_id: c_uint,
    pub width: c_uint,
    pub height: c_uint,
    pub has_stencil_buffer: bool,
    pub has_depth_buffer: bool,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ULVertex_2f_4ub_2f {
    pub pos: [c_float; 2],
    pub color: [c_uchar; 4],
    pub obj: [c_float; 2],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ULVertex_2f_4ub_2f_2f_28f {
    pub pos: [c_float; 2],
    pub color: [c_uchar; 4],
    pub tex: [c_float; 2],
    pub obj: [c_float; 2],
    pub data0: [c_float; 4],
    pub data1: [c_float; 4],
    pub data2: [c_float; 4],
    pub data3: [c_float; 4],
    pub data4: [c_float; 4],
    pub data5: [c_float; 4],
    pub data6: [c_float; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULVertexBufferFormat {
    kVertexBufferFormat_2f_4ub_2f,
    kVertexBufferFormat_2f_4ub_2f_2f_28f,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULVertexBuffer {
    pub format: ULVertexBufferFormat,
    pub size: c_uint,
    pub data: *mut c_uchar,
}

pub type ULIndexType = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULIndexBuffer {
    pub size: c_uint,
    pub data: *mut c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULShaderType {
    kShaderType_Fill,
    kShaderType_FillPath,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULMatrix4x4 {
    pub data: [c_float; 16],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULvec4 {
    pub value: [c_float; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULGPUState {
    pub viewport_width: c_uint,
    pub viewport_height: c_uint,
    pub transform: ULMatrix4x4,
    pub enable_texturing: bool,
    pub enable_blend: bool,
    pub shader_type: c_uchar,
    pub render_buffer_id: c_uint,
    pub texture_1_id: c_uint,
    pub texture_2_id: c_uint,
    pub texture_3_id: c_uint,
    pub uniform_scalar: [c_float; 8],
    pub uniform_vector: [ULvec4; 8],
    pub clip_size: c_uchar,
    pub clip: [ULMatrix4x4; 8],
    pub enable_scissor: bool,
    pub scissor_rect: ULIntRect,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ULCommandType {
    kCommandType_ClearRenderBuffer,
    kCommandType_DrawGeometry,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULCommand {
    pub command_type: c_uchar,
    pub gpu_state: ULGPUState,
    pub geometry_id: c_uint,
    pub indices_count: c_uint,
    pub indices_offset: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULCommandList {
    pub size: c_uint,
    pub commands: *mut ULCommand,
}

pub type ULGPUDriverBeginSynchronizeCallback = extern "C" fn();
pub type ULGPUDriverEndSynchronizeCallback = extern "C" fn();
pub type ULGPUDriverNextTextureIdCallback = extern "C" fn() -> c_uint;
pub type ULGPUDriverCreateTextureCallback = extern "C" fn(texture_id: c_uint, bitmap: ULBitmap);
pub type ULGPUDriverUpdateTextureCallback = extern "C" fn(texture_id: c_uint, bitmap: ULBitmap);
pub type ULGPUDriverDestroyTextureCallback = extern "C" fn(texture_id: c_uint);
pub type ULGPUDriverNextRenderBufferIdCallback = extern "C" fn() -> c_uint;
pub type ULGPUDriverCreateRenderBufferCallback =
    extern "C" fn(render_buffer_id: c_uint, buffer: ULRenderBuffer);
pub type ULGPUDriverDestroyRenderBufferCallback = extern "C" fn(render_buffer_id: c_uint);
pub type ULGPUDriverNextGeometryIdCallback = extern "C" fn() -> c_uint;
pub type ULGPUDriverCreateGeometryCallback =
    extern "C" fn(geometry_id: c_uint, vertices: ULVertexBuffer, indices: ULIndexBuffer);
pub type ULGPUDriverUpdateGeometryCallback =
    extern "C" fn(geometry_id: c_uint, vertices: ULVertexBuffer, indices: ULIndexBuffer);
pub type ULGPUDriverDestroyGeometryCallback = extern "C" fn(geometry_id: c_uint);
pub type ULGPUDriverUpdateCommandListCallback = extern "C" fn(list: ULCommandList);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ULGPUDriver {
    pub begin_synchronize: ULGPUDriverBeginSynchronizeCallback,
    pub end_synchronize: ULGPUDriverEndSynchronizeCallback,
    pub next_texture_id: ULGPUDriverNextTextureIdCallback,
    pub create_texture: ULGPUDriverCreateTextureCallback,
    pub update_texture: ULGPUDriverUpdateTextureCallback,
    pub destroy_texture: ULGPUDriverDestroyTextureCallback,
    pub next_render_buffer_id: ULGPUDriverNextRenderBufferIdCallback,
    pub create_render_buffer: ULGPUDriverCreateRenderBufferCallback,
    pub destroy_render_buffer: ULGPUDriverDestroyRenderBufferCallback,
    pub next_geometry_id: ULGPUDriverNextGeometryIdCallback,
    pub create_geometry: ULGPUDriverCreateGeometryCallback,
    pub update_geometry: ULGPUDriverUpdateGeometryCallback,
    pub destroy_geometry: ULGPUDriverDestroyGeometryCallback,
    pub update_command_list: ULGPUDriverUpdateCommandListCallback,
}

// Function definitions

// Version functions
unsafe extern "C" {
    pub fn ulVersionString() -> *const c_char;
    pub fn ulVersionMajor() -> c_uint;
    pub fn ulVersionMinor() -> c_uint;
    pub fn ulVersionPatch() -> c_uint;
    pub fn ulWebKitVersionString() -> *const c_char;
}

// Rect functions
unsafe extern "C" {
    pub fn ulRectIsEmpty(rect: ULRect) -> bool;
    pub fn ulRectMakeEmpty() -> ULRect;
    pub fn ulIntRectIsEmpty(rect: ULIntRect) -> bool;
    pub fn ulIntRectMakeEmpty() -> ULIntRect;
    pub fn ulApplyProjection(
        transform: ULMatrix4x4,
        viewport_width: c_float,
        viewport_height: c_float,
        flip_y: bool,
    ) -> ULMatrix4x4;
}

// String functions
unsafe extern "C" {
    pub fn ulCreateString(str: *const c_char) -> ULString;
    pub fn ulCreateStringUTF8(str: *const c_char, len: usize) -> ULString;
    pub fn ulCreateStringUTF16(str: *mut ULChar16, len: usize) -> ULString;
    pub fn ulCreateStringFromCopy(str: ULString) -> ULString;
    pub fn ulDestroyString(str: ULString);
    pub fn ulStringGetData(str: ULString) -> *mut c_char;
    pub fn ulStringGetLength(str: ULString) -> usize;
    pub fn ulStringIsEmpty(str: ULString) -> bool;
    pub fn ulStringAssignString(str: ULString, new_str: ULString);
    pub fn ulStringAssignCString(str: ULString, c_str: *const c_char);
}

// Buffer functions
unsafe extern "C" {
    pub fn ulCreateBuffer(
        data: *mut c_void,
        size: usize,
        user_data: *mut c_void,
        destruction_callback: ULDestroyBufferCallback,
    ) -> ULBuffer;
    pub fn ulCreateBufferFromCopy(data: *const c_void, size: usize) -> ULBuffer;
    pub fn ulDestroyBuffer(buffer: ULBuffer);
    pub fn ulBufferGetData(buffer: ULBuffer) -> *mut c_void;
    pub fn ulBufferGetSize(buffer: ULBuffer) -> usize;
    pub fn ulBufferGetUserData(buffer: ULBuffer) -> *mut c_void;
    pub fn ulBufferOwnsData(buffer: ULBuffer) -> bool;
}

// Bitmap functions
unsafe extern "C" {
    pub fn ulCreateEmptyBitmap() -> ULBitmap;
    pub fn ulCreateBitmap(width: c_uint, height: c_uint, format: ULBitmapFormat) -> ULBitmap;
    pub fn ulCreateBitmapFromPixels(
        width: c_uint,
        height: c_uint,
        format: ULBitmapFormat,
        row_bytes: c_uint,
        pixels: *const c_void,
        size: usize,
        should_copy: bool,
    ) -> ULBitmap;
    pub fn ulCreateBitmapFromCopy(existing_bitmap: ULBitmap) -> ULBitmap;
    pub fn ulDestroyBitmap(bitmap: ULBitmap);
    pub fn ulBitmapGetWidth(bitmap: ULBitmap) -> c_uint;
    pub fn ulBitmapGetHeight(bitmap: ULBitmap) -> c_uint;
    pub fn ulBitmapGetFormat(bitmap: ULBitmap) -> ULBitmapFormat;
    pub fn ulBitmapGetBpp(bitmap: ULBitmap) -> c_uint;
    pub fn ulBitmapGetRowBytes(bitmap: ULBitmap) -> c_uint;
    pub fn ulBitmapGetSize(bitmap: ULBitmap) -> usize;
    pub fn ulBitmapOwnsPixels(bitmap: ULBitmap) -> bool;
    pub fn ulBitmapLockPixels(bitmap: ULBitmap) -> *mut c_void;
    pub fn ulBitmapUnlockPixels(bitmap: ULBitmap);
    pub fn ulBitmapRawPixels(bitmap: ULBitmap) -> *mut c_void;
    pub fn ulBitmapIsEmpty(bitmap: ULBitmap) -> bool;
    pub fn ulBitmapErase(bitmap: ULBitmap);
    pub fn ulBitmapWritePNG(bitmap: ULBitmap, path: *const c_char) -> bool;
    pub fn ulBitmapSwapRedBlueChannels(bitmap: ULBitmap);
}

// Config functions
unsafe extern "C" {
    pub fn ulCreateConfig() -> ULConfig;
    pub fn ulDestroyConfig(config: ULConfig);
    pub fn ulConfigSetCachePath(config: ULConfig, cache_path: ULString);
    pub fn ulConfigSetResourcePathPrefix(config: ULConfig, resource_path_prefix: ULString);
    pub fn ulConfigSetFaceWinding(config: ULConfig, winding: ULFaceWinding);
    pub fn ulConfigSetFontHinting(config: ULConfig, font_hinting: ULFontHinting);
    pub fn ulConfigSetFontGamma(config: ULConfig, font_gamma: c_double);
    pub fn ulConfigSetUserStylesheet(config: ULConfig, css_string: ULString);
    pub fn ulConfigSetForceRepaint(config: ULConfig, enabled: bool);
    pub fn ulConfigSetAnimationTimerDelay(config: ULConfig, delay: c_double);
    pub fn ulConfigSetScrollTimerDelay(config: ULConfig, delay: c_double);
    pub fn ulConfigSetRecycleDelay(config: ULConfig, delay: c_double);
    pub fn ulConfigSetMemoryCacheSize(config: ULConfig, size: c_uint);
    pub fn ulConfigSetPageCacheSize(config: ULConfig, size: c_uint);
    pub fn ulConfigSetOverrideRAMSize(config: ULConfig, size: c_uint);
    pub fn ulConfigSetMinLargeHeapSize(config: ULConfig, size: c_uint);
    pub fn ulConfigSetMinSmallHeapSize(config: ULConfig, size: c_uint);
    pub fn ulConfigSetNumRendererThreads(config: ULConfig, num_renderer_threads: c_uint);
    pub fn ulConfigSetMaxUpdateTime(config: ULConfig, max_update_time: c_double);
    pub fn ulConfigSetBitmapAlignment(config: ULConfig, bitmap_alignment: c_uint);
}

// View Config functions
unsafe extern "C" {
    pub fn ulCreateViewConfig() -> ULViewConfig;
    pub fn ulDestroyViewConfig(config: ULViewConfig);
    pub fn ulViewConfigSetDisplayId(config: ULViewConfig, display_id: c_uint);
    pub fn ulViewConfigSetIsAccelerated(config: ULViewConfig, is_accelerated: bool);
    pub fn ulViewConfigSetIsTransparent(config: ULViewConfig, is_transparent: bool);
    pub fn ulViewConfigSetInitialDeviceScale(config: ULViewConfig, initial_device_scale: c_double);
    pub fn ulViewConfigSetInitialFocus(config: ULViewConfig, is_focused: bool);
    pub fn ulViewConfigSetEnableImages(config: ULViewConfig, enabled: bool);
    pub fn ulViewConfigSetEnableJavaScript(config: ULViewConfig, enabled: bool);
    pub fn ulViewConfigSetFontFamilyStandard(config: ULViewConfig, font_name: ULString);
    pub fn ulViewConfigSetFontFamilyFixed(config: ULViewConfig, font_name: ULString);
    pub fn ulViewConfigSetFontFamilySerif(config: ULViewConfig, font_name: ULString);
    pub fn ulViewConfigSetFontFamilySansSerif(config: ULViewConfig, font_name: ULString);
    pub fn ulViewConfigSetUserAgent(config: ULViewConfig, agent_string: ULString);
}

// Platform functions
unsafe extern "C" {
    pub fn ulPlatformSetLogger(logger: ULLogger);
    pub fn ulPlatformSetFileSystem(file_system: ULFileSystem);
    pub fn ulPlatformSetFontLoader(font_loader: ULFontLoader);
    pub fn ulPlatformSetSurfaceDefinition(surface_definition: ULSurfaceDefinition);
    pub fn ulPlatformSetGPUDriver(gpu_driver: ULGPUDriver);
    pub fn ulPlatformSetClipboard(clipboard: ULClipboard);
}

// Renderer functions
unsafe extern "C" {
    pub fn ulCreateRenderer(config: ULConfig) -> ULRenderer;
    pub fn ulDestroyRenderer(renderer: ULRenderer);
    pub fn ulUpdate(renderer: ULRenderer);
    pub fn ulRefreshDisplay(renderer: ULRenderer, display_id: c_uint);
    pub fn ulRender(renderer: ULRenderer);
    pub fn ulPurgeMemory(renderer: ULRenderer);
    pub fn ulLogMemoryUsage(renderer: ULRenderer);
    pub fn ulStartRemoteInspectorServer(
        renderer: ULRenderer,
        address: *const c_char,
        port: c_ushort,
    ) -> bool;
    pub fn ulSetGamepadDetails(
        renderer: ULRenderer,
        index: c_uint,
        id: ULString,
        axis_count: c_uint,
        button_count: c_uint,
    );
    pub fn ulFireGamepadEvent(renderer: ULRenderer, evt: ULGamepadEvent);
    pub fn ulFireGamepadAxisEvent(renderer: ULRenderer, evt: ULGamepadAxisEvent);
    pub fn ulFireGamepadButtonEvent(renderer: ULRenderer, evt: ULGamepadButtonEvent);
}

// Session functions
unsafe extern "C" {
    pub fn ulCreateSession(renderer: ULRenderer, is_persistent: bool, name: ULString) -> ULSession;
    pub fn ulDestroySession(session: ULSession);
    pub fn ulDefaultSession(renderer: ULRenderer) -> ULSession;
    pub fn ulSessionIsPersistent(session: ULSession) -> bool;
    pub fn ulSessionGetName(session: ULSession) -> ULString;
    pub fn ulSessionGetId(session: ULSession) -> c_ulonglong;
    pub fn ulSessionGetDiskPath(session: ULSession) -> ULString;
}

// Surface functions
unsafe extern "C" {
    pub fn ulSurfaceGetWidth(surface: ULSurface) -> c_uint;
    pub fn ulSurfaceGetHeight(surface: ULSurface) -> c_uint;
    pub fn ulSurfaceGetRowBytes(surface: ULSurface) -> c_uint;
    pub fn ulSurfaceGetSize(surface: ULSurface) -> usize;
    pub fn ulSurfaceLockPixels(surface: ULSurface) -> *mut c_void;
    pub fn ulSurfaceUnlockPixels(surface: ULSurface);
    pub fn ulSurfaceResize(surface: ULSurface, width: c_uint, height: c_uint);
    pub fn ulSurfaceSetDirtyBounds(surface: ULSurface, bounds: ULIntRect);
    pub fn ulSurfaceGetDirtyBounds(surface: ULSurface) -> ULIntRect;
    pub fn ulSurfaceClearDirtyBounds(surface: ULSurface);
    pub fn ulSurfaceGetUserData(surface: ULSurface) -> *mut c_void;
    pub fn ulBitmapSurfaceGetBitmap(surface: ULBitmapSurface) -> ULBitmap;
}

// View functions
unsafe extern "C" {
    pub fn ulCreateView(
        renderer: ULRenderer,
        width: c_uint,
        height: c_uint,
        view_config: ULViewConfig,
        session: ULSession,
    ) -> ULView;
    pub fn ulDestroyView(view: ULView);
    pub fn ulViewGetURL(view: ULView) -> ULString;
    pub fn ulViewGetTitle(view: ULView) -> ULString;
    pub fn ulViewGetWidth(view: ULView) -> c_uint;
    pub fn ulViewGetHeight(view: ULView) -> c_uint;
    pub fn ulViewGetDisplayId(view: ULView) -> c_uint;
    pub fn ulViewSetDisplayId(view: ULView, display_id: c_uint);
    pub fn ulViewGetDeviceScale(view: ULView) -> c_double;
    pub fn ulViewSetDeviceScale(view: ULView, scale: c_double);
    pub fn ulViewIsAccelerated(view: ULView) -> bool;
    pub fn ulViewIsTransparent(view: ULView) -> bool;
    pub fn ulViewIsLoading(view: ULView) -> bool;
    pub fn ulViewGetRenderTarget(view: ULView) -> ULRenderTarget;
    pub fn ulViewGetSurface(view: ULView) -> ULSurface;
    pub fn ulViewLoadHTML(view: ULView, html_string: ULString);
    pub fn ulViewLoadURL(view: ULView, url_string: ULString);
    pub fn ulViewResize(view: ULView, width: c_uint, height: c_uint);
    pub fn ulViewLockJSContext(view: ULView) -> JSContextRef;
    pub fn ulViewUnlockJSContext(view: ULView);
    pub fn ulViewEvaluateScript(
        view: ULView,
        js_string: ULString,
        exception: *mut ULString,
    ) -> ULString;
    pub fn ulViewCanGoBack(view: ULView) -> bool;
    pub fn ulViewCanGoForward(view: ULView) -> bool;
    pub fn ulViewGoBack(view: ULView);
    pub fn ulViewGoForward(view: ULView);
    pub fn ulViewGoToHistoryOffset(view: ULView, offset: c_int);
    pub fn ulViewReload(view: ULView);
    pub fn ulViewStop(view: ULView);
    pub fn ulViewFocus(view: ULView);
    pub fn ulViewUnfocus(view: ULView);
    pub fn ulViewHasFocus(view: ULView) -> bool;
    pub fn ulViewHasInputFocus(view: ULView) -> bool;
    pub fn ulViewFireKeyEvent(view: ULView, key_event: ULKeyEvent);
    pub fn ulViewFireMouseEvent(view: ULView, mouse_event: ULMouseEvent);
    pub fn ulViewFireScrollEvent(view: ULView, scroll_event: ULScrollEvent);
    pub fn ulViewSetChangeTitleCallback(
        view: ULView,
        callback: ULChangeTitleCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetChangeURLCallback(
        view: ULView,
        callback: ULChangeURLCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetChangeTooltipCallback(
        view: ULView,
        callback: ULChangeTooltipCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetChangeCursorCallback(
        view: ULView,
        callback: ULChangeCursorCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetAddConsoleMessageCallback(
        view: ULView,
        callback: ULAddConsoleMessageCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetCreateChildViewCallback(
        view: ULView,
        callback: ULCreateChildViewCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetCreateInspectorViewCallback(
        view: ULView,
        callback: ULCreateInspectorViewCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetBeginLoadingCallback(
        view: ULView,
        callback: ULBeginLoadingCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetFinishLoadingCallback(
        view: ULView,
        callback: ULFinishLoadingCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetFailLoadingCallback(
        view: ULView,
        callback: ULFailLoadingCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetWindowObjectReadyCallback(
        view: ULView,
        callback: ULWindowObjectReadyCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetDOMReadyCallback(
        view: ULView,
        callback: ULDOMReadyCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetUpdateHistoryCallback(
        view: ULView,
        callback: ULUpdateHistoryCallback,
        user_data: *mut c_void,
    );
    pub fn ulViewSetNeedsPaint(view: ULView, needs_paint: bool);
    pub fn ulViewGetNeedsPaint(view: ULView) -> bool;
    pub fn ulViewCreateLocalInspectorView(view: ULView);
}

// Key event functions
unsafe extern "C" {
    pub fn ulCreateKeyEvent(
        type_: ULKeyEventType,
        modifiers: c_uint,
        virtual_key_code: c_int,
        native_key_code: c_int,
        text: ULString,
        unmodified_text: ULString,
        is_keypad: bool,
        is_auto_repeat: bool,
        is_system_key: bool,
    ) -> ULKeyEvent;
    pub fn ulDestroyKeyEvent(evt: ULKeyEvent);
}

// Mouse event functions
unsafe extern "C" {
    pub fn ulCreateMouseEvent(
        type_: ULMouseEventType,
        x: c_int,
        y: c_int,
        button: ULMouseButton,
    ) -> ULMouseEvent;
    pub fn ulDestroyMouseEvent(evt: ULMouseEvent);
}

// Scroll event functions
unsafe extern "C" {
    pub fn ulCreateScrollEvent(
        type_: ULScrollEventType,
        delta_x: c_int,
        delta_y: c_int,
    ) -> ULScrollEvent;
    pub fn ulDestroyScrollEvent(evt: ULScrollEvent);
}

// Gamepad event functions
unsafe extern "C" {
    pub fn ulCreateGamepadEvent(index: c_uint, type_: ULGamepadEventType) -> ULGamepadEvent;
    pub fn ulDestroyGamepadEvent(evt: ULGamepadEvent);
    pub fn ulCreateGamepadAxisEvent(
        index: c_uint,
        axis_index: c_uint,
        value: c_double,
    ) -> ULGamepadAxisEvent;
    pub fn ulDestroyGamepadAxisEvent(evt: ULGamepadAxisEvent);
    pub fn ulCreateGamepadButtonEvent(
        index: c_uint,
        button_index: c_uint,
        value: c_double,
    ) -> ULGamepadButtonEvent;
    pub fn ulDestroyGamepadButtonEvent(evt: ULGamepadButtonEvent);
}

// Font file functions
unsafe extern "C" {
    pub fn ulFontFileCreateFromFilePath(file_path: ULString) -> ULFontFile;
    pub fn ulFontFileCreateFromBuffer(buffer: ULBuffer) -> ULFontFile;
    pub fn ulDestroyFontFile(font_file: ULFontFile);
}

// Image source functions
unsafe extern "C" {
    pub fn ulCreateImageSourceFromTexture(
        width: c_uint,
        height: c_uint,
        texture_id: c_uint,
        texture_uv: ULRect,
        bitmap: ULBitmap,
    ) -> ULImageSource;
    pub fn ulCreateImageSourceFromBitmap(bitmap: ULBitmap) -> ULImageSource;
    pub fn ulDestroyImageSource(image_source: ULImageSource);
    pub fn ulImageSourceInvalidate(image_source: ULImageSource);
    pub fn ulImageSourceProviderAddImageSource(id: ULString, image_source: ULImageSource);
    pub fn ulImageSourceProviderRemoveImageSource(id: ULString);
}
