extern crate gtk;
use gtk::*;
use std::process;
fn main(){
    // Initialize GTK
    if gtk::init().is_err() {
        eprintln!("Failed to init");
        process::exit(1);
    }

    pub struct App {
        pub window: Window,
        pub header: Header,
    }
    
    pub struct Header {
        pub container: HeaderBar
    }

    impl App {
        fn new() -> App {
            // Create new top level window
            let window = Window::new(WindowType::Toplevel);
            // Create the headbar and its content
            let header = Header::new();
    
            // Set the headbar as the title bar widget
            window.set_titlebar(&header.container);
            // Set the title of the window
            window.set_title("App Name");
            // Set the window manager class
            window.set_wmclass("app-name", "App Name");
            // The icon the app will display 
            Window::set_default_icon_name("iconname");
    
            // Programs what to do when the exit button is used 
            window.connect_delete_event(move |_, _| {
                main_quit();
                Inhibit(false)
            });
    
            // Return our main application state
            App {window, header}
        }
    }

    impl Header {
        fn new() -> Header {
            // Creates the main headerbar widget
            let container = HeaderBar::new();

            // Sets the text to display the title section of the header bar
            container.set_title("App Name");
            // Enable the window controls within the headerbar
            container.set_show_close_button(true);

            // Returns the header and all of its state
            Header { container }
        }
    }
}



