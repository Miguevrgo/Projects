use crate::game::piece::*;

pub struct Move {
    piece: Piece,
    prev_row: usize,
    prev_col: usize,
    new_row: usize,
    new_col: usize,
}

impl Move {
    pub fn from(
        piece: Piece,
        prev_row: usize,
        prev_col: usize,
        new_row: usize,
        new_col: usize,
    ) -> Self {
        Move {
            piece,
            prev_row,
            prev_col,
            new_row,
            new_col,
        }
    }

    pub fn is_en_passant(&self) -> bool {
        if self.piece == Piece::Pawn && self.prev_row.abs_diff(self.new_row) == 2 {
            return true;
        }

        false
    }
}
