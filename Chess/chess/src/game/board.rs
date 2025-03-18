use super::{moves::MoveKind, square::Square};
use crate::game::{
    bitboard::BitBoard,
    castle::CastlingRights,
    moves::*,
    piece::{Colour, Piece},
};

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pieces: [BitBoard; 6],
    sides: [BitBoard; 2],

    piece_map: [Option<Piece>; Square::COUNT],

    pub side: Colour,
    pub castling_rights: CastlingRights,
    pub en_passant: Option<Square>,
    pub halfmoves: u8,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: [BitBoard::EMPTY; 6],
            sides: [BitBoard::EMPTY; 2],
            piece_map: [None; Square::COUNT],
            en_passant: None,
            castling_rights: CastlingRights::NONE,
            halfmoves: 0,
            side: Colour::White,
        }
    }

    pub fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.piece_map[square.index()]
    }

    pub fn set_piece(&mut self, piece: Piece, square: Square) {
        let colour = piece.colour() as usize;
        self.sides[colour] = self.sides[colour].set_bit(square);
        self.pieces[piece.index()] = self.pieces[piece.index()].set_bit(square);
        self.piece_map[square.index()] = Some(piece);
    }

    pub fn remove_piece(&mut self, square: Square) {
        let piece = self.piece_at(square).expect("No piece in position given");
        let colour = piece.colour() as usize;

        self.sides[colour] = self.sides[colour].pop_bit(square);
        self.pieces[piece.index()] = self.pieces[piece.index()].pop_bit(square);
        self.piece_map[square.index()] = None;
    }

    pub fn make_move(&mut self, m: Move) {
        let (src, dest) = (m.get_source(), m.get_dest());
        let src_piece = self.piece_at(src).expect("Invalid source piece");
        let move_type = m.get_type();

        if src_piece.is_pawn() || matches!(move_type, MoveKind::Capture) {
            self.halfmoves = 0
        } else {
            self.halfmoves += 1;
        }

        if src_piece.is_king() {
            if src_piece.colour() == Colour::White {
                self.castling_rights.0 &= !(CastlingRights::WK | CastlingRights::WQ);
            } else {
                self.castling_rights.0 &= !(CastlingRights::BK | CastlingRights::BQ);
            }
        }

        if src_piece.is_rook() {
            match (src_piece.colour(), src.index()) {
                (Colour::White, 0) => self.castling_rights.0 &= !CastlingRights::WQ, // a1
                (Colour::White, 7) => self.castling_rights.0 &= !CastlingRights::WK, // h1
                (Colour::Black, 56) => self.castling_rights.0 &= !CastlingRights::BQ, // a8
                (Colour::Black, 63) => self.castling_rights.0 &= !CastlingRights::BK, // h8
                _ => {}
            }
        }

        match move_type {
            MoveKind::Quiet | MoveKind::DoublePush => {
                self.remove_piece(src);
                self.set_piece(src_piece, dest);

                if matches!(move_type, MoveKind::DoublePush) {
                    let delta = match src_piece.colour() {
                        Colour::White => 1,
                        Colour::Black => -1,
                    };
                    self.en_passant = src.jump(0, delta);
                } else {
                    self.en_passant = None;
                }
            }
            MoveKind::Capture => {
                self.remove_piece(dest);
                self.remove_piece(src);
                self.set_piece(src_piece, dest);
                self.en_passant = None;
            }
            MoveKind::EnPassant => {
                let captured_pawn_square = dest
                    .jump(
                        0,
                        match src_piece.colour() {
                            Colour::White => -1, // Piece is next to pawn
                            Colour::Black => 1,
                        },
                    )
                    .expect("Off the board en_passant");
                self.remove_piece(captured_pawn_square);
                self.remove_piece(src);
                self.set_piece(src_piece, dest);
                self.en_passant = None;
            }
            MoveKind::Castle => {
                let is_kingside = dest.col() > src.col();
                let (rook_src_col, rook_dest_col) = if is_kingside { (7, 5) } else { (0, 3) };
                let row = src.row();
                let rook_src = Square::from_row_col(row, rook_src_col);
                let rook_dest = Square::from_row_col(row, rook_dest_col);
                let rook_piece = self.piece_at(rook_src).expect("Expected rook");

                self.remove_piece(src);
                self.remove_piece(rook_src);
                self.set_piece(src_piece, dest);
                self.set_piece(rook_piece, rook_dest);
                self.en_passant = None;
            }
            MoveKind::KnightPromotion
            | MoveKind::BishopPromotion
            | MoveKind::RookPromotion
            | MoveKind::QueenPromotion => {
                if self.sides[!self.side as usize].get_bit(dest) {
                    self.remove_piece(dest);
                }
                self.remove_piece(src);
                let promo_piece = match move_type {
                    MoveKind::KnightPromotion => {
                        if src_piece.colour() == Colour::White {
                            Piece::WN
                        } else {
                            Piece::BN
                        }
                    }
                    MoveKind::BishopPromotion => {
                        if src_piece.colour() == Colour::White {
                            Piece::WB
                        } else {
                            Piece::BB
                        }
                    }
                    MoveKind::RookPromotion => {
                        if src_piece.colour() == Colour::White {
                            Piece::WR
                        } else {
                            Piece::BR
                        }
                    }
                    MoveKind::QueenPromotion => {
                        if src_piece.colour() == Colour::White {
                            Piece::WQ
                        } else {
                            Piece::BQ
                        }
                    }
                    _ => unreachable!(),
                };
                self.set_piece(promo_piece, dest);
                self.en_passant = None;
            }
        }

        self.side = !self.side;
    }

    pub fn occupied_squares(&self, colour: Colour) -> Vec<Square> {
        let mut squares = Vec::with_capacity(16);
        let mut bitboard = self.sides[colour as usize];

        while bitboard != BitBoard::EMPTY {
            let square = bitboard.lsb();
            squares.push(square);
            bitboard = bitboard.pop_bit(square);
        }

        squares
    }

    fn generate_pseudo_moves(&self, side: Colour) -> Vec<Move> {
        let mut moves = Vec::new();

        for src in self.occupied_squares(side) {
            let piece = self.piece_at(src).unwrap();
            let pseudo_moves = match piece {
                Piece::WP | Piece::BP => all_pawn_moves(src, piece, self),
                Piece::WN | Piece::BN => all_knight_moves(src),
                Piece::WB | Piece::BB => all_bishop_moves(src, self),
                Piece::WR | Piece::BR => all_rook_moves(src, self),
                Piece::WQ | Piece::BQ => all_queen_moves(src, self),
                Piece::WK | Piece::BK => all_king_moves(src),
            };

            moves.extend(pseudo_moves);
        }

        moves
    }

    pub fn generate_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let side = self.side;

        let pseudo_moves = self.generate_pseudo_moves(side);

        for m in pseudo_moves {
            if self.is_pseudo_legal(m) {
                let mut new_board = *self;
                new_board.make_move(m);

                if !new_board.is_in_check(side) {
                    moves.push(m);
                }
            }
        }

        moves
    }

    fn is_pseudo_legal(&self, m: Move) -> bool {
        let src = m.get_source();
        let dest = m.get_dest();
        let move_type = m.get_type();

        let piece = match self.piece_at(src) {
            Some(p) if p.colour() == self.side => p,
            _ => return false,
        };
        let occupied = self.sides[Colour::White as usize] | self.sides[Colour::Black as usize];
        let opponent = self.sides[!self.side as usize];
        let promo_rank = BitBoard::PROMO_RANKS[piece.colour() as usize];

        match move_type {
            MoveKind::Quiet => !occupied.get_bit(dest),
            MoveKind::Capture => opponent.get_bit(dest),
            MoveKind::DoublePush => {
                if !piece.is_pawn() || occupied.get_bit(dest) {
                    return false;
                }
                let mid = src
                    .jump(
                        0,
                        match piece.colour() {
                            Colour::White => 1,
                            Colour::Black => -1,
                        },
                    )
                    .expect("Invalid pos for double push");
                !occupied.get_bit(mid)
            }
            MoveKind::EnPassant => {
                if !piece.is_pawn() || self.en_passant != Some(dest) {
                    return false;
                }
                let ep_target = dest
                    .jump(
                        0,
                        match piece.colour() {
                            Colour::White => 1,
                            Colour::Black => -1,
                        },
                    )
                    .expect("Invalid pos for en en_passant");
                opponent.get_bit(ep_target)
            }
            MoveKind::Castle => {
                if !piece.is_king() || src.col() != 4 || (src.row() != 0 && src.row() != 7) {
                    return false;
                }
                let is_kingside = dest.col() == 6;
                let rights = self.castling_rights.0;
                let row = src.row();
                let (rook_col, mid_cols) = if is_kingside {
                    (7, vec![5, 6])
                } else {
                    (0, vec![1, 2, 3])
                };
                let rook_sq = Square::from_row_col(row, rook_col);

                (if is_kingside {
                    (piece.colour() == Colour::White && (rights & CastlingRights::WK != 0))
                        || (piece.colour() == Colour::Black && (rights & CastlingRights::BK != 0))
                } else {
                    (piece.colour() == Colour::White && (rights & CastlingRights::WQ != 0))
                        || (piece.colour() == Colour::Black && (rights & CastlingRights::BQ != 0))
                }) && self.piece_at(rook_sq)
                    == Some(if piece.colour() == Colour::White {
                        Piece::WR
                    } else {
                        Piece::BR
                    })
                    && mid_cols
                        .iter()
                        .all(|&col| !occupied.get_bit(Square::from_row_col(row, col)))
            }
            MoveKind::KnightPromotion
            | MoveKind::BishopPromotion
            | MoveKind::RookPromotion
            | MoveKind::QueenPromotion => {
                piece.is_pawn()
                    && promo_rank.get_bit(dest)
                    && (opponent.get_bit(dest) || !occupied.get_bit(dest))
            }
        }
    }

    pub fn is_in_check(&self, colour: Colour) -> bool {
        let king_sq = self.king_square(colour);
        let opponent_moves = self.generate_pseudo_moves(!colour);
        opponent_moves.iter().any(|m| m.get_dest() == king_sq)
    }

    pub fn king_square(&self, colour: Colour) -> Square {
        let king_bb = self.pieces[Piece::WK.index()] & self.sides[colour as usize];
        king_bb.lsb()
    }

    pub fn from_fen(state: &str) -> Self {
        let fen: Vec<&str> = state.split_whitespace().take(6).collect();

        if fen.len() != 6 {
            panic!("Invalid input FEN string");
        }

        let board_layout = fen[0];
        let mut board = Board::new();
        let (mut row, mut col): (u8, u8) = (7, 0);
        let mut tokens = 0;

        for token in board_layout.chars() {
            match token {
                '/' => {
                    if tokens != 8 {
                        panic!("Invalid number of positions in FEN");
                    }

                    row -= 1;
                    col = 0;
                    tokens = 0;
                }
                '1'..='8' => {
                    let empty_pos = token.to_digit(10).expect("Not a number") as u8;
                    col += empty_pos;
                    tokens += empty_pos;
                }
                _ => {
                    board.set_piece(
                        Piece::from_fen(token),
                        Square::from_row_col(row as usize, col as usize),
                    );

                    col += 1;
                    tokens += 1;
                }
            }
        }

        board.side = match fen[1] {
            "w" => Colour::White,
            "b" => Colour::Black,
            _ => unreachable!(),
        };

        board.castling_rights = CastlingRights::from(fen[2]);

        board.en_passant = match fen[3] {
            "-" => None,
            _ => Some(Square::from(fen[3])),
        };

        board.halfmoves = fen[4].parse::<u8>().unwrap();

        board
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("\r  a b c d e f g h\r");
        println!(" ┌────────────────┐\r");

        for row in (0..8).rev() {
            print!("{}│", row + 1);
            for col in 0..8 {
                let square = Square::from_row_col(row, col);
                let bg_colour = if (row + col) % 2 == 0 {
                    "\x1b[48;2;240;217;181m"
                } else {
                    "\x1b[48;2;181;136;99m"
                };

                match self.piece_map[square.index()] {
                    Some(piece) => match piece.colour() {
                        Colour::White => print!("{bg_colour}\x1b[38;2;255;255;255m{piece} \x1b[0m"),
                        Colour::Black => print!("{bg_colour}\x1b[38;2;0;0;0m{piece} \x1b[0m"),
                    },
                    None => print!("{bg_colour}  \x1b[0m"),
                }
            }

            println!("│\r");
        }

        println!(" └────────────────┘\r");
    }
}

// For debugging
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  a b c d e f g h")?;
        writeln!(f, " ┌────────────────┐")?;

        for row in (0..8).rev() {
            write!(f, "{}│", row + 1)?;
            for col in 0..8 {
                let square = Square::from_row_col(row, col);
                match self.piece_map[square.index()] {
                    Some(piece) => write!(f, "{} ", piece)?,
                    None => write!(f, "  ")?,
                }
            }
            writeln!(f, "│")?;
        }

        writeln!(f, " └────────────────┘")?;
        Ok(())
    }
}
