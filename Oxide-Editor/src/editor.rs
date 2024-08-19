use crate::{buffer::GapBuffer, status_bar::StatusBar, terminal::Terminal};
use crossterm::event::KeyCode;

pub struct Editor {
    buffer: GapBuffer,
    status_bar: StatusBar,
}

impl Editor {
    pub fn new(filename: &str, initial_text: &str) -> Self {
        let buffer = GapBuffer::from_string(initial_text);
        let status_bar = StatusBar::new(filename);
        Terminal::init();

        Self { buffer, status_bar }
    }

    pub fn run(&mut self) {
        loop {
            Terminal::clear_screen();
            Terminal::render_text(&self.buffer.extract_text());
            self.status_bar.render();

            if let Some(key) = Terminal::read_key() {
                match key {
                    KeyCode::Esc => break,
                    KeyCode::Char(ch) => {
                        self.buffer.insert_char(ch);
                        self.status_bar.update("INSERT");
                    }
                    KeyCode::Left => {
                        self.buffer.cursor_left();
                        self.status_bar.update("NORMAL");
                    }
                    KeyCode::Right => {
                        self.buffer.cursor_right();
                        self.status_bar.update("NORMAL");
                    }
                    KeyCode::Backspace => {
                        self.buffer.backspace();
                        self.status_bar.update("INSERT");
                    }
                    KeyCode::Delete => {
                        self.buffer.delete();
                        self.status_bar.update("INSERT");
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn exit(&self) {
        Terminal::restore();
    }
}
