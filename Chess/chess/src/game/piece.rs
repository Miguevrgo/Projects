/// Each piece type is represented in 3 bits, to distinguish color another
/// bit is used before the type in each piece on the board, empty pieces
/// will be considered white
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Piece {
    Empty = 0b000,
    Pawn = 0b001,
    Bishop = 0b010,
    Knight = 0b011,
    Rook = 0b100,
    King = 0b101,
    Queen = 0b110,
}

impl Piece {
    pub fn from(value: u8) -> Self {
        match value {
            0b000 => Piece::Empty,
            0b001 => Piece::Pawn,
            0b010 => Piece::Bishop,
            0b011 => Piece::Knight,
            0b100 => Piece::Rook,
            0b101 => Piece::King,
            0b110 => Piece::Queen,
            _ => panic!("Invalid piece value: {}", value),
        }
    }
}

/// Represents both possible piece colours black and white in chess, for an empty
/// square, *white* colour is arbitrarily considered
#[derive(Clone, Copy, PartialEq)]
pub enum Colour {
    White = 0b0,
    Black = 0b1,
}

impl Colour {
    pub fn from(value: u8) -> Self {
        match value {
            0b0 => Colour::White,
            0b1 => Colour::Black,
            _ => panic!("Invalid colour: {}", value),
        }
    }
}
