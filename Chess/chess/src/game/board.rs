use crate::game::piece::Piece;

/// A two dimensional array of pieces, starting easy first
pub struct Board {
    board: [[Piece; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        use Piece::*;
        Board {
            board: [
                [BR, BN, BB, BQ, BK, BB, BN, BR],
                [BP, BP, BP, BP, BP, BP, BP, BP],
                [EM, EM, EM, EM, EM, EM, EM, EM],
                [EM, EM, EM, EM, EM, EM, EM, EM],
                [EM, EM, EM, EM, EM, EM, EM, EM],
                [EM, EM, EM, EM, EM, EM, EM, EM],
                [WP, WP, WP, WP, WP, WP, WP, WP],
                [WR, WN, WB, WQ, WK, WB, WN, WR],
            ],
        }
    }

    /// Moves given piece to the new position, checking whether the move is valid or not,
    /// returning whether the move has been performed or it was invalid
    pub fn move_piece(
        &self,
        pos_x: usize,
        pos_y: usize,
        new_x: usize,
        new_y: usize,
        white: bool,
    ) -> bool {
        if self.board[pos_x][pos_y] == Piece::EM {
            false
        } else {
            match (&self.board[pos_x][pos_y], white) {
                (Piece::WP, true) | (Piece::BP, false) => todo!(),
                (Piece::WB, true) | (Piece::BB, false) => todo!(),
                (Piece::WN, true) | (Piece::BN, false) => todo!(),
                (Piece::WR, true) | (Piece::BR, false) => todo!(),
                (Piece::WQ, true) | (Piece::BQ, false) => todo!(),
                (Piece::WK, true) | (Piece::BK, false) => todo!(),
                _ => false,
            }
        }
    }

    pub fn draw(self) {
        for row in self.board {
            for square in row {
                print!("{square}");
            }
            println!()
        }
    }
}
