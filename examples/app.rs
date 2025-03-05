use ul::app_core::{
    app::App,
    overlay::Overlay,
    platform,
    Settings,
    window::{Window, WindowFlags},
};
use ul::{
    config::Config,
    view_config::ViewConfig,
};
use std::time::Instant;
use std::rc::Rc;

struct Browser {
    window: Window,
    overlay: Overlay,
    last_time: Instant,
    is_loading: bool,
}

impl Browser {
    fn new() -> Self {
        // Initialize platform
        platform::enable_default_logger("./ultralight.log");
        platform::enable_platform_file_system("./resources/");
        platform::enable_platform_font_loader();

        // Create an App instance
        let app = App::new(&Settings::default(), &Config::default())
            .expect("Failed to create app");

        // Create a window
        let window = Window::new(
            &app.main_monitor().expect("Failed to get main monitor"),
            800,
            600,
            false,
            WindowFlags::TITLED | WindowFlags::RESIZABLE,
        ).expect("Failed to create window");

        // Create a view config
        let mut view_config = ViewConfig::new();
        view_config
            .set_is_accelerated(true)
            .set_is_transparent(false)
            .set_initial_device_scale(window.scale())
            .set_initial_focus(true);

        // Create an overlay (which contains a view)
        let overlay = Overlay::new(
            &window,
            window.width(),
            window.height(),
            0,
            0,
        ).expect("Failed to create overlay");

        // Initialize the browser
        Browser {
            window,
            overlay,
            last_time: Instant::now(),
            is_loading: false,
        }
    }

    fn load_url(&self, url: &str) {
        let view = self.overlay.view().expect("Failed to get view");
        view.load_url(url);
    }

    fn resize(&self) {
        let width = self.window.width();
        let height = self.window.height();
        self.overlay.resize(width, height);
    }
    
    // Set up callbacks using closures rather than implementing the traits
    fn setup_callbacks(&self) {
        // Store weak references or use other methods to avoid callback issues
        let window_ref = &self.window;
        
        // Set up resize callback
        self.window.set_resize_callback(move |width, height| {
            println!("Window resized to {}x{}", width, height);
            // We can't easily call self.resize() here, so we would need a different approach
        }).expect("Failed to set resize callback");
        
        // Set up other callbacks as needed
    }
}

// This is safe to send between threads
struct ThreadSafeUpdateCallback {
    last_time: Instant,
}

impl ThreadSafeUpdateCallback {
    fn new() -> Self {
        Self {
            last_time: Instant::now(),
        }
    }
    
    fn on_update(&self) {
        // Calculate delta time
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_time).as_secs_f32();
        
        // Print delta time or perform updates
        println!("Delta time: {:.4}s", delta_time);
    }
}

fn main() {
    // Initialize platform
    platform::enable_default_logger("./ultralight.log");
    platform::enable_platform_file_system("./resources/");
    platform::enable_platform_font_loader();

    // Create settings
    let settings = Settings::default();
    let config = Config::default();
    
    // Create application
    let app = App::new(&settings, &config).expect("Failed to create app");
    
    // Get the main monitor
    let monitor = app.main_monitor().expect("Failed to get main monitor");
    
    // Create a window
    let window = Window::new(
        &monitor,
        800,
        600,
        false,
        WindowFlags::TITLED | WindowFlags::RESIZABLE,
    ).expect("Failed to create window");
    
    window.set_title("Ultralight Example");
    
    // Create overlay with Rc for shared ownership
    let overlay = Rc::new(Overlay::new(
        &window,
        window.width(),
        window.height(),
        0,
        0
    ).expect("Failed to create overlay"));
    
    // Clone the Rc for the closure
    let overlay_for_callback = overlay.clone();
    
    window.set_resize_callback(move |width, height| {
        println!("Window resized to {}x{}", width, height);
        overlay_for_callback.resize(width, height);
    }).expect("Failed to set resize callback");
    
    // Register a thread-safe update callback
    let update_callback = ThreadSafeUpdateCallback::new();
    app.set_update_callback(move || {
        update_callback.on_update();
    }).expect("Failed to set update callback");
    
    // Set up window callbacks with closures
    window.set_close_callback(|| {
        println!("Window closing!");
    }).expect("Failed to set close callback");
    
    // Show the window and overlay
    window.show();
    overlay.show();
    
    // Run application
    println!("Application starting...");
    app.run();
    println!("Application exited.");
}
