/// Each piece is represented by its most significant bits while the color is given by the
/// LSB so that white pieces end with 0 and black pieces 1
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

impl Piece {
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
