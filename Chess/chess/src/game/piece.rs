/// Each piece type is represented in 3 bits, to distinguish color another
/// bit is used before the type in each piece on the board, empty pieces
/// will be considered white
#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    Empty = 0b000,
    Pawn = 0b001,
    Bishop = 0b010,
    Knight = 0b011,
    Rook = 0b100,
    King = 0b101,
    Queen = 0b110,
}

#[derive(Clone, Copy)]
pub enum Color {
    White = 0b0,
    Black = 0b1,
}

//TODO: Use this in board, considering color
// impl fmt::Display for Piece {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             Piece::EM => write!(f, " "),
//             Piece::WP => write!(f, "♙"),
//             Piece::WB => write!(f, "♗"),
//             Piece::WN => write!(f, "♘"),
//             Piece::WR => write!(f, "♖"),
//             Piece::WQ => write!(f, "♕"),
//             Piece::WK => write!(f, "♔"),
//             Piece::BP => write!(f, "♟"),
//             Piece::BB => write!(f, "♝"),
//             Piece::BN => write!(f, "♞"),
//             Piece::BR => write!(f, "♜"),
//             Piece::BQ => write!(f, "♛"),
//             Piece::BK => write!(f, "♚"),
//         }
//     }
// }
