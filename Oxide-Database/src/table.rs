use core::panic;
use std::fmt;
use std::os::unix::prelude::FileExt;

pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 64;
const PAGE_SIZE: usize = 4096;
const ROW_SIZE: usize = std::mem::size_of::<Row>();
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const ID_SIZE: usize = std::mem::size_of::<u32>();
const USERNAME_SIZE: usize = std::mem::size_of::<[u8; COLUMN_USERNAME_SIZE]>();
const EMAIL_SIZE: usize = std::mem::size_of::<[u8; COLUMN_EMAIL_SIZE]>();

pub enum StatementType {
    Insert,
    Select,
}

pub struct Statement {
    pub s_type: StatementType,
    pub row: Row,
}

pub struct Row {
    pub id: u32,
    pub username: [u8; COLUMN_USERNAME_SIZE],
    pub email: [u8; COLUMN_EMAIL_SIZE],
}

impl Row {
    pub fn new() -> Self {
        Row {
            id: 0,
            username: [0; COLUMN_USERNAME_SIZE],
            email: [0; COLUMN_EMAIL_SIZE],
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Row {{ id: {}, username: {}, email: {} }}",
            self.id,
            String::from_utf8_lossy(&self.username),
            String::from_utf8_lossy(&self.email)
        )
    }
}

struct Node {
    key: u32,
    page: usize,
}

struct Page {
    content: [u8; PAGE_SIZE],
}

impl Page {
    fn new() -> Self {
        Page {
            content: [0; PAGE_SIZE],
        }
    }
}

/// Represents a Table of the database, contains the following fields:
/// - index_file: Name of the file containing index of each key in the table
///     implemented as a B-Tree
/// - entries_file: Name of the file containing the data of the table
///     data is organized in fixed-size rows so direct indexing is possible
/// - num_rows: The number of rows in the table
/// - pages: A vector of pages that contains the data of the table //TODO: Delete this and use entries_file
pub struct Table {
    pub name: String,
    index_file: String,
    entries_file: String,
    num_rows: usize,
    index_tree: Vec<Node>,
}

impl Table {
    pub fn new(name: &str) -> Self {
        let mut table = Table {
            name: name.to_string(),
            index_file: name.to_string() + "_index.txt",
            entries_file: name.to_string() + "_data.txt",
            num_rows: 0,
            index_tree: Vec::new(),
        };

        table.load_metadata().unwrap_or(());
        table
    }

    fn load_metadata(&mut self) -> std::io::Result<()> {
        let file = match std::fs::File::open(&self.index_file) {
            Ok(file) => file,
            Err(_) => return Ok(()),
        };

        let mut num_rows_bytes = [0u8; std::mem::size_of::<usize>()];
        file.read_exact_at(&mut num_rows_bytes, 0)?;
        self.num_rows = usize::from_le_bytes(num_rows_bytes);

        Ok(())
    }

    fn deserialize_row(&self, content: &[u8]) -> Row {
        let mut id_bytes = [0u8; ID_SIZE];
        id_bytes.copy_from_slice(&content[..ID_SIZE]);
        let id = u32::from_le_bytes(id_bytes);

        let mut username = [0u8; COLUMN_USERNAME_SIZE];
        username.copy_from_slice(&content[ID_SIZE..ID_SIZE + COLUMN_USERNAME_SIZE]);

        let mut email = [0u8; COLUMN_EMAIL_SIZE];
        email.copy_from_slice(
            &content[ID_SIZE + COLUMN_USERNAME_SIZE
                ..ID_SIZE + COLUMN_USERNAME_SIZE + COLUMN_EMAIL_SIZE],
        );

        Row {
            id,
            username,
            email,
        }
    }

    fn serialize_row(&self, row: &Row) -> [u8; ROW_SIZE] {
        let mut serialized_row: [u8; ROW_SIZE] = [0u8; ROW_SIZE];
        serialized_row[..ID_SIZE].copy_from_slice(&row.id.to_le_bytes());
        serialized_row[ID_SIZE..ID_SIZE + USERNAME_SIZE].copy_from_slice(&row.username);
        serialized_row[ID_SIZE + USERNAME_SIZE..ID_SIZE + USERNAME_SIZE + EMAIL_SIZE]
            .copy_from_slice(&row.email);

        serialized_row
    }

    pub fn execute_statement(&mut self, statement: &Statement) {
        match statement.s_type {
            StatementType::Insert => {
                let row_to_insert = &statement.row;
                self.insert_row(row_to_insert);
            }
            StatementType::Select => {
                let mut row: Row;
                for row_num in 0..self.num_rows {
                    row = self.read_from_offset((ROW_SIZE * row_num) as u64);
                    println!("{row}");
                }
            }
        }
    }

    // TODO: Check for previous key to avoid repetition
    pub fn insert_row(&mut self, row: &Row) {
        let row_to_insert: [u8; ROW_SIZE] = self.serialize_row(row);
        self.write_to_offset((self.num_rows * ROW_SIZE) as u64, &row_to_insert)
            .unwrap();
        self.num_rows += 1;
        self.index_tree.push(Node {
            key: row.id,
            page: self.num_rows / ROWS_PER_PAGE,
        });

        self.save_num_rows().unwrap();
    }

    fn save_num_rows(&self) -> std::io::Result<()> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.index_file)?;

        file.write_at(&self.num_rows.to_le_bytes(), 0)?;
        Ok(())
    }

    fn write_to_offset(&self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        let path = self.entries_file.clone();
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        //TODO: Scan in index for memory empty after delete
        file.write_at(data, offset)?;
        Ok(())
    }

    fn read_from_offset(&self, offset: u64) -> Row {
        let path = self.entries_file.clone();
        let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();

        let mut buf = [0u8; ROW_SIZE];
        match file.read_at(&mut buf, offset) {
            Ok(bytes_read) => {
                if bytes_read < ROW_SIZE {
                    panic!(
                        "Error: Tried to read an incomplete row from file: {}",
                        self.entries_file
                    )
                }
            }
            Err(e) => {
                panic!(
                    "Error trying to read from file {}: {}",
                    self.entries_file, e
                )
            }
        }

        self.deserialize_row(&buf)
    }
}
