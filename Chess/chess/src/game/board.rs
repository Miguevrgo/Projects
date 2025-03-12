use super::square::Square;
use crate::game::{
    bitboard::BitBoard,
    castle::CastlingRights,
    piece::{Colour, Piece},
};

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pieces: [BitBoard; 6],
    sides: [BitBoard; 2],

    en_passant: Option<Square>,
    castling_right: CastlingRights,
    halfmoves: u8,
    pub side: Colour,
}
