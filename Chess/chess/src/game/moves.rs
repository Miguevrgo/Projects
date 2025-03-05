use core::fmt;
use std::process::exit;

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
        if self.prev_row.abs_diff(self.new_row) == 2 {
            return true;
        }

        false
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letters = [
            ' ', // Empty
            'P', // Pawn
            'B', // Bishop
            'N', // Knight
            'R', // Rook
            'K', // King
            'Q', // Queen
        ];

        let piece_letter = letters[self.piece as usize];
        let col_letter = (self.new_col as u8 + b'a') as char; // a-h
        let row_number = (self.new_row + 1).to_string(); // 1-8

        write!(f, "{}{}{}", piece_letter, col_letter, row_number)
    }
}
