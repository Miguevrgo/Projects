use crate::game::square::Square;

/// A move needs 16 bits to be stored, the information is contained
/// in the following way:
///
/// bits [0-5]: Destination square (2^6 = 64 possible positions)
/// bits [6-11]: Origin square (2^6 = 64 possible positions)
/// bits [12-13]: Promotion piece type (Knight|Rook|Queen|Bishop)
/// bits [14-15]: If the move is a promotion, an en passant move or castling
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct Move(pub u16);

const DST: u16 = 0b0000_0000_0011_1111;
const SRC: u16 = 0b0000_1111_1100_0000;
const TYPE: u16 = 0b1111_0000_0000_0000;

impl Move {
    pub fn get_source(self) -> Square {
        Square::new((self.0 & SRC) as usize)
    }

    pub fn get_target(self) -> Square {
        Square::new((self.0 & DST) as usize)
    }
}
