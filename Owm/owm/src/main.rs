use std::{io::Error, path::Display, ptr::null};

use x11::xlib::{Display, Window, XDefaultRootWindow, XOpenDisplay};

pub struct WindowManager {
    display: Display,
    root: Window,
}

impl WindowManager {
    fn new() -> Self {
        let display = unsafe { XOpenDisplay(null()) };
        WindowManager { display, root: 8 }
    }
}

fn main() {}
