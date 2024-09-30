use bincode::Error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub dni: String,
    pub age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Name: {}\n Surname: {}\n DNI: \n {}, Age: {}",
            self.name, self.surname, self.dni, self.age
        )
    }
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

    pub fn delete(&mut self, dni: &str) {
        self.data.remove_entry(dni);
    }

    pub fn get(&self, dni: &str) -> Option<&Person> {
        self.data.get(dni)
    }

    pub fn save(&self) -> std::io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(&self).expect("Error serializing");
        let mut file = File::create(&self.file_path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load(file_path: &str) -> std::io::Result<Database> {
        let mut file = File::open(file_path)?;
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded)?;
        let db: Database = bincode::deserialize(&encoded).expect("Error deserializing");
        Ok(db)
    }
}
