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
    pub fn new(filename: &str) -> Self {
        let initial_text = std::fs::read_to_string(filename).unwrap_or_else(|_| String::new());
        let initial_text = Self::parse_text(initial_text);
        let buffer = GapBuffer::from(&initial_text);
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
                    KeyCode::Esc => self.normal_mode(),
                    KeyCode::Char(ch) => self.insert_char(ch),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Backspace => self.backspace_char(),
                    KeyCode::Delete => self.delete_char(),
                    KeyCode::Enter => self.insert_newline(),
                    KeyCode::Tab => self.insert_tab(),
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

    fn insert_tab(&mut self) {
        self.buffer.insert_char(' ');
        self.buffer.insert_char(' ');
        self.buffer.insert_char(' ');
        self.buffer.insert_char(' ');
        self.cursor_x += 4;
    }

    fn normal_mode(&mut self) {
        self.update_status_bar(MODE_NORMAL);

        loop {
            self.render();

            if let Some(key) = Terminal::read_key() {
                match key {
                    KeyCode::Char('i') => break,
                    KeyCode::Char(':') => self.handle_commmand(),
                    KeyCode::Char('h') => self.move_cursor_left(),
                    KeyCode::Char('l') => self.move_cursor_right(),
                    KeyCode::Char('j') => {
                        if self.cursor_y < self.buffer.get_num_lines() - 1 {
                            self.cursor_y += 1;
                        }
                    }
                    KeyCode::Char('k') => {
                        if self.cursor_y > 0 {
                            self.cursor_x = self.buffer.cursor_up(self.cursor_x);
                            self.cursor_y -= 1;
                        }
                    }
                    KeyCode::Char('x') => self.delete_char(),
                    KeyCode::Char('o') => {
                        self.insert_newline();
                        break;
                    }

                    KeyCode::Char('0') => {
                        self.cursor_x = 0;
                        self.buffer.cursor_to_line_start();
                    }
                    KeyCode::Char('$') => {
                        self.cursor_x =
                            self.buffer.get_lines()[self.cursor_y as usize].len() as u16;
                        //self.buffer.cursor_to_line_end(); //TODO:
                    }
                    _ => {}
                }
                Terminal::move_cursor_to(self.cursor_x, self.cursor_y);
            }
        }
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
        } else if self.cursor_y > 0 {
            self.buffer.backspace();
            self.buffer.backspace();
            self.cursor_y -= 1;
            self.cursor_x = self.buffer.get_lines()[self.cursor_y as usize].len() as u16;
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

    pub fn parse_text(mut text: String) -> String {
        let mut i = 0;
        while i < text.len() {
            if text.as_bytes()[i] == b'\n' {
                text.insert(i, '\r');
                i += 1;
            }
            i += 1;
        }
        text
    }

    fn handle_commmand(&mut self) {
        // TODO: FUlly implement
        let mut small_command = String::new();
        loop {
            self.render();
            Terminal::print_command_box();
            if let Some(key) = Terminal::read_key() {
                match key {
                    KeyCode::Enter => break,
                    KeyCode::Char(ch) => small_command.push(ch),
                    KeyCode::Backspace => {
                        small_command.pop();
                    }
                    _ => {}
                }
            }
        }

        Terminal::move_cursor_to(self.cursor_x, self.cursor_y);

        match small_command.bytes().next() {
            Some(b'w') => {
                self.save_to_file();
            }
            Some(b'q') => {
                self.exit();
                std::process::exit(0);
            }
            // Escape TODO? CHECK NEXT ITERATOR?
            Some(b'\x1b') => {}
            _ => {}
        }
    }

    fn save_to_file(&self) {
        let content = self.buffer.get_lines();
        let content = content.join("\n");

        std::fs::write(self.status_bar.get_filename(), content).unwrap();
    }

    pub fn exit(&self) {
        Terminal::restore();
    }
}
