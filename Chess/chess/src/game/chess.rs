use super::piece::{Colour, Piece};
use crate::game::board::Board;

pub struct Game {
    pub turn: u16, // Despite 5899 being the maximum number of moves possible
    board: Board,
    log: String,
    is_white_check: bool, // Is white king in check
    is_black_check: bool, // Is dark king in check
}

impl Game {
    pub fn new() -> Self {
        Game {
            turn: 1,
            board: Board::new(),
            log: String::new(),
            is_white_check: false,
            is_black_check: false,
        }
    }

    pub fn next_move(&mut self) {
        // Read move (TODO)
        let (row, col) = (1, 2);
        let (new_row, new_col) = (2, 2);
        let colour_turn = if self.turn % 2 == 0 {
            Colour::Black
        } else {
            Colour::White
        };

        if Self::valid_move(self, row, col, new_row, new_col, colour_turn) {
            self.board.move_piece(row, col, new_row, new_col);
            Self::log_movement(self, row, col, new_row, new_col);
        }

        self.turn += 1;
    }

    fn valid_move(
        &mut self,
        row: usize,
        col: usize,
        new_row: usize,
        new_col: usize,
        colour_turn: Colour,
    ) -> bool {
        if !(0..=7).contains(&new_row) || !(0..=7).contains(&new_col) {
            return false;
        }

        let (colour, piece) = self.board.get_piece(row, col);
        let (dest_colour, dest_piece) = self.board.get_piece(new_row, new_col);
        if colour != colour_turn
            || (dest_piece != Piece::Empty && dest_colour == colour)
            || piece == Piece::Empty
            || (piece != Piece::King && colour_turn == Colour::White && self.is_white_check)
            || (piece != Piece::King && colour_turn == Colour::Black && self.is_black_check)
        {
            return false;
        }

        match piece {
            Piece::Pawn => {
                Self::pawn_valid_moves(self, row, col, colour).contains(&(new_row, new_col))
            }
            _ => false,
        }
    }

    /// Returns a vector of the possible moves a given pawn in a position can do, these moves
    /// include going one position up or down always, two positions in starting positions and
    /// diagonally if captured piece is of the opposite colour, TODO: En passant
    /// TODO: A single is_valid_pawn_move could be used to simplify however for the sake of
    /// having a preview of the moves when clicking a piece, this method will be implemented
    /// TODO: There has to be a better approach for king checks
    fn pawn_valid_moves(&mut self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();

        match colour {
            Colour::White => {
                valid_moves.push((row + 1, col));
                if row == 1 {
                    valid_moves.push((row + 2, col));
                }
                if col < 7 {
                    let diagonal = self.board.get_piece(row + 1, col + 1);
                    if diagonal.1 != Piece::Empty && diagonal.0 != colour {
                        valid_moves.push((row + 1, col + 1))
                    }
                }
                if col > 0 {
                    let diagonal = self.board.get_piece(row + 1, col - 1);
                    if diagonal.1 != Piece::Empty && diagonal.0 != colour {
                        valid_moves.push((row + 1, col - 1))
                    }
                }
            }
            Colour::Black => {
                valid_moves.push((row - 1, col));
                if row == 6 {
                    valid_moves.push((row - 2, col));
                }
                if col < 7 {
                    let diagonal = self.board.get_piece(row - 1, col + 1);
                    if diagonal.1 != Piece::Empty && diagonal.0 != colour {
                        valid_moves.push((row - 1, col + 1))
                    }
                }
                if col > 0 {
                    let diagonal = self.board.get_piece(row - 1, col - 1);
                    if diagonal.1 != Piece::Empty && diagonal.0 != colour {
                        valid_moves.push((row - 1, col - 1))
                    }
                }
            }
        }

        for (row, col) in &valid_moves {
            if self.board.get_piece(*row, *col) == (Colour::White, Piece::King) {
                self.is_white_check = true;
            } else if self.board.get_piece(*row, *col) == (Colour::Black, Piece::King) {
                self.is_black_check = true;
            }
        }

        valid_moves
    }

    fn log_movement(&mut self, row: usize, col: usize, new_row: usize, new_col: usize) {
        self.log += &format!("Move from {row} {col} to {new_row} {new_col}");
    }

    pub fn draw(&self) {
        println!("Turn {}", self.turn);
        println!();
        self.board.draw();
        println!();
    }
}
