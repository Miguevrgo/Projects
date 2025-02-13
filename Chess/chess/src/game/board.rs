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

    fn pawn_possible_moves(&self, pos_x: usize, pos_y: usize) -> Vec<(usize, usize)> {
        let mut possible_moves = Vec::new();
        if pos_y < 8 && self.board[pos_x][pos_y + 1] == Piece::EM {
            possible_moves.push((pos_x, pos_y + 1));
        }
        if pos_y == 1 && self.board[pos_x][pos_y + 2] == Piece::EM {
            possible_moves.push((pos_x, pos_y + 2));
        }
        // TODO: Same color?
        if pos_x > 0 && pos_y < 8 && self.board[pos_x - 1][pos_y + 1] != Piece::EM {
            possible_moves.push((pos_x - 1, pos_y + 1));
        }
        if pos_x < 8 && pos_y < 8 && self.board[pos_x + 1][pos_y + 1] != Piece::EM {
            possible_moves.push((pos_x + 1, pos_y + 1));
        }

        possible_moves
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
