use std::fmt::{self, format};
use std::process::Output;
use std::{io::Write, process::exit};

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 64;
const PAGE_SIZE: usize = 4096;
const ROW_SIZE: usize = std::mem::size_of::<Row>();
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const ID_SIZE: usize = std::mem::size_of::<u32>();
const USERNAME_SIZE: usize = std::mem::size_of::<[u8; COLUMN_USERNAME_SIZE]>();
const EMAIL_SIZE: usize = std::mem::size_of::<[u8; COLUMN_EMAIL_SIZE]>();

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    s_type: StatementType,
    row: Row,
}

struct Row {
    id: u32,
    username: [u8; COLUMN_USERNAME_SIZE],
    email: [u8; COLUMN_EMAIL_SIZE],
}

impl Row {
    fn new() -> Self {
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

struct Page {
    content: [u8; PAGE_SIZE],
}

impl Page {
    fn new() -> Self {
        Page {
            content: [0; PAGE_SIZE],
        }
    }

    fn content(&self) -> &[u8] {
        &self.content
    }
}

struct Table {
    file: String,
    num_rows: usize,
    pages: Vec<Option<Page>>,
}

impl Table {
    fn new(name: &str) -> Self {
        Table {
            file: name.to_string(),
            num_rows: 0,
            pages: Vec::new(),
        }
    }

    fn row_slot(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        while self.pages.len() <= page_num {
            self.pages.push(None);
        }

        if self.pages[page_num].is_none() {
            self.pages[page_num] = Some(Page::new());
        }

        self.pages[page_num].as_mut().unwrap().content[byte_offset..byte_offset + ROW_SIZE].as_mut()
    }

    fn serialize_row(&mut self, row: &Row) {
        let slot = self.row_slot(self.num_rows);
        slot[0..ID_SIZE].copy_from_slice(&row.id.to_le_bytes());
        slot[ID_SIZE..ID_SIZE + USERNAME_SIZE].copy_from_slice(&row.username);
        slot[ID_SIZE + USERNAME_SIZE..ID_SIZE + USERNAME_SIZE + EMAIL_SIZE]
            .copy_from_slice(&row.email);
    }

    fn deserialize_row(&mut self, row_num: usize) -> Row {
        let slot = self.row_slot(row_num);

        let mut id_bytes = [0u8; ID_SIZE];
        id_bytes.copy_from_slice(&slot[..ID_SIZE]);
        let id = u32::from_le_bytes(id_bytes);

        let mut username = [0u8; COLUMN_USERNAME_SIZE];
        username.copy_from_slice(&slot[ID_SIZE..ID_SIZE + COLUMN_USERNAME_SIZE]);

        let mut email = [0u8; COLUMN_EMAIL_SIZE];
        email.copy_from_slice(
            &slot[ID_SIZE + COLUMN_USERNAME_SIZE
                ..ID_SIZE + COLUMN_USERNAME_SIZE + COLUMN_EMAIL_SIZE],
        );

        Row {
            id,
            username,
            email,
        }
    }

    fn execute_statement(&mut self, statement: &Statement) {
        match statement.s_type {
            StatementType::Insert => {
                let row_to_insert = &statement.row;

                self.serialize_row(row_to_insert);
                self.num_rows += 1;
            }
            StatementType::Select => {
                let mut row: Row;
                for row_num in 0..self.num_rows {
                    row = self.deserialize_row(row_num);
                    println!("{row}");
                }
            }
        }
    }

    fn write_to_file(&mut self) -> std::io::Result<()> {
        let path = self.file.clone();

        let mut output = std::fs::File::create(path).expect("Error opening file: {path}");
        let mut buffer: Vec<u8> = Vec::new();
        for page in self.pages.iter().flatten() {
            buffer.extend_from_slice(page.content());
        }

        output.write_all(&buffer)
    }
}

fn main() {
    clear_screen();
    println!(
        "╔════════════════════════════╗\n║  Welcome to Oxide Database ║\n╚════════════════════════════╝"
    );
    let mut current_table = Table::new("Table1");
    loop {
        let choice = read_input("➤ ");

        if choice.starts_with('.') {
            match parse_commmand(&choice, &mut current_table) {
                Ok(_) => continue,
                Err(err_msg) => {
                    println!("{err_msg}");
                    continue;
                }
            }
        }

        match prepare_statement(&choice) {
            Ok(statement) => current_table.execute_statement(&statement),
            Err(err) => {
                println!("{err}");
            }
        }
    }
}

fn parse_commmand(command: &str, table: &mut Table) -> Result<(), String> {
    match command {
        ".exit" => {
            table.write_to_file().unwrap();
            exit(0);
        }
        _ => Err(format!("Error: unrecognized command '{command}'")),
    }
}

fn prepare_statement(input: &str) -> Result<Statement, String> {
    let mut statement = Statement {
        s_type: StatementType::Select,
        row: Row::new(),
    };

    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err("No command provided".to_string());
    }

    match parts[0] {
        "insert" => {
            statement.s_type = StatementType::Insert;

            if parts.len() != 4 {
                return Err("Syntax error. Expected: insert <id> <username> <email>".to_string());
            }

            let id = parts[1]
                .parse::<u32>()
                .map_err(|_| "Invalid ID".to_string())?;
            if parts[2].len() > COLUMN_USERNAME_SIZE {
                return Err(format!(
                    "Error: Username is too long, max length is {COLUMN_USERNAME_SIZE}",
                ));
            }
            let mut username = [0; COLUMN_USERNAME_SIZE];
            if parts[3].len() > COLUMN_EMAIL_SIZE {
                return Err(format!(
                    "Error: Email is too long, max length is {COLUMN_EMAIL_SIZE}",
                ));
            }
            let mut email = [0; COLUMN_EMAIL_SIZE];
            username[..parts[2].len()].copy_from_slice(parts[2].as_bytes());
            email[..parts[3].len()].copy_from_slice(parts[3].as_bytes());

            statement.row = Row {
                id,
                username,
                email,
            };
            Ok(statement)
        }
        "select" => {
            statement.s_type = StatementType::Select;
            Ok(statement)
        }
        _ => Err(format!("Error: unrecognized keyword at start of '{input}'")),
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    std::io::stdout().flush().unwrap();
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap();
    option.trim().to_string()
}
