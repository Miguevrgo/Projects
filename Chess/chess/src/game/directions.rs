use std::io::{self, Read};

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Select,
}

impl Direction {
    /// Reads keyboard
    pub fn input_key() -> Direction {
        let mut stdin = io::stdin();
        let mut buffer = [0; 1];

        loop {
            stdin.read_exact(&mut buffer).expect("Unable to read input");
            match buffer[0] {
                b'h' => return Direction::Left,
                b'j' => return Direction::Down,
                b'k' => return Direction::Up,
                b'l' => return Direction::Right,
                b'\n' => return Direction::Select,
                _ => continue,
            }
        }
    }
}
