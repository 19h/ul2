use crate::ul::error::Error;
use crate::ul::events::{KeyEvent, MouseEvent, ScrollEvent};
use crate::ul::ffi::{
    JSContextRef, ULCursor, ULIntRect, ULMessageLevel, ULMessageSource, ULRenderTarget, ULString,
    ULView, ulCreateView, ulDestroyView, ulViewCanGoBack, ulViewCanGoForward,
    ulViewCreateLocalInspectorView, ulViewEvaluateScript, ulViewFireKeyEvent, ulViewFireMouseEvent,
    ulViewFireScrollEvent, ulViewFocus, ulViewGetDeviceScale, ulViewGetDisplayId, ulViewGetHeight,
    ulViewGetNeedsPaint, ulViewGetRenderTarget, ulViewGetSurface, ulViewGetTitle, ulViewGetURL,
    ulViewGetWidth, ulViewGoBack, ulViewGoForward, ulViewGoToHistoryOffset, ulViewHasFocus,
    ulViewHasInputFocus, ulViewIsAccelerated, ulViewIsLoading, ulViewIsTransparent, ulViewLoadHTML,
    ulViewLoadURL, ulViewLockJSContext, ulViewReload, ulViewResize,
    ulViewSetAddConsoleMessageCallback, ulViewSetBeginLoadingCallback,
    ulViewSetChangeCursorCallback, ulViewSetChangeTitleCallback, ulViewSetChangeTooltipCallback,
    ulViewSetChangeURLCallback, ulViewSetCreateChildViewCallback,
    ulViewSetCreateInspectorViewCallback, ulViewSetDOMReadyCallback, ulViewSetDeviceScale,
    ulViewSetDisplayId, ulViewSetFailLoadingCallback, ulViewSetFinishLoadingCallback,
    ulViewSetNeedsPaint, ulViewSetUpdateHistoryCallback, ulViewSetWindowObjectReadyCallback,
    ulViewStop, ulViewUnfocus, ulViewUnlockJSContext,
};
use crate::ul::geometry::{IntRect, Rect};
use crate::ul::renderer::Renderer;
use crate::ul::session::Session;
use crate::ul::string::String;
use crate::ul::surface::Surface;
use crate::ul::view_config::ViewConfig;
use std::os::raw::{c_int, c_uint, c_ulonglong, c_void};
use std::ptr;

pub use crate::ul::ffi::{
    ULCursor as Cursor, ULMessageLevel as MessageLevel, ULMessageSource as MessageSource,
};

/// A render target for GPU-accelerated views.
#[derive(Debug, Clone, Copy)]
pub struct RenderTarget {
    pub is_empty: bool,
    pub width: u32,
    pub height: u32,
    pub texture_id: u32,
    pub texture_width: u32,
    pub texture_height: u32,
    pub texture_format: crate::ul::bitmap::BitmapFormat,
    pub uv_coords: Rect,
    pub render_buffer_id: u32,
}

impl RenderTarget {
    /// Create a render target from a raw ULRenderTarget.
    pub fn from_raw(raw: ULRenderTarget) -> Self {
        Self {
            is_empty: raw.is_empty,
            width: raw.width,
            height: raw.height,
            texture_id: raw.texture_id,
            texture_width: raw.texture_width,
            texture_height: raw.texture_height,
            texture_format: raw.texture_format,
            uv_coords: Rect::from_raw(raw.uv_coords),
            render_buffer_id: raw.render_buffer_id,
        }
    }
}

/// Callback for when the page title changes.
pub trait ChangeTitleCallback: Send {
    fn on_change_title(&self, view: &View, title: &str);
}

/// Callback for when the page URL changes.
pub trait ChangeURLCallback: Send {
    fn on_change_url(&self, view: &View, url: &str);
}

/// Callback for when the tooltip changes.
pub trait ChangeTooltipCallback: Send {
    fn on_change_tooltip(&self, view: &View, tooltip: &str);
}

/// Callback for when the cursor changes.
pub trait ChangeCursorCallback: Send {
    fn on_change_cursor(&self, view: &View, cursor: Cursor);
}

/// Callback for when a message is added to the console.
pub trait AddConsoleMessageCallback: Send {
    fn on_add_console_message(
        &self,
        view: &View,
        source: MessageSource,
        level: MessageLevel,
        message: &str,
        line_number: u32,
        column_number: u32,
        source_id: &str,
    );
}

/// Callback for when a child view needs to be created.
pub trait CreateChildViewCallback: Send {
    fn on_create_child_view(
        &self,
        view: &View,
        opener_url: &str,
        target_url: &str,
        is_popup: bool,
        popup_rect: IntRect,
    ) -> Option<View>;
}

/// Callback for when an inspector view needs to be created.
pub trait CreateInspectorViewCallback: Send {
    fn on_create_inspector_view(
        &self,
        view: &View,
        is_local: bool,
        inspected_url: &str,
    ) -> Option<View>;
}

/// Callback for when a page begins loading.
pub trait BeginLoadingCallback: Send {
    fn on_begin_loading(&self, view: &View, frame_id: u64, is_main_frame: bool, url: &str);
}

/// Callback for when a page finishes loading.
pub trait FinishLoadingCallback: Send {
    fn on_finish_loading(&self, view: &View, frame_id: u64, is_main_frame: bool, url: &str);
}

/// Callback for when a page fails to load.
pub trait FailLoadingCallback: Send {
    fn on_fail_loading(
        &self,
        view: &View,
        frame_id: u64,
        is_main_frame: bool,
        url: &str,
        description: &str,
        error_domain: &str,
        error_code: i32,
    );
}

/// Callback for when the JavaScript window object is reset.
pub trait WindowObjectReadyCallback: Send {
    fn on_window_object_ready(&self, view: &View, frame_id: u64, is_main_frame: bool, url: &str);
}

/// Callback for when the DOM is ready.
pub trait DOMReadyCallback: Send {
    fn on_dom_ready(&self, view: &View, frame_id: u64, is_main_frame: bool, url: &str);
}

/// Callback for when the history is updated.
pub trait UpdateHistoryCallback: Send {
    fn on_update_history(&self, view: &View);
}

// Callback wrappers for the C API
extern "C" fn change_title_callback<T: ChangeTitleCallback>(
    user_data: *mut c_void,
    caller: ULView,
    title: ULString,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let title_str = String::from_raw(title, false);

        callback.on_change_title(&view, &title_str);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn change_url_callback<T: ChangeURLCallback>(
    user_data: *mut c_void,
    caller: ULView,
    url: ULString,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let url_str = String::from_raw(url, false);

        callback.on_change_url(&view, &url_str);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn change_tooltip_callback<T: ChangeTooltipCallback>(
    user_data: *mut c_void,
    caller: ULView,
    tooltip: ULString,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let tooltip_str = String::from_raw(tooltip, false);

        callback.on_change_tooltip(&view, &tooltip_str);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn change_cursor_callback<T: ChangeCursorCallback>(
    user_data: *mut c_void,
    caller: ULView,
    cursor: ULCursor,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);

        callback.on_change_cursor(&view, cursor);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn add_console_message_callback<T: AddConsoleMessageCallback>(
    user_data: *mut c_void,
    caller: ULView,
    source: ULMessageSource,
    level: ULMessageLevel,
    message: ULString,
    line_number: c_uint,
    column_number: c_uint,
    source_id: ULString,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let message_str = String::from_raw(message, false);
        let source_id_str = String::from_raw(source_id, false);

        callback.on_add_console_message(
            &view,
            source,
            level,
            &message_str,
            line_number,
            column_number,
            &source_id_str,
        );

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn create_child_view_callback<T: CreateChildViewCallback>(
    user_data: *mut c_void,
    caller: ULView,
    opener_url: ULString,
    target_url: ULString,
    is_popup: bool,
    popup_rect: ULIntRect,
) -> ULView {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let opener_url_str = String::from_raw(opener_url, false);
        let target_url_str = String::from_raw(target_url, false);
        let popup_rect_rust = IntRect::from_raw(popup_rect);

        let result = callback.on_create_child_view(
            &view,
            &opener_url_str,
            &target_url_str,
            is_popup,
            popup_rect_rust,
        );

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);

        match result {
            Some(child_view) => {
                let raw = child_view.raw;
                // Prevent drop of child_view to avoid deallocation
                std::mem::forget(child_view);
                raw
            }
            None => ptr::null_mut(),
        }
    }
}

extern "C" fn create_inspector_view_callback<T: CreateInspectorViewCallback>(
    user_data: *mut c_void,
    caller: ULView,
    is_local: bool,
    inspected_url: ULString,
) -> ULView {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let inspected_url_str = String::from_raw(inspected_url, false);

        let result = callback.on_create_inspector_view(&view, is_local, &inspected_url_str);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);

        match result {
            Some(inspector_view) => {
                let raw = inspector_view.raw;
                // Prevent drop of inspector_view to avoid deallocation
                std::mem::forget(inspector_view);
                raw
            }
            None => ptr::null_mut(),
        }
    }
}

extern "C" fn begin_loading_callback<T: BeginLoadingCallback>(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let url_str = String::from_raw(url, false);

        callback.on_begin_loading(&view, frame_id, is_main_frame, &url_str);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn finish_loading_callback<T: FinishLoadingCallback>(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let url_str = String::from_raw(url, false);

        callback.on_finish_loading(&view, frame_id, is_main_frame, &url_str);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn fail_loading_callback<T: FailLoadingCallback>(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
    description: ULString,
    error_domain: ULString,
    error_code: c_int,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let url_str = String::from_raw(url, false);
        let description_str = String::from_raw(description, false);
        let error_domain_str = String::from_raw(error_domain, false);

        callback.on_fail_loading(
            &view,
            frame_id,
            is_main_frame,
            &url_str,
            &description_str,
            &error_domain_str,
            error_code,
        );

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn window_object_ready_callback<T: WindowObjectReadyCallback>(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let url_str = String::from_raw(url, false);

        callback.on_window_object_ready(&view, frame_id, is_main_frame, &url_str);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn dom_ready_callback<T: DOMReadyCallback>(
    user_data: *mut c_void,
    caller: ULView,
    frame_id: c_ulonglong,
    is_main_frame: bool,
    url: ULString,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);
        let url_str = String::from_raw(url, false);

        callback.on_dom_ready(&view, frame_id, is_main_frame, &url_str);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

extern "C" fn update_history_callback<T: UpdateHistoryCallback>(
    user_data: *mut c_void,
    caller: ULView,
) {
    unsafe {
        let callback = &*(user_data as *const T);
        let view = View::from_raw(caller);

        callback.on_update_history(&view);

        // Prevent drop of view to avoid deallocation
        std::mem::forget(view);
    }
}

/// A structure that holds callback data and keeps it alive.
struct CallbackData<T: ?Sized> {
    data: Box<T>,
}

impl<T> CallbackData<T> {
    fn new(data: T) -> *mut c_void {
        let data = Box::new(CallbackData {
            data: Box::new(data),
        });
        Box::into_raw(data) as *mut c_void
    }

    unsafe fn drop(ptr: *mut c_void) {
        unsafe {
            if !ptr.is_null() {
                let _ = Box::from_raw(ptr as *mut CallbackData<T>);
            }
        }
    }
}

/// A structure that manages a locked JavaScript context.
pub struct LockedJSContext<'a> {
    view: &'a View,
    context: JSContextRef,
}

impl LockedJSContext<'_> {
    /// Get the raw JavaScriptCore context.
    pub fn raw(&self) -> JSContextRef {
        self.context
    }
}

impl Drop for LockedJSContext<'_> {
    fn drop(&mut self) {
        unsafe {
            ulViewUnlockJSContext(self.view.raw);
        }
    }
}

/// A safe wrapper around Ultralight's ULView type.
pub struct View {
    raw: ULView,
}

impl View {
    /// Create a new view.
    pub fn new(
        renderer: &Renderer,
        width: u32,
        height: u32,
        config: &ViewConfig,
        session: Option<&Session>,
    ) -> Self {
        unsafe {
            let session_ptr = match session {
                Some(s) => s.raw(),
                None => ptr::null_mut(),
            };

            let raw = ulCreateView(renderer.raw(), width, height, config.raw(), session_ptr);
            Self { raw }
        }
    }

    /// Create a view from a raw ULView pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid ULView created by the Ultralight API.
    pub unsafe fn from_raw(raw: ULView) -> Self {
        Self { raw }
    }

    /// Get a reference to the raw ULView.
    pub fn raw(&self) -> ULView {
        self.raw
    }

    /// Get the current URL.
    pub fn url(&self) -> String {
        unsafe {
            let url = ulViewGetURL(self.raw);
            String::from_raw(url, false)
        }
    }

    /// Get the current title.
    pub fn title(&self) -> String {
        unsafe {
            let title = ulViewGetTitle(self.raw);
            String::from_raw(title, false)
        }
    }

    /// Get the width in pixels.
    pub fn width(&self) -> u32 {
        unsafe { ulViewGetWidth(self.raw) }
    }

    /// Get the height in pixels.
    pub fn height(&self) -> u32 {
        unsafe { ulViewGetHeight(self.raw) }
    }

    /// Get the display ID.
    pub fn display_id(&self) -> u32 {
        unsafe { ulViewGetDisplayId(self.raw) }
    }

    /// Set the display ID.
    pub fn set_display_id(&self, display_id: u32) {
        unsafe {
            ulViewSetDisplayId(self.raw, display_id);
        }
    }

    /// Get the device scale.
    pub fn device_scale(&self) -> f64 {
        unsafe { ulViewGetDeviceScale(self.raw) }
    }

    /// Set the device scale.
    pub fn set_device_scale(&self, scale: f64) {
        unsafe {
            ulViewSetDeviceScale(self.raw, scale);
        }
    }

    /// Check if the view is GPU-accelerated.
    pub fn is_accelerated(&self) -> bool {
        unsafe { ulViewIsAccelerated(self.raw) }
    }

    /// Check if the view supports transparent backgrounds.
    pub fn is_transparent(&self) -> bool {
        unsafe { ulViewIsTransparent(self.raw) }
    }

    /// Check if the main frame is currently loading.
    pub fn is_loading(&self) -> bool {
        unsafe { ulViewIsLoading(self.raw) }
    }

    /// Get the render target (for GPU-accelerated views).
    pub fn render_target(&self) -> RenderTarget {
        unsafe {
            let target = ulViewGetRenderTarget(self.raw);
            RenderTarget::from_raw(target)
        }
    }

    /// Get the surface (for CPU-rendered views).
    pub fn surface(&self) -> Option<Surface> {
        unsafe {
            let surface = ulViewGetSurface(self.raw);
            if surface.is_null() {
                None
            } else {
                Some(Surface::from_raw(surface))
            }
        }
    }

    /// Load raw HTML.
    pub fn load_html(&self, html: &str) {
        let html_str = String::from_str(html);
        unsafe {
            ulViewLoadHTML(self.raw, html_str.raw());
        }
    }

    /// Load a URL.
    pub fn load_url(&self, url: &str) {
        let url_str = String::from_str(url);
        unsafe {
            ulViewLoadURL(self.raw, url_str.raw());
        }
    }

    /// Resize the view.
    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            ulViewResize(self.raw, width, height);
        }
    }

    /// Lock the JavaScript context.
    pub fn lock_js_context(&self) -> LockedJSContext {
        unsafe {
            let context = ulViewLockJSContext(self.raw);
            LockedJSContext {
                view: self,
                context,
            }
        }
    }

    /// Evaluate JavaScript.
    pub fn evaluate_script(&self, js: &str) -> Result<String, Error> {
        let js_str = String::from_str(js);
        let mut exception: ULString = ptr::null_mut();

        unsafe {
            let result = ulViewEvaluateScript(self.raw, js_str.raw(), &mut exception);

            if !exception.is_null() {
                let exception_str = String::from_raw(exception, false);
                return Err(Error::JavaScriptError(exception_str.to_string()));
            }

            Ok(String::from_raw(result, false))
        }
    }

    /// Check if can navigate backwards in history.
    pub fn can_go_back(&self) -> bool {
        unsafe { ulViewCanGoBack(self.raw) }
    }

    /// Check if can navigate forwards in history.
    pub fn can_go_forward(&self) -> bool {
        unsafe { ulViewCanGoForward(self.raw) }
    }

    /// Navigate backwards in history.
    pub fn go_back(&self) {
        unsafe {
            ulViewGoBack(self.raw);
        }
    }

    /// Navigate forwards in history.
    pub fn go_forward(&self) {
        unsafe {
            ulViewGoForward(self.raw);
        }
    }

    /// Navigate to an arbitrary offset in history.
    pub fn go_to_history_offset(&self, offset: i32) {
        unsafe {
            ulViewGoToHistoryOffset(self.raw, offset);
        }
    }

    /// Reload the current page.
    pub fn reload(&self) {
        unsafe {
            ulViewReload(self.raw);
        }
    }

    /// Stop all page loads.
    pub fn stop(&self) {
        unsafe {
            ulViewStop(self.raw);
        }
    }

    /// Give focus to the view.
    pub fn focus(&self) {
        unsafe {
            ulViewFocus(self.raw);
        }
    }

    /// Remove focus from the view.
    pub fn unfocus(&self) {
        unsafe {
            ulViewUnfocus(self.raw);
        }
    }

    /// Check if the view has focus.
    pub fn has_focus(&self) -> bool {
        unsafe { ulViewHasFocus(self.raw) }
    }

    /// Check if the view has an input element with visible keyboard focus.
    pub fn has_input_focus(&self) -> bool {
        unsafe { ulViewHasInputFocus(self.raw) }
    }

    /// Fire a keyboard event.
    pub fn fire_key_event(&self, event: &KeyEvent) {
        unsafe {
            ulViewFireKeyEvent(self.raw, event.raw());
        }
    }

    /// Fire a mouse event.
    pub fn fire_mouse_event(&self, event: &MouseEvent) {
        unsafe {
            ulViewFireMouseEvent(self.raw, event.raw());
        }
    }

    /// Fire a scroll event.
    pub fn fire_scroll_event(&self, event: &ScrollEvent) {
        unsafe {
            ulViewFireScrollEvent(self.raw, event.raw());
        }
    }

    /// Set callback for when the page title changes.
    pub fn set_change_title_callback<T: 'static + ChangeTitleCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetChangeTitleCallback(
                self.raw,
                std::mem::transmute(
                    change_title_callback::<T> as extern "C" fn(*mut c_void, *mut _, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the page URL changes.
    pub fn set_change_url_callback<T: 'static + ChangeURLCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetChangeURLCallback(
                self.raw,
                std::mem::transmute(
                    change_url_callback::<T> as extern "C" fn(*mut c_void, *mut _, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the tooltip changes.
    pub fn set_change_tooltip_callback<T: 'static + ChangeTooltipCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetChangeTooltipCallback(
                self.raw,
                std::mem::transmute(
                    change_tooltip_callback::<T> as extern "C" fn(*mut c_void, *mut _, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the cursor changes.
    pub fn set_change_cursor_callback<T: 'static + ChangeCursorCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetChangeCursorCallback(
                self.raw,
                std::mem::transmute(
                    change_cursor_callback::<T> as extern "C" fn(*mut c_void, *mut _, _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when a message is added to the console.
    pub fn set_add_console_message_callback<T: 'static + AddConsoleMessageCallback>(
        &self,
        callback: T,
    ) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetAddConsoleMessageCallback(
                self.raw,
                std::mem::transmute(
                    add_console_message_callback::<T>
                        as extern "C" fn(*mut c_void, *mut _, _, _, *mut _, _, _, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the page wants to create a new View.
    pub fn set_create_child_view_callback<T: 'static + CreateChildViewCallback>(
        &self,
        callback: T,
    ) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetCreateChildViewCallback(
                self.raw,
                std::mem::transmute(
                    create_child_view_callback::<T>
                        as extern "C" fn(*mut c_void, *mut _, *mut _, *mut _, bool, _) -> *mut _,
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the page wants to create a new View to display the inspector in.
    pub fn set_create_inspector_view_callback<T: 'static + CreateInspectorViewCallback>(
        &self,
        callback: T,
    ) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetCreateInspectorViewCallback(
                self.raw,
                std::mem::transmute(
                    create_inspector_view_callback::<T>
                        as extern "C" fn(*mut c_void, *mut _, bool, *mut _) -> *mut _,
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the page begins loading a new URL into a frame.
    pub fn set_begin_loading_callback<T: 'static + BeginLoadingCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetBeginLoadingCallback(
                self.raw,
                std::mem::transmute(
                    begin_loading_callback::<T>
                        as extern "C" fn(*mut c_void, *mut _, u64, bool, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the page finishes loading a URL into a frame.
    pub fn set_finish_loading_callback<T: 'static + FinishLoadingCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetFinishLoadingCallback(
                self.raw,
                std::mem::transmute(
                    finish_loading_callback::<T>
                        as extern "C" fn(*mut c_void, *mut _, u64, bool, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when an error occurs while loading a URL into a frame.
    pub fn set_fail_loading_callback<T: 'static + FailLoadingCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetFailLoadingCallback(
                self.raw,
                std::mem::transmute(
                    fail_loading_callback::<T>
                        as extern "C" fn(
                            *mut c_void,
                            *mut _,
                            u64,
                            bool,
                            *mut _,
                            *mut _,
                            *mut _,
                            i32,
                        ),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the JavaScript window object is reset for a new page load.
    pub fn set_window_object_ready_callback<T: 'static + WindowObjectReadyCallback>(
        &self,
        callback: T,
    ) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetWindowObjectReadyCallback(
                self.raw,
                std::mem::transmute(
                    window_object_ready_callback::<T>
                        as extern "C" fn(*mut c_void, *mut _, u64, bool, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when all JavaScript has been parsed and the document is ready.
    pub fn set_dom_ready_callback<T: 'static + DOMReadyCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetDOMReadyCallback(
                self.raw,
                std::mem::transmute(
                    dom_ready_callback::<T>
                        as extern "C" fn(*mut c_void, *mut _, u64, bool, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set callback for when the history is modified.
    pub fn set_update_history_callback<T: 'static + UpdateHistoryCallback>(&self, callback: T) {
        unsafe {
            let user_data = CallbackData::new(callback);
            ulViewSetUpdateHistoryCallback(
                self.raw,
                std::mem::transmute(
                    update_history_callback::<T> as extern "C" fn(*mut c_void, *mut _),
                ),
                user_data,
            );
        }
    }

    /// Set whether the view should be repainted during the next render call.
    pub fn set_needs_paint(&self, needs_paint: bool) {
        unsafe {
            ulViewSetNeedsPaint(self.raw, needs_paint);
        }
    }

    /// Check if the view should be painted during the next render call.
    pub fn needs_paint(&self) -> bool {
        unsafe { ulViewGetNeedsPaint(self.raw) }
    }

    /// Create an Inspector View to inspect/debug this View locally.
    pub fn create_local_inspector_view(&self) {
        unsafe {
            ulViewCreateLocalInspectorView(self.raw);
        }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ulDestroyView(self.raw);
            }
        }
    }
}
