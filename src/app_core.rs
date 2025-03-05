//! # Ultralight AppCore
//! 
//! Safe Rust bindings for the Ultralight AppCore library, providing window management and UI capabilities.
//! 
//! AppCore is a convenient windowing system for desktop platforms built on top of the Ultralight renderer.
//! It automatically sets up the Renderer, creates a run loop, and handles all window creation, painting,
//! and platform-specific operations.
//! 
//! ## Example usage
//! 
//! ```rust
//! use ultralight_rs::*;
//! use ultralight_appcore::*;
//! 
//! fn main() {
//!     // Create app settings with default values
//!     let settings = Settings::new();
//!     
//!     // Create ultralight config with default values
//!     let config = Config::new();
//!     
//!     // Create the app with our settings and config
//!     let app = App::new(&settings, &config);
//!     
//!     // Create a window
//!     let window = Window::new(
//!         &app.main_monitor(),
//!         800,
//!         600,
//!         false,
//!         WindowFlags::TITLED | WindowFlags::RESIZABLE
//!     );
//!     
//!     // Set window title
//!     window.set_title("Ultralight AppCore Example");
//!     
//!     // Create an overlay (web view) that fills the window
//!     let overlay = Overlay::new(&window, 800, 600, 0, 0);
//!     
//!     // Get the view from the overlay and load a URL
//!     let view = overlay.view();
//!     view.load_url("https://example.com");
//!     
//!     // Run the app
//!     app.run();
//! }
//! ```

pub mod app;
pub mod error;
pub mod ffi;
pub mod monitor;
pub mod overlay;
pub mod platform;
pub mod settings;
pub mod window;

// Re-exports
pub use self::app::App;
pub use self::error::Error;
pub use self::monitor::Monitor;
pub use self::overlay::Overlay;
pub use self::settings::Settings;
pub use self::window::{Window, WindowFlags};