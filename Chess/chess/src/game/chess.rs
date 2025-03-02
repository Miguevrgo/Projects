use super::piece::{Colour, Piece};
use crate::game::board::Board;
use crate::game::directions::*;
use crate::game::moves::Move;

pub struct Game {
    pub turn: u16, // Despite 5899 being the maximum number of moves possible
    board: Board,
    log: Vec<Move>,
    is_white_check: bool, // Is white king in check
    is_black_check: bool, // Is dark king in check
    game_over: bool,
    pause: bool,
    winner: Option<Colour>,
}

impl Game {
    /// Creates a new game of chess, with a default board, empty log and white
    /// to move, it also sets both kings out of check
    pub fn new() -> Self {
        Game {
            turn: 1,
            board: Board::new(),
            log: Vec::new(),
            is_white_check: false,
            is_black_check: false,
            game_over: false,
            pause: false,
            winner: None,
        }
    }

    /// Draws and handles movements of a game of chess until the game is over,
    /// when it is, it shows the game result
    pub fn play(&mut self) {
        self.board.draw();
        while !self.game_over {
            if !self.next_move() {
                return;
            }
        }

        self.show_game_result();
    }

    /// Gets the next desired move as an input from the keyboard, in order for
    /// a player to move, first a square has to be selected, then another set
    /// of inputs determines where the player wants to move, if the move is not
    /// valid, this method loops until a valid move is found, returns false if
    /// players want to pause
    pub fn next_move(&mut self) -> bool {
        if self.is_white_check && Self::is_checkmate(self, Colour::White) {
            Self::end_game(self, Colour::Black);
            return true;
        }
        if self.is_black_check && Self::is_checkmate(self, Colour::Black) {
            Self::end_game(self, Colour::White);
            return true;
        }

        let dir = match Direction::input_key() {
            Some(dir) => dir,
            None => return false,
        };
        if dir == Direction::Select {
            if self.board.selected.is_none() {
                self.board.selected = Some(self.board.cursor);
            } else {
                let (row, col) = self.board.selected.unwrap();
                let (new_row, new_col) = self.board.cursor;
                if self.valid_move(row, col, new_row, new_col) {
                    let (colour, piece) = self.board.get_piece(row, col);
                    let was_empty = self.board.get_piece(new_row, new_col).1 == Piece::Empty;
                    self.board.move_piece(row, col, new_row, new_col);
                    if piece == Piece::Pawn {
                        if was_empty && row != new_row && col != new_col {
                            self.board
                                .set_piece(row, new_col, Colour::White, Piece::Empty);
                        }
                        if colour == Colour::White && new_row == 7
                            || colour == Colour::Black && new_row == 0
                        {
                            self.board.set_piece(new_row, new_col, colour, Piece::Queen);
                        }
                    }

                    self.update_opponent_check();
                    self.log_movement(row, col, new_row, new_col);
                    self.turn += 1;
                    self.board.selected = None;
                    self.board.draw();
                    return true;
                } else {
                    self.board.selected = None;
                }
            }
        } else {
            self.board.move_cursor(&dir);
        }
        self.board.draw();
        true
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
            || Self::king_checked(self, row, col, new_row, new_col)
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
            Piece::Knight => {
                Self::knight_valid_moves(self, row, col, colour).contains(&(new_row, new_col))
            }
            Piece::Queen => {
                Self::queen_valid_moves(self, row, col, colour).contains(&(new_row, new_col))
            }
            Piece::King => {
                Self::king_valid_moves(self, row, col, colour).contains(&(new_row, new_col))
            }
            _ => false,
        }
    }

    /// Returns a vector of the possible moves a given pawn in a position can do, these moves
    /// include going one position up or down always, two positions in starting positions and
    /// diagonally if captured piece is of the opposite colour,
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
                    if (diagonal.1 != Piece::Empty && diagonal.0 != colour)
                        || (self.log.last().is_some_and(|m| m.is_en_passant())
                            && self.board.get_piece(row, col + 1).1 == Piece::Pawn)
                    {
                        valid_moves.push((row + 1, col + 1))
                    }
                }
                if col > 0 && row < 7 {
                    let diagonal = self.board.get_piece(row + 1, col - 1);
                    if (diagonal.1 != Piece::Empty && diagonal.0 != colour)
                        || (self.log.last().is_some_and(|m| m.is_en_passant())
                            && self.board.get_piece(row, col - 1).1 == Piece::Pawn)
                    {
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
                    if (diagonal.1 != Piece::Empty && diagonal.0 != colour)
                        || (self.log.last().is_some_and(|m| m.is_en_passant())
                            && self.board.get_piece(row, col + 1).1 == Piece::Pawn)
                    {
                        valid_moves.push((row - 1, col + 1))
                    }
                }
                if col > 0 && row > 0 {
                    let diagonal = self.board.get_piece(row - 1, col - 1);
                    if (diagonal.1 != Piece::Empty && diagonal.0 != colour)
                        || (self.log.last().is_some_and(|m| m.is_en_passant())
                            && self.board.get_piece(row, col - 1).1 == Piece::Pawn)
                    {
                        valid_moves.push((row - 1, col - 1))
                    }
                }
            }
        }

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

    fn knight_valid_moves(
        &mut self,
        row: usize,
        col: usize,
        colour: Colour,
    ) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();

        let jumps = [
            // Top Jumps
            (2, 1),
            (2, -1),
            // Right Jumps
            (1, 2),
            (-1, 2),
            // Bottom Jumps
            (-2, 1),
            (-2, -1),
            // Left Jumps
            (-1, -2),
            (1, -2),
        ];

        for jump in jumps {
            let r = row as isize + jump.0;
            let c = col as isize + jump.1;

            if (0..=7).contains(&r) && (0..=7).contains(&c) {
                let pos_r = r as usize;
                let pos_c = c as usize;

                let (piece_colour, piece) = self.board.get_piece(pos_r, pos_c);
                if piece != Piece::Empty {
                    if piece_colour != colour {
                        valid_moves.push((pos_r, pos_c));
                    }
                } else {
                    valid_moves.push((pos_r, pos_c));
                }
            }
        }

        valid_moves
    }

    fn king_valid_moves(&mut self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();

        let directions = [
            (1, 0),   // Top
            (1, 1),   // Top-Right
            (0, 1),   // Right
            (-1, 1),  // Bottom Right
            (-1, 0),  // Bottom
            (-1, -1), // Bottom Left
            (0, -1),  // Left
            (1, -1),  // Top Left
        ];

        for dir in directions {
            let r = row as isize + dir.0;
            let c = col as isize + dir.1;

            if (0..=7).contains(&r) && (0..=7).contains(&c) {
                let pos_r = r as usize;
                let pos_c = c as usize;

                let (piece_colour, piece) = self.board.get_piece(pos_r, pos_c);
                if piece != Piece::Empty {
                    if piece_colour != colour {
                        valid_moves.push((pos_r, pos_c));
                    }
                } else {
                    valid_moves.push((pos_r, pos_c));
                }
            }
        }

        valid_moves
    }

    fn queen_valid_moves(&mut self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();

        for (dr, dc) in [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ] {
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

    fn is_checkmate(&mut self, colour: Colour) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                let (piece_colour, piece) = self.board.get_piece(row, col);
                if piece_colour == colour {
                    let moves = match piece {
                        Piece::Queen => Self::queen_valid_moves(self, row, col, colour),
                        Piece::Rook => Self::rook_valid_moves(self, row, col, colour),
                        Piece::Bishop => Self::bishop_valid_moves(self, row, col, colour),
                        Piece::Knight => Self::knight_valid_moves(self, row, col, colour),
                        Piece::Pawn => Self::pawn_valid_moves(self, row, col, colour),
                        Piece::King => Self::king_valid_moves(self, row, col, colour),
                        Piece::Empty => Vec::new(),
                    };
                    for valid_move in moves {
                        let (new_row, new_col) = valid_move;
                        if !self.king_checked(row, col, new_row, new_col) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn show_game_result(&self) {
        println!("\x1B[2J\x1B[1;1H"); // Clear screen
        crossterm::terminal::disable_raw_mode().unwrap();
        match self.winner {
            Some(Colour::White) => println!("Checkmate! White wins!"),
            Some(Colour::Black) => println!("Checkmate! Black wins!"),
            None => println!("Game ended"),
        }

        println!("\nGame Log:");
        for played_move in self.log.iter() {
            println!("{played_move}\r")
        }

        println!("\nPress any key to return to menu...");
        crossterm::terminal::enable_raw_mode().unwrap();
        let _ = crossterm::event::read();
        crossterm::terminal::disable_raw_mode().unwrap();
    }

    /// Sets variable game_over to true and winner to the colour of the winner
    fn end_game(&mut self, winner: Colour) {
        self.game_over = true;
        self.winner = Some(winner);
    }

    fn log_movement(&mut self, row: usize, col: usize, new_row: usize, new_col: usize) {
        self.log.push(Move::from(
            self.board.get_piece(new_row, new_col).1,
            row,
            col,
            new_row,
            new_col,
        ));
    }

    fn king_checked(&mut self, row: usize, col: usize, new_row: usize, new_col: usize) -> bool {
        let (original_piece_colour, original_piece) = self.board.get_piece(new_row, new_col);
        self.board.move_piece(row, col, new_row, new_col);

        let king_colour = if self.turn % 2 == 0 {
            Colour::Black
        } else {
            Colour::White
        };
        let (king_row, king_col) = self.find_king(king_colour);

        let is_checked = self.is_square_under_attack(king_row, king_col, king_colour);

        self.board.move_piece(new_row, new_col, row, col);
        self.board
            .set_piece(new_row, new_col, original_piece_colour, original_piece);

        is_checked
    }

    fn update_opponent_check(&mut self) {
        let opponent_colour = if self.turn % 2 == 0 {
            Colour::White
        } else {
            Colour::Black
        };

        let (king_row, king_col) = self.find_king(opponent_colour);

        let is_checked = self.is_square_under_attack(king_row, king_col, opponent_colour);

        if opponent_colour == Colour::White {
            self.is_white_check = is_checked;
        } else {
            self.is_black_check = is_checked;
        }
    }

    /// Returns the position of the given colour king, panics if there is a missing king
    fn find_king(&self, colour: Colour) -> (usize, usize) {
        for row in 0..8 {
            for col in 0..8 {
                let (piece_colour, piece) = self.board.get_piece(row, col);
                if piece == Piece::King && piece_colour == colour {
                    return (row, col);
                }
            }
        }
        panic!("King not found!");
    }

    /// Checks if the given position is under attack, colour represents the colour of the piece
    /// whose current situation wants to be known
    fn is_square_under_attack(&mut self, row: usize, col: usize, colour: Colour) -> bool {
        for r in 0..8 {
            for c in 0..8 {
                let (piece_colour, piece) = self.board.get_piece(r, c);
                if piece_colour != colour && piece != Piece::Empty {
                    let valid_moves = match piece {
                        Piece::Pawn => self.pawn_valid_moves(r, c, piece_colour),
                        Piece::Rook => self.rook_valid_moves(r, c, piece_colour),
                        Piece::Bishop => self.bishop_valid_moves(r, c, piece_colour),
                        Piece::Knight => self.knight_valid_moves(r, c, piece_colour),
                        Piece::Queen => self.queen_valid_moves(r, c, piece_colour),
                        Piece::King => self.king_valid_moves(r, c, piece_colour),
                        _ => Vec::new(),
                    };
                    if valid_moves.contains(&(row, col)) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
