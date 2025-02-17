use crate::game::directions::*;
use crate::game::piece::*;

/// A board consists on 64 squares where each one contains one piece (where empty is considered)
/// a piece, as there are 7 possible pieces, each one with a colour, we can fit each piece in 4
/// bits, as we want 8 pieces per row and 8 rows, we need an array of 8 u32 values
///
/// A cursor is used to keep track of the current selected piece so that it can be moved
pub struct Board {
    board: [u32; 8],
    pub cursor: (usize, usize),
    pub selected: Option<(usize, usize)>,
}

impl Board {
    /// Creates a board with default pieces positions, white is on top because top represents
    /// the first position on the array which matches the 1 and 2 rows where white are always
    /// located
    pub fn new() -> Self {
        use Piece::*;
        Board {
            board: [
                Self::create_row(
                    Colour::White,
                    &[Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
                ),
                Self::create_row(Colour::White, &[Pawn; 8]),
                Self::create_row(Colour::White, &[Empty; 8]),
                Self::create_row(Colour::White, &[Empty; 8]),
                Self::create_row(Colour::White, &[Empty; 8]),
                Self::create_row(Colour::White, &[Empty; 8]),
                Self::create_row(Colour::Black, &[Pawn; 8]),
                Self::create_row(
                    Colour::Black,
                    &[Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
                ),
            ],
            cursor: (0, 0),
            selected: None,
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
    /// both row and col, ensuring row and col in [0,7]x[0,7]
    pub fn get_piece(&self, row: usize, col: usize) -> (Colour, Piece) {
        assert!((0..=7).contains(&row) && ((0..=7).contains(&col)));

        let board_row = self.board[row];

        let bits = board_row >> (col * 4) & 0b1111;
        let colour = Colour::from((bits >> 3) as u8);
        let piece = Piece::from((bits & 0b0111) as u8);

        (colour, piece)
    }

    /// Sets a piece in given new position checking bounds, does not check whether the move
    /// is valid or not
    pub fn set_piece(&mut self, row: usize, col: usize, colour: Colour, piece: Piece) {
        assert!((0..=7).contains(&row) && ((0..=7).contains(&col)));
        let piece_bits = (colour as u32) << 3 | (piece as u32);
        let mask: u32 = !(0b1111 << (col * 4));
        self.board[row] &= mask;
        self.board[row] |= piece_bits << (col * 4);
    }

    /// Moves a piece from old position to new given one, not checking whether or not the given
    /// piece was valid, it returns whether or not the move was a capture (not checking whether the
    /// capture is valid, it just checks whether the new position was occupied by a non empty piece)
    pub fn move_piece(&mut self, row: usize, col: usize, new_r: usize, new_c: usize) -> bool {
        let (colour, piece) = Self::get_piece(self, row, col);
        Self::set_piece(self, row, col, Colour::White, Piece::Empty);
        let capture = Self::get_piece(self, row, col) != (Colour::White, Piece::Empty);
        Self::set_piece(self, new_r, new_c, colour, piece);

        capture
    }

    /// Moves the cursor one position in the given direction. If the cursor is at
    /// the edge of the board, it wraps around to the opposite side. If selected,
    /// it only moves selected cursor
    pub fn move_cursor(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.cursor.0 = (self.cursor.0 + 1) % 8,
            Direction::Down => self.cursor.0 = (self.cursor.0 + 7) % 8,
            Direction::Left => self.cursor.1 = (self.cursor.1 + 7) % 8,
            Direction::Right => self.cursor.1 = (self.cursor.1 + 1) % 8,
            Direction::Select => self.selected = Some(self.cursor),
        }
    }

    /// Draws the board in terminal inside a square getting each of the pieces in each
    /// position
    pub fn draw(&self) {
        let symbols = [
            (' ', ' '), // Empty
            ('♙', '♟'), // Pawn
            ('♗', '♝'), // Bishop
            ('♘', '♞'), // Knight
            ('♖', '♜'), // Rook
            ('♔', '♚'), // King
            ('♕', '♛'), // Queen
        ];

        println!("  a b c d e f g h");
        println!(" ┌────────────────┐");
        for row in (0..8).rev() {
            print!("{}│", row + 1);
            for col in 0..8 {
                let (colour, piece) = self.get_piece(row, col);
                let symbol = if colour == Colour::White {
                    symbols[piece as usize].0
                } else {
                    symbols[piece as usize].1
                };
                if self.cursor == (row, col) {
                    print!("\x1b[93m{symbol} \x1b[0m");
                } else if self.selected == Some((row, col)) {
                    print!("\x1b[31m{symbol} \x1b[0m");
                } else {
                    print!("{symbol} ",);
                }
            }
            println!("│");
        }
        println!(" └────────────────┘");
    }
}
