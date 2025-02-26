use crate::game::piece::Piece;

pub struct Move {
    piece: Piece,
    prev_x: usize,
    prev_y: usize,
    new_x: usize,
    new_y: usize,
}

impl Move {
    pub fn from(piece: Piece, prev_x: usize, prev_y: usize, new_x: usize, new_y: usize) -> Self {
        Move {
            piece,
            prev_x,
            prev_y,
            new_x,
            new_y,
        }
    }
}
