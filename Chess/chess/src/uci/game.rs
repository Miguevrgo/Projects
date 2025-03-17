use crate::game::piece::Colour;
use crate::game::square::Square;
use crate::game::{board::Board, moves::Move};
use crate::uci::direction::Direction;
use std::time::{Duration, Instant};

/// Represents a chess game with interactive cursor and time control.
///
/// This struct encapsulates the game state, including the board, cursor, selected square,
/// time for each player, and pause state. It supports time increments per move.
#[derive(Clone, Debug)]
pub struct Game {
    board: Board,             // The current chess board state
    cursor: Square,           // Current cursor position
    selected: Option<Square>, // Currently selected square, if any
    white_time: Duration,     // Remaining time for White
    black_time: Duration,     // Remaining time for Black
    increment: Duration,      // Time increment added after each move
    last_update: Instant,     // Last time the clock was updated
}

impl Game {
    /// Creates a new game with initial time and increment per move.
    ///
    /// # Arguments
    ///
    /// * `initial_time_secs` - Initial time for each player in seconds.
    /// * `increment_secs` - Time increment added after each move in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// let game = Game::new(300, 3); // 5 minutes + 3 seconds/move
    /// ```
    pub fn new(initial_time_secs: u64, increment_secs: u64) -> Self {
        let initial_time = Duration::from_secs(initial_time_secs);
        let increment = Duration::from_secs(increment_secs);
        Game {
            board: Board::default(),
            cursor: Square::from("e2"),
            selected: None,
            white_time: initial_time,
            black_time: initial_time,
            increment,
            last_update: Instant::now(),
        }
    }

    /// Runs the main game loop until time runs out or the game ends.
    pub fn play(&mut self) {
        while self.white_time > Duration::ZERO && self.black_time > Duration::ZERO {
            self.draw();
            if let Some(direction) = Direction::input_key() {
                self.process_input(direction);
            } else {
                std::process::exit(1);
            }
        }
        println!(
            "Time's up! Winner: {}",
            if self.white_time == Duration::ZERO {
                "Black"
            } else {
                "White"
            }
        );
    }

    // Updates the time for the current player based on elapsed time since last update
    fn update_time(&mut self) {
        let elapsed = self.last_update.elapsed();
        self.last_update = Instant::now();
        if self.board.side == Colour::White {
            self.white_time = self.white_time.saturating_sub(elapsed);
        } else {
            self.black_time = self.black_time.saturating_sub(elapsed);
        }
    }

    /// Moves the cursor by the given file and rank deltas.
    ///
    /// Does nothing if the move would take the cursor off the board.
    pub fn move_cursor(&mut self, file_delta: i8, rank_delta: i8) {
        if let Some(new_cursor) = self.cursor.jump(file_delta, rank_delta) {
            self.cursor = new_cursor;
        }
    }

    /// Attempts to move a piece from the selected square to the cursor position.
    ///
    /// If a piece is moved, the turn switches and the increment is added to the player's time.
    pub fn try_move_piece(&mut self) {
        if let Some(src) = self.selected {
            let dest = self.cursor;
            let legal_moves = self.board.generate_legal_moves();
            let move_candidate = legal_moves
                .iter()
                .find(|m| m.get_source() == src && m.get_dest() == dest);

            if let Some(&m) = move_candidate {
                self.board.make_move(m);
                self.update_time();

                self.selected = None;

                if self.board.side == Colour::Black {
                    self.white_time = self.white_time.saturating_add(self.increment);
                } else {
                    self.black_time = self.black_time.saturating_add(self.increment);
                }
            }
        }
    }

    /// Processes a user input direction and updates the game state.
    pub fn process_input(&mut self, direction: Direction) {
        self.update_time();
        match direction {
            Direction::Up => self.move_cursor(0, -1),
            Direction::Down => self.move_cursor(0, 1),
            Direction::Left => self.move_cursor(-1, 0),
            Direction::Right => self.move_cursor(1, 0),
            Direction::Select => {
                if self.selected.is_some() {
                    self.try_move_piece();
                    self.selected = None;
                } else if self.board.piece_at(self.cursor).is_some() {
                    self.selected = Some(self.cursor);
                } else {
                    self.selected = None;
                }
            }
        }
    }

    /// Draws the game board with cursor, selection, and metadata.
    pub fn draw(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("\r  a b c d e f g h\r");
        println!(" ┌────────────────┐\r");

        for row in (0..8).rev() {
            print!("{}│", row + 1);
            for col in 0..8 {
                let square = Square::from_row_col(row, col);
                let is_selected = self.selected == Some(square);
                let is_cursor = self.cursor == square && !is_selected;

                let bg_colour = if (row + col) % 2 == 0 {
                    "\x1b[48;2;181;136;99m" // Dark square
                } else {
                    "\x1b[48;2;240;217;181m" // Light square
                };

                let highlight = if is_selected {
                    "\x1b[102m" // Red for selected
                } else if is_cursor {
                    "\x1b[41m" // Green for cursor
                } else {
                    bg_colour
                };

                match self.board.piece_at(square) {
                    Some(piece) => match piece.colour() {
                        Colour::White => print!("{highlight}\x1b[38;2;255;255;255m{piece} \x1b[0m"),
                        Colour::Black => print!("{highlight}\x1b[38;2;0;0;0m{piece} \x1b[0m"),
                    },
                    None => print!("{highlight}  \x1b[0m"),
                }
            }
            println!("│\r");
        }

        println!(" └────────────────┘\r");
        println!(
            "Turn: {} \n\rWhite Time: {:02.0}:{:02.0} | Black Time: {:02.0}:{:02.0} | Increment: {}s",
            if self.board.side == Colour::White {
                "White"
            } else {
                "Black"
            },
            self.white_time.as_secs() / 60,
            self.white_time.as_secs() % 60,
            self.black_time.as_secs() / 60,
            self.black_time.as_secs() % 60,
            self.increment.as_secs()
        );
    }
}
