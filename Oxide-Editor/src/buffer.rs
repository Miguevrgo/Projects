use core::cmp::max;
const MIN_BUF_SIZE: usize = 32;

pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize, // gap_start
    gap_end: usize,
}

impl GapBuffer {
    pub fn from(text: &str) -> Self {
        let text_len = text.chars().count();
        let buffer_size = max(text_len * 2, MIN_BUF_SIZE);
        let mut buffer = Vec::with_capacity(buffer_size);
        buffer.extend(text.chars());
        buffer.resize(buffer_size, '\0');

        GapBuffer {
            buffer,
            gap_start: text_len,
            gap_end: buffer_size,
        }
    }

    fn gb_front(&self) -> usize {
        self.gap_start
    }

    fn gb_back(&self) -> usize {
        self.buffer.len() - self.gap_end
    }

    fn gb_used(&self) -> usize {
        self.gb_front() + self.gb_back()
    }

    fn move_backtext(&mut self, new_size: usize) {
        let back_text_len = self.gb_back();
        let start = self.buffer.len() - back_text_len;
        let end = self.buffer.len();
        let new_start = new_size - back_text_len;

        self.buffer.copy_within(start..end, new_start);
    }

    fn shrink_buffer(&mut self, new_size: usize) {
        let new_size = max(new_size, MIN_BUF_SIZE);
        if new_size < self.gb_used() {
            return;
        }

        self.move_backtext(new_size);
        self.gap_end = new_size - self.gb_back();
        self.buffer.resize(new_size, '\0');
    }

    fn grow_buffer(&mut self) {
        let new_size = self.buffer.len() * 2;
        let mut new_buffer = vec!['\0'; new_size];

        new_buffer[0..self.gap_start].copy_from_slice(&self.buffer[0..self.gap_start]);
        let new_gap_end = new_size - (self.buffer.len() - self.gap_end);
        new_buffer[new_gap_end..].copy_from_slice(&self.buffer[self.gap_end..]);

        self.gap_end = new_gap_end;
        self.buffer = new_buffer;
    }

    pub fn insert_char(&mut self, c: char) {
        if self.gap_start == self.gap_end {
            self.grow_buffer();
        }
        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
    }

    pub fn cursor_left(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.gap_end -= 1;
            self.buffer[self.gap_end] = self.buffer[self.gap_start];
        }
    }

    pub fn cursor_right(&mut self) {
        if self.gap_end < self.buffer.len() {
            self.buffer[self.gap_start] = self.buffer[self.gap_end];
            self.gap_start += 1;
            self.gap_end += 1;
        }
    }

    pub fn cursor_to_line_start(&mut self) {
        let last_crlf_pos = self.cursor_after_last_crlf();
        for _ in 1..last_crlf_pos {
            self.cursor_left();
        }
    }

    pub fn cursor_up(&mut self, cursor_x: u16) -> u16 {
        let last_crlf_pos = self.cursor_after_last_crlf();
        for _ in 0..last_crlf_pos {
            self.cursor_left();
        }

        self.cursor_left();
        self.cursor_left();

        let previous_line = self.cursor_after_last_crlf();
        for _ in 0..previous_line {
            self.cursor_left();
        }

        for i in 0..cursor_x {
            self.cursor_right();
            if self.buffer[self.gap_start] == '\r' {
                return i + 1;
            }
        }

        cursor_x
    }

    pub fn backspace(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }

        if self.gb_used() < self.buffer.len() / 4 {
            self.shrink_buffer(self.buffer.len() / 2);
        }
    }

    pub fn delete(&mut self) {
        if self.gap_end < self.buffer.len() {
            self.gap_end += 1;
        }

        if self.gb_used() < self.buffer.len() / 4 {
            self.shrink_buffer(self.buffer.len() / 2);
        }
    }

    pub fn extract_text(&self) -> String {
        let front_text: String = self.buffer[0..self.gap_start].iter().collect();
        let back_text: String = self.buffer[self.gap_end..].iter().collect();
        let back_text = back_text.replace('\0', "").clone();

        format!("{}{}", front_text, back_text)
    }

    pub fn get_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut was_cr = false;

        for (i, &c) in self.buffer.iter().enumerate() {
            if i >= self.gap_start && i < self.gap_end {
                continue;
            }

            current_line.push(c);

            if was_cr && c == '\n' {
                lines.push(current_line.clone());
                current_line.clear();
            }

            was_cr = c == '\r';
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        // TODO: Improve this
        for line in lines.iter_mut() {
            if line.ends_with("\r\n") {
                line.pop();
                line.pop();
            }
        }

        lines
    }

    pub fn cursor_after_last_crlf(&self) -> usize {
        let mut last_crlf_pos = None;

        for i in 0..self.gap_start {
            if i > 0 && self.buffer[i - 1] == '\r' && self.buffer[i] == '\n' {
                last_crlf_pos = Some(i + 1);
            }
        }

        self.gap_start - last_crlf_pos.unwrap_or(0)
    }

    pub fn get_num_lines(&self) -> u16 {
        let mut num_lines = 1;
        for (i, &c) in self.buffer.iter().enumerate() {
            if i >= self.gap_start && i < self.gap_end {
                continue;
            }
            if c == '\n' {
                num_lines += 1;
            }
        }
        num_lines as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_text() {
        let buffer = GapBuffer::from("Hello World");
        let buffer = buffer.extract_text();
        assert_eq!(buffer, "Hello World");
    }

    #[test]
    fn left_movement() {
        let mut buffer = GapBuffer::from("Hello");
        buffer.cursor_left();
        assert_eq!(buffer.gap_start, 4);
        assert_eq!(buffer.extract_text(), "Hello");
    }

    #[test]
    fn right_movement() {
        let mut buffer = GapBuffer::from("Hello");
        buffer.cursor_right();
        assert_eq!(buffer.gap_start, 5);
        assert_eq!(buffer.extract_text(), "Hello");
    }

    #[test]
    fn left_movement_over_crlf() {
        let mut buffer = GapBuffer::from("Hel\r\nWod");
        assert_eq!(buffer.gap_start, 8);
        buffer.cursor_left();
        assert_eq!(buffer.gap_start, 7);
        buffer.cursor_left();
        assert_eq!(buffer.gap_start, 6);
        buffer.cursor_left();
        assert_eq!(buffer.gap_start, 5);
        buffer.cursor_left();
        assert_eq!(buffer.gap_start, 4);
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_left();
        assert_eq!(buffer.gap_start, 0);

        assert_eq!(buffer.extract_text(), "Hel\r\nWod");
    }

    #[test]
    fn right_movement_over_crlf() {
        let mut buffer = GapBuffer::from("Hel\r\nWod");
        assert_eq!(buffer.gap_start, 8);
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_left();
        buffer.cursor_right();
        assert_eq!(buffer.gap_start, 3);
        buffer.cursor_right();
        assert_eq!(buffer.gap_start, 4);
        buffer.cursor_right();
        assert_eq!(buffer.gap_start, 5);
        buffer.cursor_right();
        buffer.cursor_right();
        buffer.cursor_right();
        assert_eq!(buffer.gap_start, 8);
        buffer.cursor_right();
        assert_eq!(buffer.gap_start, 8);
    }

    #[test]
    fn get_lines() {
        let buffer = GapBuffer::from("Hello\r\nWorld\r\n");
        let lines = buffer.get_lines();
        assert_eq!(lines, vec!["Hello", "World"]);
    }

    #[test]
    fn get_lines_no_crlf() {
        let buffer = GapBuffer::from("Hello\r\n World");
        let lines = buffer.get_lines();
        assert_eq!(lines, vec!["Hello", " World"]);
    }

    #[test]
    fn get_empty_lines() {
        let buffer = GapBuffer::from("");
        let lines = buffer.get_lines();
        assert_eq!(lines, Vec::<String>::new())
    }

    #[test]
    fn get_one_line() {
        let buffer = GapBuffer::from("Hello");
        let lines = buffer.get_lines();
        assert_eq!(lines, vec!["Hello"]);
    }

    #[test]
    fn cursor_position() {
        let buffer = GapBuffer::from("Hello");
        assert_eq!(buffer.cursor_after_last_crlf(), 5);
    }

    #[test]
    fn cursor_position_multiline() {
        let buffer = GapBuffer::from("Hello\r\nWorld");
        assert_eq!(buffer.cursor_after_last_crlf(), 5);
    }

    #[test]
    fn insert_chars() {
        let mut buffer = GapBuffer::from("");
        buffer.insert_char('H');
        buffer.insert_char('e');
        buffer.insert_char('l');
        buffer.insert_char('l');
        buffer.insert_char('o');
        assert_eq!(buffer.extract_text(), "Hello");
        assert_eq!(buffer.gap_start, 5);
        let mut text = Vec::new();
        text.push("Hello");
        for _ in 0..2000 {
            buffer.insert_char('a');
        }
        text.append(&mut vec!["a"; 2000]);
        assert_eq!(buffer.extract_text().chars().count(), 2005);
        assert_eq!(buffer.gap_start, 2005);
    }

    #[test]
    fn remove_chars() {
        let mut buffer = GapBuffer::from("");
        buffer.insert_char('H');
        buffer.insert_char('e');
        buffer.insert_char('l');
        buffer.insert_char('l');
        buffer.insert_char('o');
        assert_eq!(buffer.extract_text(), "Hello");
        assert_eq!(buffer.gap_start, 5);
        let mut text = Vec::new();
        text.push("Hello");
        for _ in 0..2000 {
            buffer.insert_char('a');
        }
        text.append(&mut vec!["a"; 2000]);
        assert_eq!(buffer.extract_text().chars().count(), 2005);
        assert_eq!(buffer.gap_start, 2005);

        for _ in 0..2000 {
            buffer.backspace();
        }
        assert_eq!(buffer.extract_text().chars().count(), 5);
        assert_eq!(buffer.gap_start, 5);
        assert_eq!(buffer.extract_text(), "Hello");
    }

    #[test]
    fn num_lines() {
        let buffer = GapBuffer::from("holas\r\nndo cruel\r\nerro");
        assert_eq!(buffer.get_num_lines(), 3);
    }
}
