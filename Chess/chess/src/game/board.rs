use crate::game::piece::*;

/// A board consists on 64 squares where each one contains one piece (where empty is considered)
/// a piece, as there are 7 possible pieces, each one with a colour, we can fit each piece in 4
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
                Self::create_row(Colour::White, &[Pawn; 8]),
                Self::create_row(
                    Colour::White,
                    &[Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
                ),
                Self::create_row(Colour::White, &[Empty; 8]),
                Self::create_row(Colour::White, &[Empty; 8]),
                Self::create_row(Colour::White, &[Empty; 8]),
                Self::create_row(Colour::White, &[Empty; 8]),
                Self::create_row(
                    Colour::Black,
                    &[Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
                ),
                Self::create_row(Colour::Black, &[Pawn; 8]),
            ],
        }
    }

    /// Creates an u32 value representing a row of 8 pieces given their colour which
    /// is stored in most significant bit of each piece and the piece which are stored
    /// in the same order as the one in pieces
    fn create_row(colour: Colour, pieces: &[Piece]) -> u32 {
        let mut row = 0;
        for (i, piece) in pieces.iter().enumerate() {
            let piece_value = (colour as u32) << 3 | *piece as u32;
            row |= piece_value << (i << 2);
        }

        row
    }

    /// Gets the piece and colour in the given position, starting in 0 for
    /// both row and col, ensuring pos_x and pos_y in [0,7]x[0,7]
    pub fn get_piece(&self, pos_x: usize, pos_y: usize) -> (Colour, Piece) {
        assert!((0..=7).contains(&pos_x) && ((0..=7).contains(&pos_y)));

        let row = self.board[pos_x];

        let bits = row >> (pos_y * 4) & 0b1111;
        let colour = Colour::from((bits >> 3) as u8);
        let piece = Piece::from((bits & 0b0111) as u8);

        (colour, piece)
    }

    /// Sets a piece in given new position checking bounds, does not check whether the move
    /// is valid or not
    pub fn set_piece(&mut self, pos_x: usize, pos_y: usize, colour: Colour, piece: Piece) {
        assert!((0..=7).contains(&pos_x) && ((0..=7).contains(&pos_y)));
        let piece_bits = (colour as u32) << 3 | (piece as u32);
        let mask: u32 = !(0b1111 << (pos_y * 4));
        self.board[pos_x] &= mask;
        self.board[pos_x] |= piece_bits << (pos_y * 4);
    }

    /// Moves a piece from old position to new given one, not checking whether or not the given
    /// piece was valid, it returns whether or not the move was a capture (not checking whether the
    /// capture is valid, it just checks whether the new position was occupied by a non empty piece)
    pub fn move_piece(&mut self, pos_x: usize, pos_y: usize, new_x: usize, new_y: usize) -> bool {
        let (colour, piece) = Self::get_piece(self, pos_x, pos_y);
        Self::set_piece(self, pos_x, pos_y, Colour::White, Piece::Empty);
        let capture = Self::get_piece(self, new_x, new_y) != (Colour::White, Piece::Empty);
        Self::set_piece(self, new_x, new_y, colour, piece);

        capture
    }

    pub fn draw(self) {
        unimplemented!();
    }
}
