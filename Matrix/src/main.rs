use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, poll};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use rand::RngExt;
use std::io::{self, Stdout, Write};
use std::time::Duration;

const MAX_COLS: usize = 512;
const GREEN: &str = "\x1b[0;38;5;46m";
const CHARS: &[u8] = b"0123456789ABEFGJKLMNOPQRSTUVWXYZabcfgjklmnopqrstuvwxyz@#$%&*+=<>~:;{}[]|/";

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    write!(stdout, "\x1b[?25l\x1b[2J{GREEN}")?;
    run_matrix_loop(&mut stdout)?;
    write!(stdout, "\x1b[?25h\x1b[2J\x1b[1;1H\x1b[0m")?;
    disable_raw_mode()
}

fn run_matrix_loop(stdout: &mut Stdout) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;
    let mut current_row = [0; MAX_COLS];
    let mut current_row_streak = [0; MAX_COLS];
    let mut rng = rand::rng();

    loop {
        for _ in 0..rows {
            for col in 0..cols as usize {
                if current_row[col] >= rows - 1 {
                    for row in 0..rows {
                        write!(stdout, "{} ", MoveTo(col as u16, row))?;
                    }
                    current_row[col] = 0;
                    current_row_streak[col] = 0;
                }

                if current_row[col] != 0 || rng.random_bool(0.05) {
                    if current_row_streak[col] == 0 && rng.random_bool(0.1) || current_row[col] == 0
                    {
                        current_row_streak[col] = rng.random_range(0..(rows >> 2));
                    }

                    let char = CHARS[rng.random_range(0..CHARS.len())] as char;
                    current_row[col] += 1;

                    if current_row_streak[col] > 0 {
                        write!(stdout, "{}{char}", MoveTo(col as u16, current_row[col]))?;
                        current_row_streak[col] -= 1;
                    }
                }
            }
            stdout.flush()?;

            if poll(Duration::from_millis(50))?
                && let crossterm::event::Event::Key(k) = crossterm::event::read()?
                && k.code == KeyCode::Char('c')
            {
                return Ok(());
            }
        }
    }
}
