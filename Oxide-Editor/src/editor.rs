use crate::{buffer::GapBuffer, status_bar::StatusBar, terminal::Terminal};
use crossterm::event::KeyCode;

pub struct Editor {
    buffer: GapBuffer,
    status_bar: StatusBar,
    cursor_x: u16,
    cursor_y: u16,
}

const MODE_INSERT: usize = 1;
const MODE_NORMAL: usize = 2;

impl Editor {
    pub fn new(filename: &str, initial_text: &str) -> Self {
        let buffer = GapBuffer::from(initial_text);
        let status_bar = StatusBar::new(filename);
        let (cursor_x, cursor_y) = (
            buffer.cursor_after_last_crlf() as u16,
            buffer.get_num_lines() - 1,
        );
        Terminal::init();

        Self {
            buffer,
            status_bar,
            cursor_x,
            cursor_y,
        }
    }

    pub fn run(&mut self) {
        Terminal::show_cursor();
        loop {
            self.render();

            if let Some(key) = Terminal::read_key() {
                match key {
                    KeyCode::Esc => break,
                    KeyCode::Char(ch) => self.insert_char(ch),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Backspace => self.backspace_char(),
                    KeyCode::Delete => self.delete_char(),
                    KeyCode::Enter => self.insert_newline(),
                    _ => {}
                }
                Terminal::move_cursor_to(self.cursor_x, self.cursor_y);
            }
        }
    }

    fn render(&mut self) {
        Terminal::clear_screen();
        Terminal::render_text(&self.buffer.extract_text());
        self.status_bar.render();
        Terminal::move_cursor_to(self.cursor_x, self.cursor_y);
    }

    fn insert_char(&mut self, ch: char) {
        self.buffer.insert_char(ch);
        self.update_status_bar(MODE_INSERT);
        self.cursor_x += 1;
    }

    fn update_status_bar(&mut self, mode: usize) {
        let mode_str = match mode {
            MODE_INSERT => "INSERT",
            MODE_NORMAL => "NORMAL",
            _ => "UNKNOWN",
        };

        self.status_bar.update(mode_str);
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.buffer.cursor_left();
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.buffer.cursor_left();
            self.buffer.cursor_left();
            self.cursor_y -= 1;
            self.cursor_x = self.buffer.get_lines()[self.cursor_y as usize].len() as u16;
        }
        self.update_status_bar(MODE_NORMAL);
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_x < self.buffer.get_lines()[self.cursor_y as usize].len() as u16 {
            self.buffer.cursor_right();
            self.cursor_x += 1;
        } else if self.cursor_y < self.buffer.get_num_lines() - 1 {
            self.buffer.cursor_right();
            self.buffer.cursor_right();
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    fn backspace_char(&mut self) {
        if self.cursor_x > 0 {
            self.buffer.backspace();
            self.update_status_bar(MODE_INSERT);
            self.cursor_x -= 1;
        }
    }

    fn delete_char(&mut self) {
        self.buffer.delete();
        self.update_status_bar(MODE_INSERT);
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_char('\r');
        self.buffer.insert_char('\n');

        self.update_status_bar(MODE_INSERT);
        self.cursor_x = 0;
        self.cursor_y += 1;
    }

    pub fn exit(&self) {
        Terminal::restore();
    }
}
