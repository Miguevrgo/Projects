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
}

#[allow(dead_code)]
impl GapBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            text: vec![' '; max(size, MIN_BUF_SIZE)],
            gap_start: 0,
            gap_end: size,
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

    pub fn extract_text(&mut self) -> String {
        let mut result = String::new();
        result.extend(&self.text[..self.gap_start]);
        result.extend(&self.text[self.gap_end..]);
        result
    }

    pub fn print_buffer(&mut self) {
        let text = self.extract_text();
        println!("{}", text);
    }
}
