use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, poll};
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
    let (width, height) = terminal::size()?;
    let mut c_row = [0; MAX_COLS];
    let mut c_row_len = [0; MAX_COLS];
    let mut rng = rand::rng();

    loop {
        for col in 0..width as usize {
            if c_row_len[col] == 0 && rng.random_bool(0.05) {
                c_row_len[col] = rng.random_range(3..(height >> 1) - 1);
                c_row[col] = 0;
            }

            if c_row_len[col] != 0 {
                let ch = CHARS[rng.random_range(0..CHARS.len())] as char;

                if c_row[col] < height {
                    write!(stdout, "{}{ch}", MoveTo(col as u16, c_row[col]))?;
                }

                if c_row[col] >= c_row_len[col] {
                    write!(
                        stdout,
                        "{} ",
                        MoveTo(col as u16, c_row[col] - c_row_len[col])
                    )?;
                }
                c_row[col] += 1;
            }

            if c_row[col] > height + c_row_len[col] {
                c_row_len[col] = 0;
            }
        }

        stdout.flush()?;
        if poll(Duration::from_millis(50))?
            && let Event::Key(k) = crossterm::event::read()?
            && k.code == KeyCode::Char('c')
        {
            return Ok(());
        }
    }
}
