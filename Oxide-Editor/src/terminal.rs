use std::io::stdout;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle,
    },
};

pub struct Terminal;

impl Terminal {
    /// Initializes the terminal for the editor, setting its title name to Oxide
    /// and enabling raw_mode
    pub fn init() {
        execute!(stdout(), EnterAlternateScreen, Hide).unwrap();
        execute!(stdout(), SetTitle("Oxide")).unwrap();
        enable_raw_mode().unwrap();
    }

    /// Restores the terminal to its original state
    pub fn restore() {
        execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }

    /// Clears the terminal screen and moves the cursor to (0,0)
    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    }

    /// Renders the given text to the terminal without any formatting
    /// or cursor movement
    ///
    /// # Arguments
    ///
    /// * `text`- The text to render to the terminal with type
    ///
    pub fn render_text(text: &str) {
        execute!(stdout(), Print(text)).unwrap();
    }

    pub fn read_key() -> Option<KeyCode> {
        if let Event::Key(event) = read().unwrap() {
            Some(event.code)
        } else {
            None
        }
    }

    pub fn show_cursor() {
        execute!(stdout(), Show).unwrap();
    }

    pub fn move_cursor_to(x: u16, y: u16) {
        execute!(stdout(), MoveTo(x, y)).unwrap();
    }
}
