use crate::game::directions::*;
use crate::game::piece::*;

/// Represents a chess board with 64 squares, each holding a piece.
///
/// The board is implemented as an array of 8 `u32` values, where each `u32` represents a row.
/// Each square uses 4 bits to encode a piece: 1 bit for the color (`Colour`) and 3 bits for the
/// piece type (`Piece`). This allows efficient storage of 8 squares per row (32 bits total).
/// The internal representation of a piece is as follows:
///
/// | Bit 3     | Bit 2     | Bit 1     | Bit 0     |
/// |-----------|-----------|-----------|-----------|
/// | Color     |      Piece Type (3 bits)          |
///
/// - **Bit 3**: Represents the color (0 = White, 1 = Black).
/// - **Bits 2-0**: Represent the piece type (e.g., 000 = Empty, 001 = Pawn, etc.).
///
/// A cursor tracks the currently selected square for movement purposes.
pub struct Board {
    board: [u32; 8],
    pub cursor: (usize, usize),
    pub selected: Option<(usize, usize)>,
}

impl Board {
    /// Creates a new chess board with pieces in their standard starting positions.
    ///
    /// White pieces are placed at the top (rows 0 and 1), and Black pieces at the bottom
    /// (rows 6 and 7), matching standard chess notation where White starts on rows 1 and 2.
    /// The cursor starts at (0, 0), and no piece is selected initially.
    ///
    /// # Returns
    /// A new `Board` instance with the default chess setup.
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

    /// Constructs a `u32` value representing a row of 8 pieces.
    ///
    /// Each piece occupies 4 bits: the most significant bit indicates the color (`Colour`),
    /// and the remaining 3 bits represent the piece type (`Piece`). The pieces are packed
    /// in order from left to right (column 0 to 7).
    ///
    /// # Parameters
    /// - `colour`: The color of all pieces in the row.
    /// - `pieces`: An array of 8 `Piece` values to place in the row.
    ///
    /// # Returns
    /// A `u32` value encoding the row.
    fn create_row(colour: Colour, pieces: &[Piece]) -> u32 {
        let mut row = 0;
        for (i, piece) in pieces.iter().enumerate() {
            let piece_value = ((colour as u32) << 3) | *piece as u32;
            row |= piece_value << (i << 2);
        }

        row
    }

    /// Retrieves the piece and its color at a given position on the board.
    ///
    /// Positions are zero-based, with (0, 0) being the top-left corner (a8 in chess notation).
    ///
    /// # Parameters
    /// - `row`: The row index, must be in [0, 7].
    /// - `col`: The column index, must be in [0, 7].
    ///
    /// # Returns
    /// A tuple `(Colour, Piece)` representing the color and type of the piece at the position.
    ///
    /// # Panics
    /// Panics if `row` or `col` is not in the range [0, 7].
    pub fn get_piece(&self, row: usize, col: usize) -> (Colour, Piece) {
        assert!((0..=7).contains(&row) && ((0..=7).contains(&col)));

        let board_row = self.board[row];

        let bits = (board_row >> (col * 4)) & 0b1111;
        let colour = Colour::from((bits >> 3) as u8);
        let piece = Piece::from((bits & 0b0111) as u8);

        (colour, piece)
    }

    /// Sets a piece at a specified position on the board.
    ///
    /// Overwrites the existing piece at the position without checking move validity.
    ///
    /// # Parameters
    /// - `row`: The row index, must be in [0, 7].
    /// - `col`: The column index, must be in [0, 7].
    /// - `colour`: The color of the piece to set.
    /// - `piece`: The type of piece to set.
    ///
    /// # Panics
    /// Panics if `row` or `col` is not in the range [0, 7].
    pub fn set_piece(&mut self, row: usize, col: usize, colour: Colour, piece: Piece) {
        assert!((0..=7).contains(&row) && ((0..=7).contains(&col)));
        let piece_bits = ((colour as u32) << 3) | (piece as u32);
        let mask: u32 = !(0b1111 << (col * 4));
        self.board[row] &= mask;
        self.board[row] |= piece_bits << (col * 4);
    }

    /// Moves a piece from one position to another on the board.
    ///
    /// Does not validate the move's legality (e.g., chess rules). The original position
    /// is cleared to an empty white square after the move.
    ///
    /// # Parameters
    /// - `row`: The starting row index, must be in [0, 7].
    /// - `col`: The starting column index, must be in [0, 7].
    /// - `new_r`: The destination row index, must be in [0, 7].
    /// - `new_c`: The destination column index, must be in [0, 7].
    ///
    /// # Returns
    /// `true` if the move resulted in a capture (destination was not empty), `false` otherwise.
    pub fn move_piece(&mut self, row: usize, col: usize, new_r: usize, new_c: usize) -> bool {
        let (colour, piece) = Self::get_piece(self, row, col);
        Self::set_piece(self, row, col, Colour::White, Piece::Empty);
        let capture = Self::get_piece(self, row, col) != (Colour::White, Piece::Empty);
        Self::set_piece(self, new_r, new_c, colour, piece);

        capture
    }

    /// Moves the cursor one position in the specified direction.
    ///
    /// If the cursor reaches the board's edge, it wraps around to the opposite side.
    /// If a piece is selected, this only affects the cursor, not the selected piece.
    ///
    /// # Parameters
    /// - `dir`: The direction to move the cursor (`Up`, `Down`, `Left`, `Right`, or `Select`).
    pub fn move_cursor(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.cursor.0 = (self.cursor.0 + 1) % 8,
            Direction::Down => self.cursor.0 = (self.cursor.0 + 7) % 8,
            Direction::Left => self.cursor.1 = (self.cursor.1 + 7) % 8,
            Direction::Right => self.cursor.1 = (self.cursor.1 + 1) % 8,
            Direction::Select => self.selected = Some(self.cursor),
        }
    }

    /// Renders the chess board to the terminal.
    ///
    /// Displays the board with pieces as Unicode symbols, using colors to indicate:
    /// - Cursor position (green background).
    /// - Selected piece (red background).
    /// - Alternating square colors (light and dark brown).
    /// - Piece colors (white or black).
    pub fn draw(&self) {
        let symbols = [
            ' ', // Empty
            '♟', // Pawn
            '♝', // Bishop
            '♞', // Knight
            '♜', // Rook
            '♚', // King
            '♛', // Queen
        ];

        print!("\x1B[2J\x1B[1;1H");
        println!("\r  a b c d e f g h\r");
        println!(" ┌────────────────┐\r");
        for row in (0..8).rev() {
            print!("{}│", row + 1);
            for col in 0..8 {
                let (colour, piece) = self.get_piece(row, col);
                let symbol = symbols[piece as usize];

                let bg_color = if self.cursor == (row, col) {
                    "\x1b[41m" // Green
                } else if self.selected == Some((row, col)) {
                    "\x1b[102m" // Red
                } else if (row + col) % 2 == 0 {
                    "\x1b[48;2;240;217;181m"
                } else {
                    "\x1b[48;2;181;136;99m"
                };

                let piece_color = if colour == Colour::White {
                    "\x1b[38;2;255;255;255m"
                } else {
                    "\x1b[38;2;0;0;0m"
                };

                print!("{bg_color}{piece_color}{symbol} \x1b[0m");
            }
            println!("│\r");
        }
        println!(" └────────────────┘\r");
    }
}
