use crate::game::board::Board;

pub struct Game {
    turn: u16, // Despite 5899 being the maximum number of moves possible
    board: Board,
    log: String,
}

impl Game {
    pub fn new() -> Self {
        Game {
            turn: 0,
            board: Board::new(),
            log: String::new(),
        }
    }
}
