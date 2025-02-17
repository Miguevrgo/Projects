use crossterm::event::{self, Event, KeyCode};

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Select,
}

impl Direction {
    pub fn input_key() -> Option<Direction> {
        if let Ok(Event::Key(event)) = event::read() {
            match event.code {
                KeyCode::Char('h') => Some(Direction::Left),
                KeyCode::Char('j') => Some(Direction::Down),
                KeyCode::Char('k') => Some(Direction::Up),
                KeyCode::Char('l') => Some(Direction::Right),
                KeyCode::Char(' ') => Some(Direction::Select),
                _ => None,
            }
        } else {
            None
        }
    }
}
