use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Select,
}

impl Direction {
    /// Returns Direction to move, select command or return to menu (None)
    /// reading input from user keyboard
    pub fn input_key() -> Option<Direction> {
        enable_raw_mode().unwrap();
        loop {
            if let Ok(Event::Key(event)) = event::read() {
                disable_raw_mode().unwrap();
                match event.code {
                    KeyCode::Char('h') | KeyCode::Left => return Some(Direction::Left),
                    KeyCode::Char('j') | KeyCode::Down => return Some(Direction::Down),
                    KeyCode::Char('k') | KeyCode::Up => return Some(Direction::Up),
                    KeyCode::Char('l') | KeyCode::Right => return Some(Direction::Right),
                    KeyCode::Enter => return Some(Direction::Select),
                    KeyCode::Esc => return None, // Pause game
                    _ => continue,
                }
            }
        }
    }
}
