use super::piece::{Colour, Piece};
use crate::game::board::Board;
use crate::game::directions::Direction;
use crate::game::moves::Move;

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    New,
    InProgress,
    Paused,
    Over(Option<Colour>), // Winner, if any
}

pub struct Game {
    turn: u16, // Despite 5899 being the maximum number of moves possible
    board: Board,
    log: Vec<Move>,
    white_king_pos: (usize, usize),
    black_king_pos: (usize, usize),
    is_white_check: bool,
    is_black_check: bool,
    state: GameState,
}

impl Game {
    /// Creates a new game of chess, with a default board, empty log and white
    /// to move, it also sets both kings out of check
    pub fn new() -> Self {
        Game {
            turn: 1,
            board: Board::new(),
            log: Vec::new(),
            white_king_pos: (0, 4),
            black_king_pos: (7, 4),
            is_white_check: false,
            is_black_check: false,
            state: GameState::New,
        }
    }

    /// Draws and handles movements of a game of chess until the game is over, or
    /// paused, when it is over, it shows the game result
    pub fn play(&mut self) {
        if self.state == GameState::New {
            self.state = GameState::InProgress;
        }

        self.board.draw();
        while self.state == GameState::InProgress {
            if !self.next_move() {
                self.state = GameState::Paused;
                return;
            }
            self.board.draw();
        }

        self.show_game_result();
    }

    /// Gets the next desired move as an input from the keyboard, in order for
    /// a player to move, first a square has to be selected, then another set
    /// of inputs determines where the player wants to move, if the move is not
    /// valid, this method loops until a valid move is found, returns false if
    /// players want to pause
    pub fn next_move(&mut self) -> bool {
        if self.is_checkmate(self.current_colour()) {
            self.end_game(self.opponent_colour());
            return true;
        }

        let dir = match Direction::input_key() {
            Some(dir) => dir,
            None => return false, // Pause
        };

        match dir {
            Direction::Select => self.handle_selection(),
            _ => self.board.move_cursor(&dir),
        }

        true
    }

    fn handle_selection(&mut self) {
        match self.board.selected {
            None => {
                let (row, col) = self.board.cursor;
                if self.board.get_piece(row, col).0 == self.current_colour() {
                    self.board.selected = Some((row, col));
                }
            }

            Some((row, col)) => {
                let (new_row, new_col) = self.board.cursor;
                if self.can_move(row, col, new_row, new_col) {
                    self.board.selected = None;
                    self.turn += 1;
                } else {
                    self.board.selected = None;
                }
            }
        }
    }

    fn can_move(&mut self, row: usize, col: usize, new_row: usize, new_col: usize) -> bool {
        if !self.valid_move(row, col, new_row, new_col) {
            return false;
        }

        let (colour, piece) = self.board.get_piece(row, col);
        let was_empty = self.board.get_piece(new_row, new_col).1 == Piece::Empty;

        if piece == Piece::Pawn {
            self.handle_pawn_special_moves(row, col, new_row, new_col, colour, was_empty);
        } else if piece == Piece::King {
            self.update_king_position(colour, new_row, new_col);
        }

        self.log_movement(row, col, new_row, new_col);
        self.update_opponent_check();

        true
    }

    fn handle_pawn_special_moves(
        &mut self,
        row: usize,
        col: usize,
        new_row: usize,
        new_col: usize,
        colour: Colour,
        was_empty: bool,
    ) {
        if was_empty && row != new_row && col != new_col {
            self.board
                .set_piece(row, new_col, Colour::White, Piece::Empty);
        }

        if (colour == Colour::White && new_row == 7) || (colour == Colour::Black && new_row == 0) {
            self.promote(new_row, new_col, colour);
        }
    }

    //TODO: Not just a queen
    fn promote(&mut self, new_row: usize, new_col: usize, colour: Colour) {
        self.board.set_piece(new_row, new_col, colour, Piece::Queen);
    }

    fn current_colour(&self) -> Colour {
        if self.turn % 2 == 1 {
            Colour::White
        } else {
            Colour::Black
        }
    }

    fn opponent_colour(&self) -> Colour {
        if self.turn % 2 == 1 {
            Colour::White
        } else {
            Colour::Black
        }
    }

    /// Returns whether or not a move is valid based on the piece to move and the new
    /// position where it wants to move, to do so, it tests if movement is of the player
    /// whose turn it is and if its piece can move that way in the context of the game
    fn valid_move(&mut self, row: usize, col: usize, new_row: usize, new_col: usize) -> bool {
        let (colour, piece) = self.board.get_piece(row, col);
        let (dest_colour, dest_piece) = self.board.get_piece(new_row, new_col);
        let colour_turn = self.current_colour();

        if colour != colour_turn
            || (dest_piece != Piece::Empty && dest_colour == colour)
            || piece == Piece::Empty
        {
            return false;
        }

        let valid_moves = self.get_valid_moves(row, col, colour, piece);
        valid_moves.contains(&(new_row, new_col))
    }

    fn get_valid_moves(
        &mut self,
        row: usize,
        col: usize,
        colour: Colour,
        piece: Piece,
    ) -> Vec<(usize, usize)> {
        match piece {
            Piece::Pawn => Self::pawn_valid_moves(self, row, col, colour),
            Piece::Rook => Self::rook_valid_moves(self, row, col, colour),
            Piece::Bishop => Self::bishop_valid_moves(self, row, col, colour),
            Piece::Knight => Self::knight_valid_moves(self, row, col, colour),
            Piece::Queen => Self::queen_valid_moves(self, row, col, colour),
            Piece::King => Self::king_valid_moves(self, row, col, colour),
            _ => Vec::new(),
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

    fn show_game_result(&self) {
        println!("\x1B[2J\x1B[1;1H"); // Clear screen
        crossterm::terminal::disable_raw_mode().unwrap();
        match self.state {
            GameState::Over(Some(Colour::White)) => println!("Checkmate! White wins!"),
            GameState::Over(Some(Colour::Black)) => println!("Checkmate! Black wins!"),
            GameState::Over(None) => println!("Game ended ~ Draw"),
            _ => unreachable!(),
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

    /// Sets game state to Over and corresponding winner
    fn end_game(&mut self, winner: Colour) {
        self.state = GameState::Over(Some(winner))
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

    fn would_cause_check(
        &mut self,
        row: usize,
        col: usize,
        new_row: usize,
        new_col: usize,
    ) -> bool {
        let (original_colour, original_piece) = self.board.get_piece(new_row, new_col);
        self.board.move_piece(row, col, new_row, new_col);

        let king_pos = if self.current_colour() == Colour::White {
            self.white_king_pos
        } else {
            self.black_king_pos
        };
        let is_checked = self.is_square_under_attack(king_pos.0, king_pos.1, self.current_colour());

        self.board.move_piece(new_row, new_col, row, col);
        self.board
            .set_piece(new_row, new_col, original_colour, original_piece);
        is_checked
    }

    fn is_checkmate(&mut self, colour: Colour) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                let (piece_colour, piece) = self.board.get_piece(row, col);
                if piece_colour == colour && piece != Piece::Empty {
                    let moves = self.get_valid_moves(row, col, colour, piece);
                    if moves
                        .iter()
                        .any(|&(r, c)| !self.would_cause_check(row, col, r, c))
                    {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn update_opponent_check(&mut self) {
        let opponent_colour = self.opponent_colour();
        let (king_row, king_col) = if opponent_colour == Colour::White {
            self.white_king_pos
        } else {
            self.black_king_pos
        };

        let is_checked = self.is_square_under_attack(king_row, king_col, opponent_colour);

        if opponent_colour == Colour::White {
            self.is_white_check = is_checked;
        } else {
            self.is_black_check = is_checked;
        }
    }

    fn update_king_position(&mut self, colour: Colour, row: usize, col: usize) {
        match colour {
            Colour::White => self.white_king_pos = (row, col),
            Colour::Black => self.black_king_pos = (row, col),
        }
    }

    /// Checks if the given position is under attack, colour represents the colour of the piece
    /// whose current situation wants to be known
    fn is_square_under_attack(&mut self, row: usize, col: usize, colour: Colour) -> bool {
        for r in 0..8 {
            for c in 0..8 {
                let (piece_colour, piece) = self.board.get_piece(r, c);
                if piece_colour != colour && piece != Piece::Empty {
                    let moves = self.get_valid_moves(r, c, piece_colour, piece);
                    if moves.contains(&(row, col)) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
