use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

const PAGE_SIZE: usize = 4096;

#[derive(Serialize, Deserialize)]
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
    pub fn write_to_file(self, path: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        file.write_all(&encoded)?;
        Ok(())
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
