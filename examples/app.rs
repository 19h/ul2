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
    app: App,
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
            WindowFlags::BORDERLESS | WindowFlags::RESIZABLE,
        ).expect("Failed to create window");
        
        window.set_title("Ultralight Example").expect("Failed to set window title");

        // Create a view config
        let mut view_config = ViewConfig::new();
        view_config
            .set_is_accelerated(true)
            .set_is_transparent(true)
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
            app,
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
    
    fn setup_callbacks(&self) {
        // Clone overlay for the resize callback
        let overlay = self.overlay.clone();
        
        // Set up resize callback
        self.window.set_resize_callback(move |width, height| {
            println!("Window resized to {}x{}", width, height);
            overlay.resize(width, height);
        }).expect("Failed to set resize callback");
        
        // Set up close callback
        self.window.set_close_callback(|| {
            println!("Window closing!");
        }).expect("Failed to set close callback");
        
        // Set up update callback - clone app first to avoid borrowing self
        let update_callback = ThreadSafeUpdateCallback::new();
        self.app.set_update_callback(move || {
            update_callback.on_update(); // Use the already cloned app
        }).expect("Failed to set update callback");
    }
    
    fn show(&self) {
        self.window.show();
        self.overlay.show();
    }
    
    fn run(&self) {
        println!("Application starting...");
        self.app.run();
        println!("Application exited.");
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
        println!("Delta time: {:.4}s", delta_time);
    }
}

fn main() {
    // Create browser instance
    let browser = Browser::new();
    
    // Set up callbacks
    browser.setup_callbacks();
    
    // Show the browser window
    browser.show();
    
    // Load a URL (optional)
    browser.load_url("https://ultralig.ht");
    
    // Run the application
    browser.run();
}
