use crate::game::square::Square;

/// A move needs 16 bits to be stored, the information is contained
/// in the following way:
///
/// bits [0-5]: Origin square (2^6 = 64 possible positions)
/// bits [6-11]: Destination square (2^6 = 64 possible positions)
/// bits [12-13]: Promotion piece type (Knight|Rook|Queen|Bishop)
/// bits [14-15]: If the move is a promotion, an en passant move or castling
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct Move(pub u16);

const SRC: u16 = 0b0000_0000_0011_1111;
const DST: u16 = 0b0000_1111_1100_0000;
const TYPE: u16 = 0b1111_0000_0000_0000;

impl Move {
    pub fn new(src: Square, dest: Square, kind: MoveKind) -> Self {
        Self((src.index() as u16) | (dest.index() as u16) << 6 | (kind as u16) << 12)
    }

    pub fn get_source(self) -> Square {
        Square::new((self.0 & SRC) as usize)
    }

    pub fn get_dest(self) -> Square {
        Square::new(((self.0 & DST) >> 6) as usize)
    }
}

/// MoveKind is a 4-bit enum that represents the type of move
/// Structured as follows:
///
/// 4rd bit: 1 if the move is a promotion, 0 otherwise
pub enum MoveKind {
    Quiet = 0b0000,
    Castle = 0b0001,
    DoublePush = 0b0010,
    Capture = 0b0011,
    EnPassant = 0b0100,

    KnightPromotion = 0b1000,
    BishopPromotion = 0b1001,
    RookPromotion = 0b1010,
    QueenPromotion = 0b1011,
}
