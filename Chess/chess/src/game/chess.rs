use super::piece::{Colour, Piece};
use crate::game::board::Board;
use crate::game::directions::*;

pub struct Game {
    pub turn: u16, // Despite 5899 being the maximum number of moves possible
    board: Board,
    log: String,
    is_white_check: bool, // Is white king in check
    is_black_check: bool, // Is dark king in check
}

impl Game {
    /// Creates a new game of chess, with a default board, empty log and white
    /// to move, it also sets both kings out of check
    pub fn new() -> Self {
        Game {
            turn: 1,
            board: Board::new(),
            log: String::new(),
            is_white_check: false,
            is_black_check: false,
        }
    }

    /// Gets the next desired move as an input from the keyboard, in order for
    /// a player to move, first a square has to be selected, then another set
    /// of inputs determines where the player wants to move, if the move is not
    /// valid, this method loops until a valid move is found
    pub fn next_move(&mut self) {
        loop {
            if let Some(dir) = Direction::input_key() {
                if dir == Direction::Select {
                    if self.board.selected.is_none() {
                        self.board.selected = Some(self.board.cursor);
                    } else {
                        let (row, col) = self.board.selected.unwrap();
                        let (new_row, new_col) = self.board.cursor;
                        if self.valid_move(row, col, new_row, new_col) {
                            self.board.move_piece(row, col, new_row, new_col);
                            self.log_movement(row, col, new_row, new_col);
                            self.turn += 1;
                            self.board.selected = None;
                            self.draw();
                            break;
                        } else {
                            self.board.selected = None;
                        }
                    }
                } else {
                    self.board.move_cursor(&dir);
                }
            }
            self.draw();
        }
    }

    /// Returns whether or not a move is valid based on the piece to move and the new
    /// position where it wants to move, to do so, it tests if movement is of the player
    /// whose turn it is and if its piece can move that way in the context of the game
    fn valid_move(&mut self, row: usize, col: usize, new_row: usize, new_col: usize) -> bool {
        if !(0..=7).contains(&new_row) || !(0..=7).contains(&new_col) {
            return false;
        }

        let (colour, piece) = self.board.get_piece(row, col);
        let (dest_colour, dest_piece) = self.board.get_piece(new_row, new_col);
        let colour_turn = if self.turn % 2 == 0 {
            Colour::Black
        } else {
            Colour::White
        };

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
            Piece::Rook => {
                Self::rook_valid_moves(self, row, col, colour).contains(&(new_row, new_col))
            }
            Piece::Bishop => {
                Self::bishop_valid_moves(self, row, col, colour).contains(&(new_row, new_col))
            }
            _ => false,
        }
    }

    fn king_checked(&mut self, valid_moves: &Vec<(usize, usize)>) {
        for (row, col) in valid_moves {
            if self.board.get_piece(*row, *col) == (Colour::White, Piece::King) {
                self.is_white_check = true;
            } else if self.board.get_piece(*row, *col) == (Colour::Black, Piece::King) {
                self.is_black_check = true;
            }
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
                if row < 7 && self.board.get_piece(row + 1, col).1 == Piece::Empty {
                    valid_moves.push((row + 1, col));
                }

                if row == 1
                    && self.board.get_piece(row + 1, col).1 == Piece::Empty
                    && self.board.get_piece(row + 2, col).1 == Piece::Empty
                {
                    valid_moves.push((row + 2, col));
                }

                if col < 7 && row < 7 {
                    let diagonal = self.board.get_piece(row + 1, col + 1);
                    if diagonal.1 != Piece::Empty && diagonal.0 != colour {
                        valid_moves.push((row + 1, col + 1))
                    }
                }
                if col > 0 && row < 7 {
                    let diagonal = self.board.get_piece(row + 1, col - 1);
                    if diagonal.1 != Piece::Empty && diagonal.0 != colour {
                        valid_moves.push((row + 1, col - 1))
                    }
                }
            }
            Colour::Black => {
                if row > 0 && self.board.get_piece(row - 1, col).1 == Piece::Empty {
                    valid_moves.push((row - 1, col));
                }

                if row == 6
                    && self.board.get_piece(row - 1, col).1 == Piece::Empty
                    && self.board.get_piece(row - 2, col).1 == Piece::Empty
                {
                    valid_moves.push((row - 2, col));
                }

                if col < 7 && row > 0 {
                    let diagonal = self.board.get_piece(row - 1, col + 1);
                    if diagonal.1 != Piece::Empty && diagonal.0 != colour {
                        valid_moves.push((row - 1, col + 1))
                    }
                }
                if col > 0 && row > 0 {
                    let diagonal = self.board.get_piece(row - 1, col - 1);
                    if diagonal.1 != Piece::Empty && diagonal.0 != colour {
                        valid_moves.push((row - 1, col - 1))
                    }
                }
            }
        }

        Self::king_checked(self, &valid_moves);

        valid_moves
    }

    fn rook_valid_moves(&mut self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();

        // Left
        for c in (0..col).rev() {
            let (piece_colour, piece) = self.board.get_piece(row, c);
            if piece != Piece::Empty {
                if piece_colour != colour {
                    valid_moves.push((row, c));
                }
                break;
            }
            valid_moves.push((row, c));
        }

        // Right
        for c in (col + 1)..8 {
            let (piece_colour, piece) = self.board.get_piece(row, c);
            if piece != Piece::Empty {
                if piece_colour != colour {
                    valid_moves.push((row, c));
                }
                break;
            }
            valid_moves.push((row, c));
        }

        // Up
        for r in (0..row).rev() {
            let (piece_colour, piece) = self.board.get_piece(r, col);
            if piece != Piece::Empty {
                if piece_colour != colour {
                    valid_moves.push((r, col));
                }
                break;
            }
            valid_moves.push((r, col));
        }

        // Down
        for r in (row + 1)..8 {
            let (piece_colour, piece) = self.board.get_piece(r, col);
            if piece != Piece::Empty {
                if piece_colour != colour {
                    valid_moves.push((r, col));
                }
                break;
            }
            valid_moves.push((r, col));
        }

        Self::king_checked(self, &valid_moves);

        valid_moves
    }

    fn bishop_valid_moves(
        &mut self,
        row: usize,
        col: usize,
        colour: Colour,
    ) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();

        for (dr, dc) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let mut r = row as isize;
            let mut c = col as isize;
            loop {
                r += dr;
                c += dc;

                if !(0..=7).contains(&r) || !(0..=7).contains(&c) {
                    break;
                }

                let pos_r = r as usize;
                let pos_c = c as usize;

                let (piece_colour, piece) = self.board.get_piece(pos_r, pos_c);
                if piece != Piece::Empty {
                    if piece_colour != colour {
                        valid_moves.push((pos_r, pos_c));
                    }
                    break;
                }
                valid_moves.push((pos_r, pos_c));
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
