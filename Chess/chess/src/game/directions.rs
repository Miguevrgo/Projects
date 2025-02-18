use crossterm::{
    event::{self, Event, KeyCode},
    terminal::enable_raw_mode,
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
    pub fn input_key() -> Option<Direction> {
        enable_raw_mode().unwrap();
        if let Ok(Event::Key(event)) = event::read() {
            match event.code {
                KeyCode::Char('h') => Some(Direction::Left),
                KeyCode::Char('j') => Some(Direction::Down),
                KeyCode::Char('k') => Some(Direction::Up),
                KeyCode::Char('l') => Some(Direction::Right),
                KeyCode::Enter => Some(Direction::Select),
                KeyCode::Esc => std::process::exit(0),
                _ => None,
            }
        } else {
            None
        }
    }
}
