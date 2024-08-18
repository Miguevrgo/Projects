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
    pub fn init() {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, Hide).unwrap();
        execute!(stdout, SetTitle("Oxide")).unwrap();
        enable_raw_mode().unwrap();
    }

    pub fn restore() {
        let mut stdout = stdout();
        execute!(stdout, Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }

    pub fn clear_screen() {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    }

    pub fn render_text(text: &str) {
        let mut stdout = stdout();
        execute!(stdout, Print(text)).unwrap();
    }

    pub fn read_key() -> Option<KeyCode> {
        if let Event::Key(event) = read().unwrap() {
            Some(event.code)
        } else {
            None
        }
    }

    pub fn move_cursor_to(x: u16, y: u16) {
        let mut stdout = stdout();
        execute!(stdout, MoveTo(x, y)).unwrap();
    }
}
