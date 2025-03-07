use super::piece::{Colour, Piece};
use crate::game::board::Board;
use crate::game::directions::Direction;
use crate::game::moves::Move;

/// Represents the possible states of a chess game, used to control flow and UI behavior
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    New,
    InProgress,
    Paused,
    Over(Option<Colour>), // Winner, if any
}

pub struct Game {
    turn: u16, // Despite 5899 being the maximum number of moves possible
    pub board: Board,
    log: Vec<Move>,
    white_king_pos: (usize, usize),
    black_king_pos: (usize, usize),
    state: GameState,
}

impl Game {
    /// Creates a new chess game with the standard starting position.
    ///
    /// - The turn starts at 1 (White's move).
    /// - Kings are initialized at e1 (0, 4) for White and e8 (7, 4) for Black, per standard chess rules.
    /// - The game begins in the `New` state.
    ///
    /// # Returns
    /// A new `Game` instance ready to be played.
    pub fn new() -> Self {
        Game {
            turn: 1,
            board: Board::new(),
            log: Vec::new(),
            white_king_pos: (0, 4),
            black_king_pos: (7, 4),
            state: GameState::New,
        }
    }

    /// Runs the main game loop, rendering the board and processing moves until the game ends or is paused.
    ///
    /// - Transitions the game state from `New` to `InProgress` on the first call.
    /// - Continues until checkmate, stalemate, or a player pauses the game.
    /// - Displays the game result when the game ends.
    ///
    /// # Examples
    /// ```rust
    /// let mut game = Game::new();
    /// game.play();
    /// ```
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
            self.draw();
        }

        self.show_game_result();
    }

    /// Processes the next move based on player input.
    ///
    /// - Checks for checkmate before proceeding.
    /// - Reads keyboard input to move the cursor or select a piece.
    /// - Returns `false` if the player chooses to pause the game, `true` otherwise.
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

    /// Handles piece selection and movement when the player presses the "Select" key.
    ///
    /// - If no piece is selected, selects the piece under the cursor if it belongs to the current player.
    /// - If a piece is already selected, attempts to move it to the cursor's position.
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
                    self.board.move_piece(row, col, new_row, new_col);
                    self.turn += 1;
                }
                self.board.selected = None;
            }
        }
    }

    /// Determines if a move from `(row, col)` to `(new_row, new_col)` is valid and executable.
    ///
    /// - Validates the move based on piece rules and game state.
    /// - Handles special pawn moves (e.g., en passant, promotion) and king position updates.
    /// - Logs the move if successful.
    ///
    /// # Returns
    /// `true` if the move is valid and executed, `false` otherwise.
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

        true
    }

    /// Handles special moves such as en passant and promotion
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

    /// Validates if a move is legal according to chess rules.
    ///
    /// - Ensures the move is made by the current player, does not capture a friendly piece,
    ///   and does not leave the king in check.
    /// - Returns `false` if the move is invalid or illegal.
    fn valid_move(&mut self, row: usize, col: usize, new_row: usize, new_col: usize) -> bool {
        let (colour, piece) = self.board.get_piece(row, col);
        let (dest_colour, dest_piece) = self.board.get_piece(new_row, new_col);
        let colour_turn = self.current_colour();

        if colour != colour_turn
            || (dest_piece != Piece::Empty && dest_colour == colour)
            || piece == Piece::Empty
            || self.would_cause_check(row, col, new_row, new_col)
        {
            return false;
        }

        let valid_moves = self.get_valid_moves(row, col, colour, piece);
        valid_moves.contains(&(new_row, new_col))
    }

    /// Retrieves a list of valid moves for a piece at the given position.
    ///
    /// # Returns
    /// A vector of `(row, col)` tuples representing possible destinations.
    pub fn get_valid_moves(
        &self,
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

    /// Promotes a pawn to a queen when it reaches the opponent's back rank.
    ///
    /// #TODO:
    /// Allow promotion to other pieces (not just Queen).
    fn promote(&mut self, new_row: usize, new_col: usize, colour: Colour) {
        self.board.set_piece(new_row, new_col, colour, Piece::Queen);
    }

    /// Returns the colour of the player whose turn it is.
    fn current_colour(&self) -> Colour {
        if self.turn % 2 == 1 {
            Colour::White
        } else {
            Colour::Black
        }
    }

    /// Returns the colour of the opponent of the current player
    fn opponent_colour(&self) -> Colour {
        if self.turn % 2 == 1 {
            Colour::White
        } else {
            Colour::Black
        }
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

    //// Ends the game and sets the winner.
    fn end_game(&mut self, winner: Colour) {
        self.state = GameState::Over(Some(winner))
    }

    /// Logs a move to the game’s move history.
    fn log_movement(&mut self, row: usize, col: usize, new_row: usize, new_col: usize) {
        self.log.push(Move::from(
            self.board.get_piece(row, col).1,
            row,
            col,
            new_row,
            new_col,
        ));
    }

    /// Checks if a move would put the current player’s king in check.
    fn would_cause_check(
        &mut self,
        row: usize,
        col: usize,
        new_row: usize,
        new_col: usize,
    ) -> bool {
        let (original_colour, original_piece) = self.board.get_piece(new_row, new_col);
        let (_, piece) = self.board.get_piece(row, col);
        self.board.move_piece(row, col, new_row, new_col);

        let (king_row, king_col) = if piece != Piece::King {
            if self.current_colour() == Colour::White {
                self.white_king_pos
            } else {
                self.black_king_pos
            }
        } else {
            (new_row, new_col)
        };
        let is_checked = self.is_square_under_attack(king_row, king_col, self.current_colour());

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

    fn update_king_position(&mut self, colour: Colour, row: usize, col: usize) {
        match colour {
            Colour::White => self.white_king_pos = (row, col),
            Colour::Black => self.black_king_pos = (row, col),
        }
    }

    pub fn draw(&self) {
        self.board.draw();
        println!("Eval: {}", crate::engine::evaluation::evaluate(self))
    }

    /// Checks if the given position is under attack, colour represents the colour of the piece
    /// whose current situation wants to be known
    fn is_square_under_attack(&self, row: usize, col: usize, colour: Colour) -> bool {
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

    /// Returns a vector of the possible moves a given pawn in a position can do, these moves
    /// include going one position up or down always, two positions in starting positions and
    /// diagonally if captured piece is of the opposite colour,
    fn pawn_valid_moves(&self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
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

    fn rook_valid_moves(&self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
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

    fn bishop_valid_moves(&self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
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

    fn knight_valid_moves(&self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
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

    fn king_valid_moves(&self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
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

    fn queen_valid_moves(&self, row: usize, col: usize, colour: Colour) -> Vec<(usize, usize)> {
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
}
