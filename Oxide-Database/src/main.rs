use std::io::{self, Write};

mod database;

use crate::database::{Database, Person};

fn main() {
    let mut db: Database = Database::load("data").unwrap();

    print!("\x1B[2J\x1B[1;1H");
    println!(
        "╔════════════════════════════╗\n║  Welcome to Oxide Database ║\n╚════════════════════════════╝"
    );
    loop {
        println!("[1] ~ Insert Person");
        println!("[2] ~ Delete Person");
        println!("[3] ~ Find Person");
        println!("[4] ~ Exit");

        let choice = read_input();

        match choice.as_str() {
            "1" => insert_person(&mut db),
            "2" => delete_person(&mut db),
            "3" => find_person(&mut db),
            _ => break,
        };
    }

    db.save().unwrap();
}

fn read_input() -> String {
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap();
    option.trim().to_string()
}

fn insert_person(database: &mut Database) {
    print!("DNI: ");
    io::stdout().flush().unwrap();
    let dni = read_input();
    print!("Name: ");
    io::stdout().flush().unwrap();
    let name = read_input();
    print!("Surname: ");
    io::stdout().flush().unwrap();
    let surname = read_input();
    print!("Age: ");
    io::stdout().flush().unwrap();
    let age: u32 = read_input().parse().expect("Invalid age");

    database.insert(Person {
        dni,
        name,
        surname,
        age,
    });
}

fn delete_person(database: &mut Database) {
    println!("Insert DNI to delete:");
    let dni = read_input();
    database.delete(&dni);
}

fn find_person(database: &mut Database) {
    println!("DNI: ");
    let dni = read_input();
    if let Some(person) = database.get(&dni) {
        println!("{person}");
    } else {
        println!("Person with DNI: {} Not found", dni);
    }
}
