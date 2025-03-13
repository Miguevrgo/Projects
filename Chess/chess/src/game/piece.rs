/// Represents a chess piece, with its type encoded in the most significant bits
/// and its color in the least significant bit (0 for White, 1 for Black).
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
pub enum Piece {
    WP = 0,
    BP = 1,
    WN = 2,
    BN = 3,
    WB = 4,
    BB = 5,
    WR = 6,
    BR = 7,
    WQ = 8,
    BQ = 9,
    WK = 10,
    BK = 11,
}

// Mapping of pieces to their FEN characters.
#[rustfmt::skip]
const PIECE_CHAR: [char; 12] = [
    'P', 'p', 'N', 'n', 'B', 'b',
    'R', 'r', 'Q', 'q', 'K', 'k',
];

impl Piece {
    /// Array of all possible pieces
    pub const ALL: [Self; 12] = [
        Piece::WP,
        Piece::BP,
        Piece::WN,
        Piece::BN,
        Piece::WB,
        Piece::BB,
        Piece::WR,
        Piece::BR,
        Piece::WQ,
        Piece::BQ,
        Piece::WK,
        Piece::BK,
    ];

    /// Creates a `Piece` from an index into `Self::ALL`.
    fn from(index: usize) -> Self {
        Self::ALL[index]
    }

    /// Converts a FEN character into a `Piece`.
    ///
    /// # Arguments
    ///
    /// * `value` - The FEN character (e.g., 'P' for White Pawn, 'k' for Black King).
    ///
    /// # Panics
    ///
    /// Panics if the character is not a valid FEN piece representation.
    pub fn from_fen(value: char) -> Self {
        Self::from(
            PIECE_CHAR
                .iter()
                .position(|&ch| ch == value)
                .expect("Not found FEN char"),
        )
    }

    /// Returns the piece type index, ignoring color (e.g., both WP and BP return 0).
    pub const fn index(self) -> usize {
        self as usize / 2
    }

    pub const fn colour(self) -> Colour {
        if self as u8 & 1 == 0 {
            Colour::White
        } else {
            Colour::Black
        }
    }

    pub const fn opposite_colour(self) -> Self {
        Piece::ALL[(self as u8 ^ 1) as usize]
    }

    pub const fn is_pawn(self) -> bool {
        self as u8 & 0b1110 == 0b0000
    }

    pub const fn is_knight(self) -> bool {
        self as u8 & 0b1110 == 0b0010
    }

    pub const fn is_bishop(self) -> bool {
        self as u8 & 0b1110 == 0b0100
    }

    pub const fn is_rook(self) -> bool {
        self as u8 & 0b1110 == 0b0110
    }

    pub const fn is_queen(self) -> bool {
        self as u8 & 0b1110 == 0b1000
    }

    pub const fn is_king(self) -> bool {
        self as u8 & 0b1110 == 0b1010
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Piece::WP => '♟',
            Piece::WN => '♞',
            Piece::WB => '♝',
            Piece::WR => '♜',
            Piece::WQ => '♛',
            Piece::WK => '♚',
            Piece::BP => '♙',
            Piece::BN => '♘',
            Piece::BB => '♗',
            Piece::BR => '♖',
            Piece::BQ => '♕',
            Piece::BK => '♔',
        };
        write!(f, "{}", symbol)
    }
}

/// Represents both possible piece colours black and white in chess
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
pub enum Colour {
    White = 0,
    Black = 1,
}

impl std::ops::Not for Colour {
    type Output = Colour;

    fn not(self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}
