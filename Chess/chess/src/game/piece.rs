use std::fmt;

#[derive(PartialEq)]
pub enum Piece {
    EM, // Empty
    WP, // White Pawn
    WB, // White Bishop
    WN, // White kNight
    WR, // White Rook
    WQ, // White Queen
    WK, // White King
    BP, // Black Pawn
    BB, // Black Bishop
    BN, // Black kNight
    BR, // Black Rook
    BQ, // Black Queen
    BK, // Black King
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Piece::EM => write!(f, " "),
            Piece::WP => write!(f, "♙"),
            Piece::WB => write!(f, "♗"),
            Piece::WN => write!(f, "♘"),
            Piece::WR => write!(f, "♖"),
            Piece::WQ => write!(f, "♕"),
            Piece::WK => write!(f, "♔"),
            Piece::BP => write!(f, "♟"),
            Piece::BB => write!(f, "♝"),
            Piece::BN => write!(f, "♞"),
            Piece::BR => write!(f, "♜"),
            Piece::BQ => write!(f, "♛"),
            Piece::BK => write!(f, "♚"),
        }
    }
}
