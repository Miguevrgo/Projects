use std::error::Error;
use std::ptr::null;
use x11::xlib::{Display, Window, XCloseDisplay, XDefaultRootWindow, XOpenDisplay};

pub struct WindowManager {
    display: *mut Display,
    root: Window,
}

impl WindowManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let display = unsafe { XOpenDisplay(null()) };
        if display.is_null() {
            return Err(
                "Unable to open XOpenDisplay (Maybe there is another instance running?)".into(),
            );
        }

        let root = unsafe { XDefaultRootWindow(display) };
        Ok(WindowManager { display, root })
    }

    pub fn close(&mut self) {
        if !self.display.is_null() {
            unsafe {
                XCloseDisplay(self.display);
            }
        }
    }
}
impl Drop for WindowManager {
    fn drop(&mut self) {
        self.close()
    }
}

fn main() {
    let window_manager = WindowManager::new();
    if let Err(e) = window_manager {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
