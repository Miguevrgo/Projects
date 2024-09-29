use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub dni: String,
    pub age: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    data: BTreeMap<String, Person>,
    file_path: String,
}

impl Database {
    pub fn new(file_path: &str) -> Self {
        Database {
            data: BTreeMap::new(),
            file_path: file_path.to_string(),
        }
    }

    pub fn insert(&mut self, person: Person) {
        self.data.insert(person.dni.clone(), person);
    }

    pub fn get(&self, dni: &str) -> Option<&Person> {
        self.data.get(dni)
    }

    pub fn save(&self) -> std::io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(&self).expect("Error serializing");
        Ok(())
    }
}
