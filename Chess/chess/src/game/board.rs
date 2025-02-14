use crate::game::piece::*;

/// A board consists on 64 squares where each one contains one piece (where empty is considered)
/// a piece, as there are 7 possible pieces, each one with a color, we can fit each piece in 4
/// bits, as we want 8 pieces per row and 8 rows, we need an array of 8 u32 values
pub struct Board {
    board: [u32; 8],
}

impl Board {
    /// Creates a board with default pieces positions, white is on top because top represents
    /// the first position on the array which matches the 1 and 2 rows where white are always
    /// located
    pub fn new() -> Self {
        use Piece::*;
        Board {
            board: [
                Self::create_row(Color::White, &[Pawn; 8]),
                Self::create_row(
                    Color::White,
                    &[Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
                ),
                Self::create_row(Color::White, &[Empty; 8]),
                Self::create_row(Color::White, &[Empty; 8]),
                Self::create_row(Color::White, &[Empty; 8]),
                Self::create_row(Color::White, &[Empty; 8]),
                Self::create_row(
                    Color::Black,
                    &[Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
                ),
                Self::create_row(Color::Black, &[Pawn; 8]),
            ],
        }
    }

    /// Creates an u32 value representing a row of 8 pieces given their color which
    /// is stored in most significant bit of each piece and the piece which are stored
    /// in the same order as the one in pieces
    fn create_row(color: Color, pieces: &[Piece]) -> u32 {
        let mut row = 0;
        for (i, piece) in pieces.iter().enumerate() {
            let piece_value = (color as u32) << 3 | *piece as u32;
            row |= piece_value << (i << 2);
        }

        row
    }

    pub fn draw(self) {
        unimplemented!();
    }
}
