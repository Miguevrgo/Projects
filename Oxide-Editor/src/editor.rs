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
        let buffer = GapBuffer::from_string(initial_text);
        let status_bar = StatusBar::new(filename);
        let (cursor_x, cursor_y) = buffer.get_cursor_position();
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
        self.buffer.cursor_left();
        self.update_status_bar(MODE_NORMAL);
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        let (cursor_x, cursor_y) = self.buffer.get_cursor_position();
        let lines = self.buffer.get_lines();
        if cursor_y as usize >= lines.len() || cursor_x as usize >= lines[cursor_y as usize].len() {
            return;
        }
        self.buffer.cursor_right();
        self.update_status_bar(MODE_NORMAL);
        self.cursor_x += 1;
    }

    fn backspace_char(&mut self) {
        self.buffer.backspace();
        self.update_status_bar(MODE_INSERT);
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    fn delete_char(&mut self) {
        self.buffer.delete();
        self.update_status_bar(MODE_INSERT);
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_char('\n');

        self.update_status_bar(MODE_INSERT);
        self.cursor_x = 0;
        self.cursor_y += 1;
    }

    fn show_numbers(&mut self) {
        let lines = self.buffer.get_lines();

        for (i, line) in lines.iter().enumerate() {
            Terminal::render_text(&format!("{:>4} {}", i + 1, line));
        }
    }

    pub fn exit(&self) {
        Terminal::restore();
    }
}
