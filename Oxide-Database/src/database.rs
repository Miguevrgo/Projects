use crate::cache::Cache;
use crate::log::Log;
use crate::table::*;

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
}
