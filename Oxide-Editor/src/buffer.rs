use core::cmp::max;
const MIN_BUF_SIZE: usize = 32;

#[inline]
pub fn saturating_dbl(size: usize) -> usize {
    if size < (usize::MAX / 2) {
        2 * size
    } else {
        usize::MAX
    }
}

pub struct GapBuffer {
    text: Vec<char>,
    gap_start: usize, // Cursor
    gap_end: usize,
    line_indices: Vec<usize>,
}

#[allow(dead_code)]
impl GapBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            text: vec![' '; max(size, MIN_BUF_SIZE)],
            gap_start: 0,
            gap_end: size,
            line_indices: vec![0],
        }
    }

    pub fn update_line_indices(&mut self) {
        self.line_indices.clear();
        self.line_indices.push(0);

        for i in 0..self.gap_start {
            if self.text[i] == '\n' {
                self.line_indices.push(i + 1);
            }
        }

        for i in self.gap_end..self.text.len() {
            if self.text[i] == '\n' {
                self.line_indices.push(i + 1);
            }
        }
    }

    pub fn grow_bufffer(&mut self, mut new_size: usize) {
        new_size = max(new_size, MIN_BUF_SIZE);

        if new_size > self.text.len() {
            let mut new_text = vec![' '; new_size];

            new_text[0..self.gap_start].copy_from_slice(&self.text[..self.gap_start]);
            new_text[new_size - (self.text.len() - self.gap_end)..]
                .copy_from_slice(&self.text[self.gap_end..]);

            self.gap_end = new_size - (self.text.len() - self.gap_end);
            self.text = new_text
        }
    }

    pub fn from_string(s: &str) -> Self {
        let mut buffer = Self::new(s.len() + 16);

        for ch in s.chars() {
            buffer.insert_char(ch);
        }
        buffer
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.gap_start == self.gap_end {
            let new_size = saturating_dbl(self.text.len());
            self.grow_bufffer(new_size)
        }

        self.text[self.gap_start] = ch;
        self.gap_start += 1;

        if ch == '\n' {
            self.line_indices.push(self.gap_start);
            self.line_indices.sort_unstable();
        }
    }

    pub fn cursor_left(&mut self) {
        if self.gap_start > 0 {
            self.gap_end -= 1;
            self.gap_start -= 1;
            self.text[self.gap_end] = self.text[self.gap_start];
        }
    }

    pub fn cursor_right(&mut self) {
        if self.gap_end < self.text.len() {
            self.text[self.gap_start] = self.text[self.gap_end];
            self.gap_start += 1;
            self.gap_end += 1;
        }
    }

    pub fn cursor_up(&mut self) {
        let current_line = self.get_line_number();
        if current_line > 0 {
            self.gap_start = self.line_indices[current_line];
        }
    }

    pub fn cursor_down(&mut self) {
        let current_line = self.get_line_number();
        if current_line < self.line_indices.len() {
            self.gap_start = self.line_indices[current_line];
        }
    }
    // Lets ignore this logic now and just not allow for going up in lines
    // pub fn backspace(&mut self) {
    //      if self.gap_start > 0 {
    //          self.gap_start -= 1;
    //          if self.text[self.gap_start] == '\n' {
    //              let line_idx = self.line_indices.binary_search(&(self.gap_start + 1)).unwrap();
    //              self.line_indices.remove(line_idx);
    //          }
    //      }
    //  }
    //
    //  pub fn delete(&mut self) {
    //      if self.gap_end < self.text.len() {
    //          if self.text[self.gap_end] == '\n' {
    //              let line_idx = self.line_indices.binary_search(&(self.gap_end + 1)).unwrap();
    //              self.line_indices.remove(line_idx);
    //          }
    //          self.gap_end += 1;
    //      }
    //  }
    pub fn backspace(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
        //        todo!("Shrink when len < len/4");
    }

    pub fn delete(&mut self) {
        if self.gap_end < self.text.len() {
            self.gap_end += 1;
        }
        //       todo!("Shrink when len < len/4");
    }

    pub fn get_line_number(&self) -> usize {
        match self.line_indices.binary_search(&self.gap_start) {
            Ok(line) => line,
            Err(line) => line - 1,
        }
    }

    pub fn get_cursor_position(&self) -> (u16, u16) {
        let line_start = self.line_indices[self.get_line_number()];
        let x = self.gap_start - line_start;
        (x as u16, self.get_line_number() as u16)
    }

    pub fn extract_text(&mut self) -> String {
        let mut result = String::new();
        result.extend(&self.text[..self.gap_start]);
        result.extend(&self.text[self.gap_end..]);
        result
    }

    pub fn get_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();

        for i in 0..self.line_indices.len() {
            let start = self.line_indices[i];
            let end = if i + 1 < self.line_indices.len() {
                self.line_indices[i + 1] - 1
            } else {
                self.gap_start + (self.text.len() - self.gap_end)
            };
            let mut line = String::new();
            line.extend(&self.text[start..end]);
            lines.push(line);
        }
        todo!("Fix it");
        lines
    }

    pub fn print_buffer(&mut self) {
        let text = self.extract_text();
        println!("{}", text);
    }
}