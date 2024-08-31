use crate::{
    buffer::GapBuffer,
    status_bar::StatusBar,
    style::{Token, TokenType},
    syntax,
    terminal::Terminal,
};
use crossterm::event::KeyCode;
use crossterm::style::{Color, StyledContent, Stylize};
use std::io;

pub struct Editor {
    buffer: GapBuffer,
    status_bar: StatusBar,
    cursor_x: u16,
    cursor_y: u16,
    scroll_offset: u16,
}

const MODE_INSERT: usize = 1;
const MODE_NORMAL: usize = 2;

impl Editor {
    pub fn new(filename: &str) -> Result<Self, io::Error> {
        let initial_text = std::fs::read_to_string(filename).unwrap_or_else(|_| String::new());
        let initial_text = Self::parse_text(initial_text);
        let buffer = GapBuffer::from(&initial_text);
        let status_bar = StatusBar::new(filename);
        let (cursor_x, cursor_y) = (
            buffer.cursor_after_last_crlf() as u16,
            buffer.get_num_lines() - 1,
        );
        Terminal::init()?;

        Ok(Self {
            buffer,
            status_bar,
            cursor_x,
            cursor_y,
            scroll_offset: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        Terminal::show_cursor()?;
        loop {
            self.render()?;

            if let Some(key) = Terminal::read_key()? {
                match key {
                    KeyCode::Esc => self.normal_mode()?,
                    KeyCode::Char(ch) => self.insert_char(ch),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Backspace => self.backspace_char(),
                    KeyCode::Delete => self.delete_char(),
                    KeyCode::Enter => self.insert_newline(),
                    KeyCode::Tab => self.insert_tab(),
                    KeyCode::Up => self.scroll_up(),
                    KeyCode::Down => self.scroll_down(),
                    _ => {}
                }
            }
        }
    }

    fn render(&mut self) -> Result<(), io::Error> {
        Terminal::clear_screen()?;
        self.render_text()?;
        self.status_bar.render();
        Terminal::move_cursor_to(self.cursor_x, self.cursor_y)?;
        Ok(())
    }

    fn render_text(&mut self) -> Result<(), io::Error> {
        let mut rendered_text = String::new();
        let lines = self.buffer.get_lines();
        let term_height = Terminal::size()?.1 - 1;
        let start_line = self.scroll_offset as usize;
        let end_line = std::cmp::min((self.scroll_offset + term_height) as usize, lines.len());

        if start_line < lines.len() {
            for line in &lines[start_line..end_line] {
                let tokens = syntax::tokenize(line);

                for token in tokens {
                    let styled_token = Self::style_token(&token);
                    rendered_text.push_str(&styled_token.to_string());
                }

                rendered_text.push_str("\r\n");
            }
        }

        Terminal::render_text(&rendered_text)?;
        Ok(())
    }

    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    fn scroll_down(&mut self) {
        if self.scroll_offset < self.buffer.get_num_lines() - Terminal::size().unwrap().1 {
            self.scroll_offset += 1;
        }
    }

    fn style_token(token: &Token) -> StyledContent<&str> {
        match token.token_type {
            TokenType::Keyword => token.text.as_str().with(Color::Magenta),
            TokenType::Comment => token.text.as_str().with(Color::Rgb {
                r: 92,
                g: 99,
                b: 112,
            }),
            TokenType::StringLiteral => token.text.as_str().with(Color::Rgb {
                r: 142,
                g: 183,
                b: 115,
            }),
            TokenType::Function => token.text.as_str().with(Color::Rgb {
                r: 97,
                g: 175,
                b: 239,
            }),
            TokenType::Normal => token.text.as_str().with(Color::Rgb {
                r: 171,
                g: 178,
                b: 191,
            }),
        }
    }

    fn insert_tab(&mut self) {
        self.buffer.insert_text("    ");
        self.cursor_x += 4;
    }

    fn normal_mode(&mut self) -> Result<(), io::Error> {
        self.update_status_bar(MODE_NORMAL)?;

        loop {
            self.render()?;

            if let Some(key) = Terminal::read_key()? {
                match key {
                    KeyCode::Char('i') => {
                        self.update_status_bar(MODE_INSERT)?;
                        break;
                    }
                    KeyCode::Char(':') => self.handle_commmand()?,
                    KeyCode::Char('h') => self.move_cursor_left(),
                    KeyCode::Char('l') => self.move_cursor_right(),
                    KeyCode::Char('j') => {
                        if self.cursor_y < self.buffer.get_num_lines() - 1 {
                            self.cursor_x = self.buffer.cursor_down(self.cursor_x);
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
                        self.buffer.cursor_to_line_end();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_commmand(&mut self) -> Result<(), io::Error> {
        let mut small_command = String::new();
        loop {
            let (box_x, box_y) = Terminal::print_command_box(&small_command)?;
            Terminal::move_cursor_to(box_x + small_command.len() as u16, box_y)?;
            if let Some(key) = Terminal::read_key()? {
                match key {
                    KeyCode::Enter => break,
                    KeyCode::Char(ch) => {
                        small_command.push(ch);
                    }
                    KeyCode::Backspace => {
                        small_command.pop();
                    }
                    _ => {}
                }
            }
        }

        Terminal::move_cursor_to(self.cursor_x, self.cursor_y)?;

        match small_command.bytes().next() {
            Some(b'w') => {
                self.save_file()?;
            }
            Some(b'q') => {
                self.exit()?;
                std::process::exit(0);
            }
            Some(b'd') => {
                self.delete_file()?;
                self.exit()?;
                std::process::exit(0);
            }
            Some(b'\x1b') => {}
            _ => {}
        }
        Ok(())
    }

    fn save_file(&self) -> Result<(), io::Error> {
        let content = self.buffer.get_lines();
        let content = content.join("\n");

        std::fs::write(self.status_bar.get_filename(), content)?;
        Ok(())
    }

    fn delete_file(&self) -> Result<(), io::Error> {
        std::fs::remove_file(self.status_bar.get_filename())?;
        Ok(())
    }

    pub fn exit(&self) -> Result<(), io::Error> {
        Terminal::restore()?;
        Ok(())
    }

    fn insert_char(&mut self, ch: char) {
        self.buffer.insert_char(ch);
        self.cursor_x += 1;
    }

    fn update_status_bar(&mut self, mode: usize) -> Result<(), io::Error> {
        let mode_str = match mode {
            MODE_INSERT => "INSERT",
            MODE_NORMAL => "NORMAL",
            _ => "UNKNOWN",
        };

        self.status_bar.update(mode_str);
        Ok(())
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
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_char('\r');
        self.buffer.insert_char('\n');

        self.cursor_x = 0;
        self.cursor_y += 1;
    }

    pub fn parse_text(text: String) -> String {
        let mut bytes = Vec::with_capacity(text.len() * 2);
        for byte in text.bytes() {
            if byte == b'\n' {
                bytes.push(b'\r');
            }
            bytes.push(byte);
        }
        String::from_utf8(bytes).unwrap()
    }
}