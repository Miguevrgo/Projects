use crossterm::cursor::{self, MoveTo};
use crossterm::event::{KeyCode, poll};
use crossterm::execute;
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use rand::RngExt;
use std::io::{self, Write};
use std::time::Duration;

const GREEN: &str = "\x1b[1;32m";

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    let columns = terminal::size().unwrap().0;
    let rows = terminal::size().unwrap().1;
    let mut current_row = vec![0; columns as usize];
    let mut current_row_streak = vec![0; columns as usize];
    let mut rng = rand::rng();

    loop {
        for _ in 0..rows {
            for col in 0..columns as usize {
                if current_row[col] >= rows - 1 {
                    for row in 0..rows {
                        execute!(stdout, MoveTo(col as u16, row)).unwrap();
                        print!(" ");
                    }
                    current_row[col] = 0;
                    current_row_streak[col] = 0;
                }

                if current_row[col] != 0 || rng.random_bool(0.05) {
                    execute!(stdout, MoveTo(col as u16, current_row[col])).unwrap();
                    if current_row_streak[col] == 0 && rng.random_bool(0.1) {
                        current_row_streak[col] = rng.random_range(0..10);
                    }

                    let char = (rng.random_range(60..127) as u8) as char;
                    current_row[col] += 1;

                    if current_row_streak[col] > 0 {
                        print!("{GREEN}{char}");
                        current_row_streak[col] -= 1;
                    }
                }
            }
            stdout.flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        if poll(Duration::from_millis(1)).unwrap()
            && let crossterm::event::Event::Key(k) = crossterm::event::read().unwrap()
            && k.code == KeyCode::Char('c')
        {
            break;
        }
    }

    execute!(stdout, cursor::Show).unwrap();
    disable_raw_mode().unwrap();
}
