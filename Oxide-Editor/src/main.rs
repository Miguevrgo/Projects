mod buffer;
mod status_bar;
mod terminal;

use buffer::GapBuffer;
use crossterm::event::KeyCode;
use status_bar::StatusBar;
use terminal::Terminal;

fn main() {
    Terminal::init();
    let mut buffer = GapBuffer::from_string("Welcome to Oxide-Editor!");
    let filename = "untitled.txt";
    let mut status_bar = StatusBar::new(filename);

    loop {
        Terminal::clear_screen();

        Terminal::render_text(&buffer.extract_text());

        status_bar.render();

        if let Some(key) = Terminal::read_key() {
            match key {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Char(c) => {
                    buffer.insert_char(c);
                    status_bar.update("INSERT");
                }
                KeyCode::Left => {
                    buffer.cursor_left();
                    status_bar.update("NORMAL");
                }
                KeyCode::Right => {
                    buffer.cursor_right();
                    status_bar.update("NORMAL");
                }
                KeyCode::Backspace => {
                    buffer.backspace();
                    status_bar.update("INSERT");
                }
                KeyCode::Delete => {
                    buffer.delete();
                    status_bar.update("INSERT");
                }
                _ => {}
            }
        }
    }

    // Restaurar la terminal
    Terminal::restore();
}
