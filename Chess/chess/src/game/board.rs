use super::square::Square;
use crate::game::{
    bitboard::BitBoard,
    castle::CastlingRights,
    moves::Move,
    piece::{Colour, Piece},
};

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pieces: [BitBoard; 6],
    sides: [BitBoard; 2],

    piece_map: [Option<Piece>; Square::COUNT],

    pub side: Colour,
    pub castling_right: CastlingRights,
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
            castling_right: CastlingRights::NONE,
            halfmoves: 0,
            side: Colour::White,
        }
    }

    fn piece_at(&self, square: Square) -> Piece {
        self.piece_map[square.index()].unwrap()
    }

    pub fn set_piece(&mut self, piece: Piece, square: Square) {
        let colour = piece.colour() as usize;

        self.sides[colour].set_bit(square);
        self.pieces[piece.index()].set_bit(square);
        self.piece_map[square.index()] = Some(piece);
    }

    pub fn remove_piece(&mut self, square: Square) {
        let piece = self.piece_at(square);
        let colour = piece.colour() as usize;

        self.sides[colour].pop_bit(square);
        self.pieces[piece as usize].pop_bit(square);
        self.piece_map[square.index()] = None;
    }

    pub fn make_move(&mut self, m: Move) {
        self.halfmoves += 1;
        unimplemented!();
    }

    pub fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
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

        board.castling_right = CastlingRights::from(fen[2]);

        board.en_passant = match fen[3] {
            "-" => None,
            _ => Some(Square::from(fen[3])),
        };

        board.halfmoves = fen[4].parse::<u8>().unwrap();

        board
    }

    // NOTE: Optional method draw_with_cursor
    pub fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("\r  a b c d e f g h\r");
        println!(" ┌────────────────┐\r");

        for row in (0..8).rev() {
            print!("{}|", row + 1);
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

            println!("|\r");
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
