//! # Ultralight-rs
//! 
//! Safe Rust bindings for the Ultralight web browser engine.
//! 
//! Ultralight is a lightweight, cross-platform browser engine designed for embedding in applications.
//! These bindings provide a safe, idiomatic Rust interface to the Ultralight C API.
//! 
//! ## Example usage
//! 
//! ```rust
//! use ultralight_rs::*;
//! 
//! fn main() {
//!     // Initialize the platform with default handlers
//!     Platform::enable_default_logger();
//!     Platform::enable_platform_file_system("./resources/");
//!     Platform::enable_platform_font_loader();
//!     
//!     // Create configuration
//!     let config = Config::new();
//!     
//!     // Create the renderer
//!     let renderer = Renderer::new(config);
//!     
//!     // Create view configuration
//!     let view_config = ViewConfig::new();
//!     
//!     // Create a view
//!     let view = View::new(&renderer, 800, 600, &view_config, None);
//!     
//!     // Load content
//!     view.load_url("https://example.com");
//!     
//!     // Main loop
//!     loop {
//!         // Update timers and dispatch callbacks
//!         renderer.update();
//!         
//!         // Refresh display and render views
//!         renderer.refresh_display(0);
//!         renderer.render();
//!         
//!         // Access and display the bitmap surface
//!         let surface = view.surface().unwrap();
//!         let bitmap_surface = surface.as_bitmap_surface().unwrap();
//!         let bitmap = bitmap_surface.bitmap();
//!         
//!         // Your application logic to display the bitmap...
//!     }
//! }
//! ```

pub mod ffi;
pub mod error;
pub mod string;
pub mod bitmap;
pub mod buffer;
pub mod config;
pub mod view_config;
pub mod platform;
pub mod renderer;
pub mod session;
pub mod surface;
pub mod view;
pub mod geometry;
pub mod events;
pub mod image_source;

// Re-exports
pub use error::Error;
pub use string::String;
pub use bitmap::{Bitmap, BitmapFormat};
pub use buffer::Buffer;
pub use config::Config;
pub use view_config::ViewConfig;
pub use platform::Platform;
pub use renderer::Renderer;
pub use session::Session;
pub use surface::{Surface, BitmapSurface, SurfaceDefinition};
pub use view::View;
pub use geometry::{Rect, IntRect};
pub use events::{
    KeyEvent, KeyEventType, MouseEvent, MouseEventType, MouseButton,
    ScrollEvent, ScrollEventType, GamepadEvent, GamepadEventType,
    GamepadAxisEvent, GamepadButtonEvent
};
pub use image_source::ImageSource;

// Constants and enums
pub use ffi::{
    ULCursor as Cursor,
    ULFaceWinding as FaceWinding,
    ULFontHinting as FontHinting,
    ULMessageSource as MessageSource,
    ULMessageLevel as MessageLevel,
};