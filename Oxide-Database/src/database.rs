use crate::cache::Cache;
use crate::log::Log;
use crate::table::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Database {
    pub tables: Vec<Table>,
    pub cache: Cache, //TODO: Implement Fixed Sized Cache pages
    pub log: Log,     //TODO Implement Log File
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: Vec::new(),
            cache: Cache::new(),
            log: Log::new(),
        }
    }

    pub fn add_table(&mut self, name: &str) -> Result<(), String> {
        if self.tables.iter().any(|table| table.name == name) {
            return Err(format!("Error: Table {name} already exists"));
        }

        self.tables.push(Table::new(name));
        Ok(())
    }

    pub fn load(&mut self, file: &str) -> std::io::Result<()> {
        let reader = BufReader::new(File::open(file)?);

        for line in reader.lines() {
            for word in line.unwrap().split_whitespace() {
                self.tables.push(Table::new(word))
            }
        }

        Ok(())
    }

    pub fn execute(&mut self, input: &str, statement_type: &StatementType) -> Result<(), String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match statement_type {
            StatementType::Insert => {
                if parts.len() != 5 {
                    return Err(
                        "Syntax error. Expected: insert <table> <id> <username> <email>"
                            .to_string(),
                    );
                }

                let table_name = parts[1];
                let table: &mut Table = match self
                    .tables
                    .iter_mut()
                    .find(|table| table.name == table_name)
                {
                    Some(pos) => pos,
                    None => return Err(format!("Error: Table {table_name} does not exist")),
                };

                let id = parts[2]
                    .parse::<u32>()
                    .map_err(|_| "Invalid ID".to_string())?;
                if parts[3].len() > COLUMN_USERNAME_SIZE {
                    return Err(format!(
                        "Error: Username is too long, max length is {COLUMN_USERNAME_SIZE}",
                    ));
                }
                let mut username = [0; COLUMN_USERNAME_SIZE];
                if parts[4].len() > COLUMN_EMAIL_SIZE {
                    return Err(format!(
                        "Error: Email is too long, max length is {COLUMN_EMAIL_SIZE}",
                    ));
                }
                let mut email = [0; COLUMN_EMAIL_SIZE];
                username[..parts[2].len()].copy_from_slice(parts[2].as_bytes());
                email[..parts[3].len()].copy_from_slice(parts[3].as_bytes());
                //TODO: Find Table and insert_row
                table.insert_row(&Row {
                    id,
                    username,
                    email,
                });
            }
            StatementType::Select => {
                if parts.len() != 2 {
                    return Err("Syntax error. Expected: select <table>".to_string());
                }

                let table_name = parts[1];

                if table.name {} //TODO

                let mut row: Row;
                for row_num in 0..self.num_rows {
                    row = self.read_from_offset((ROW_SIZE * row_num) as u64);
                    println!("{row}");
                }
            }
        }

        Ok(())
    }
}
