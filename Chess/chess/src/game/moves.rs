use crate::game::square::Square;

use super::{
    bitboard::BitBoard,
    board::Board,
    piece::{Colour, Piece},
};

#[rustfmt::skip]
const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (2, 1), (2, -1), (-2, 1), (-2, -1),
    (1, 2), (1, -2), (-1, 2), (-1, -2),
];
#[rustfmt::skip]
const KING_OFFSETS: [(i8, i8); 8] = [
    (1, 0), (1, 1), (1, -1), (0, 1),
    (0, -1), (-1, 0), (-1, 1), (-1, -1),
];
const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const ROOK_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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
        Self((src.index() as u16) | ((dest.index() as u16) << 6) | ((kind as u16) << 12))
    }

    pub fn get_source(self) -> Square {
        Square::new((self.0 & SRC) as usize)
    }

    pub fn get_dest(self) -> Square {
        Square::new(((self.0 & DST) >> 6) as usize)
    }

    pub fn get_type(self) -> MoveKind {
        match (self.0 & TYPE) >> 12 {
            0b0000 => MoveKind::Quiet,
            0b0001 => MoveKind::Castle,
            0b0010 => MoveKind::DoublePush,
            0b0011 => MoveKind::Capture,
            0b0100 => MoveKind::EnPassant,
            0b1000 => MoveKind::KnightPromotion,
            0b1001 => MoveKind::BishopPromotion,
            0b1010 => MoveKind::RookPromotion,
            0b1011 => MoveKind::QueenPromotion,
            _ => unreachable!(),
        }
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

pub fn all_pawn_moves(src: Square, piece: Piece, board: &Board) -> Vec<Move> {
    let mut moves = Vec::with_capacity(4);
    let forward = match piece.colour() {
        Colour::White => -1,
        Colour::Black => 1,
    };
    let start_rank = BitBoard::START_RANKS[piece.colour() as usize];
    let promo_rank = BitBoard::PROMO_RANKS[piece.colour() as usize];

    if let Some(dest) = src.jump(0, forward) {
        if promo_rank.get_bit(dest) {
            moves.push(Move::new(src, dest, MoveKind::QueenPromotion));
            moves.push(Move::new(src, dest, MoveKind::BishopPromotion));
            moves.push(Move::new(src, dest, MoveKind::RookPromotion));
            moves.push(Move::new(src, dest, MoveKind::KnightPromotion));
        } else {
            moves.push(Move::new(src, dest, MoveKind::Quiet))
        }
    }

    if start_rank.get_bit(src) {
        if let Some(dest) = src.jump(0, 2 * forward) {
            if let Some(middle) = src.jump(0, forward) {
                if board.piece_at(middle).is_none() {
                    moves.push(Move::new(src, dest, MoveKind::DoublePush));
                }
            }
        }
    }

    for delta in [(-1, forward), (1, forward)] {
        if let Some(dest) = src.jump(delta.0, delta.1) {
            if promo_rank.get_bit(dest) {
                moves.push(Move::new(src, dest, MoveKind::QueenPromotion));
                moves.push(Move::new(src, dest, MoveKind::BishopPromotion));
                moves.push(Move::new(src, dest, MoveKind::RookPromotion));
                moves.push(Move::new(src, dest, MoveKind::KnightPromotion));
            } else {
                moves.push(Move::new(src, dest, MoveKind::Capture));
                moves.push(Move::new(src, dest, MoveKind::EnPassant));
            }
        }
    }

    moves
}

pub fn all_knight_moves(src: Square) -> Vec<Move> {
    let mut moves = Vec::with_capacity(6);

    for &(file_delta, rank_delta) in &KNIGHT_OFFSETS {
        if let Some(dest) = src.jump(file_delta, rank_delta) {
            moves.push(Move::new(src, dest, MoveKind::Quiet));
            moves.push(Move::new(src, dest, MoveKind::Capture));
        }
    }

    moves
}

pub fn all_bishop_moves(src: Square, board: &Board) -> Vec<Move> {
    let mut moves = Vec::with_capacity(8);

    for &(file_delta, rank_delta) in &BISHOP_DIRECTIONS {
        let mut dest = src;
        while let Some(next) = dest.jump(file_delta, rank_delta) {
            dest = next;
            if board.piece_at(dest).is_some() {
                moves.push(Move::new(src, dest, MoveKind::Capture));
                break;
            }
            moves.push(Move::new(src, dest, MoveKind::Quiet));
        }
    }

    moves
}

pub fn all_rook_moves(src: Square, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    for &(file_delta, rank_delta) in &ROOK_DIRECTIONS {
        let mut dest = src;
        while let Some(next) = dest.jump(file_delta, rank_delta) {
            dest = next;
            if board.piece_at(dest).is_some() {
                moves.push(Move::new(src, dest, MoveKind::Capture));
                break;
            }
            moves.push(Move::new(src, dest, MoveKind::Quiet));
        }
    }
    moves
}

pub fn all_queen_moves(src: Square, board: &Board) -> Vec<Move> {
    let mut moves = all_bishop_moves(src, board);
    moves.extend(all_rook_moves(src, board));
    moves
}

pub fn all_king_moves(src: Square) -> Vec<Move> {
    let mut moves = Vec::with_capacity(4);
    for &(file_delta, rank_delta) in &KING_OFFSETS {
        if let Some(dest) = src.jump(file_delta, rank_delta) {
            moves.push(Move::new(src, dest, MoveKind::Quiet));
            moves.push(Move::new(src, dest, MoveKind::Capture));
        }
    }
    if src.col() == 4 && (src.row() == 0 || src.row() == 7) {
        moves.push(Move::new(
            src,
            Square::from_row_col(src.row(), 6),
            MoveKind::Castle,
        ));
        moves.push(Move::new(
            src,
            Square::from_row_col(src.row(), 2),
            MoveKind::Castle,
        ));
    }

    moves
}
