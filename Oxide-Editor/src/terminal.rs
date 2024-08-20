use std::io::stdout;

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle,
    },
};

pub struct Terminal;

#[allow(dead_code)]
impl Terminal {
    pub fn init() {
        execute!(stdout(), EnterAlternateScreen, Hide).unwrap();
        execute!(stdout(), SetTitle("Oxide")).unwrap();
        enable_raw_mode().unwrap();
    }

    pub fn restore() {
        execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }

    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    }

    pub fn render_multiline_text(text: &str, line_jumps: &Vec<usize>) {
        let mut text = text.to_string();
        for pos in line_jumps {
            text.insert(*pos, '\r');
        }

        execute!(stdout(), Print(text)).unwrap();
    }

    pub fn render_text(text: &str) {
        execute!(stdout(), Print(text)).unwrap();
    }

    pub fn render_new_line() {
        execute!(stdout(), MoveToNextLine(1)).unwrap();
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
