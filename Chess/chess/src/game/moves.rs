/// A move needs 16 bits to be stored, the information is contained
/// in the following way:
///
/// bits [0-5]: Destination square (2^6 = 64 possible positions)
/// bits [6-11]: Origin square (2^6 = 64 possible positions)
/// bits [12-13]: Promotion piece type (Knight|Rook|Queen|Bishop)
/// bits [14-15]: If the move is a promotion, an en passant move or castling
pub struct Move(pub u16);
