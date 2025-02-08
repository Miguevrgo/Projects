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

    pub fn player_move(&self, pos_x: usize, pos_y: usize, new_x: usize, new_y: usize) -> bool {
        if self.turn % 2 == 0 {
            self.board.move_piece(pos_x, pos_y, new_x, new_y, true) // White
        } else {
            self.board.move_piece(pos_x, pos_y, new_x, new_y, false) // Black
        }
    }
}
