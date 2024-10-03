const PAGE_SIZE: usize = 4096;

#[allow(dead_code)] //TODO: Remove
pub struct Page {
    data: [u8; PAGE_SIZE],
}

#[allow(dead_code)] //TODO: Remove
impl Page {
    /// Initializes a page with value 0
    pub fn new() -> Self {
        Page {
            data: [0; PAGE_SIZE],
        }
    }

    /// This method writes the buffer content to the page,
    /// Will panic if there is not enough space //TODO: Overflow pages
    pub fn write(&mut self, offset: usize, buf: &[u8]) {
        if offset + buf.len() < PAGE_SIZE {
            self.data[offset..(offset + buf.len())].copy_from_slice(buf);
        } else {
            panic!("Error: Not enough size within the page for the buffer");
        }
    }

    /// This method reads [`length`] bytes from the [`offset`] provided
    /// Will panic if requested bytes excede page //TODO Overflow pages
    pub fn read(&self, offset: usize, length: usize) -> &[u8] {
        if offset + length < PAGE_SIZE {
            &self.data[offset..(offset + length)]
        } else {
            panic!("Error: Requested data not full in page");
        }
    }
}
