use std::io::{self, Write};

mod database;
mod storage;

use crate::database::{Database, Person};

fn main() {
    let mut db: Database = Database::load("data").unwrap();

    clear_screen();
    println!(
        "╔════════════════════════════╗\n║  Welcome to Oxide Database ║\n╚════════════════════════════╝"
    );
    loop {
        println!("[1] ~ Insert Person");
        println!("[2] ~ Delete Person");
        println!("[3] ~ Find Person");
        println!("[4] ~ Clear Screen");
        println!("[5] ~ Exit");

        let choice = read_input("➤ ");

        match choice.as_str() {
            "1" => insert_person(&mut db),
            "2" => delete_person(&mut db),
            "3" => find_person(&mut db),
            "4" => clear_screen(),
            _ => break,
        };
    }

    db.save().unwrap();
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

fn insert_person(database: &mut Database) {
    io::stdout().flush().unwrap();
    let dni = read_input("DNI: ");
    io::stdout().flush().unwrap();
    let name = read_input("Name: ");
    io::stdout().flush().unwrap();
    let surname = read_input("Surname: ");
    io::stdout().flush().unwrap();
    let age: u32 = read_input("Age: ").parse().expect("Invalid age");

    database.insert(Person {
        dni,
        name,
        surname,
        age,
    });
}

fn delete_person(database: &mut Database) {
    let dni = read_input("Insert DNI to delete: ");
    database.delete(&dni);
}

fn find_person(database: &mut Database) {
    let dni = read_input("DNI: ");
    if let Some(person) = database.get(&dni) {
        println!("{person}");
    } else {
        println!("Person with DNI: {} Not found", dni);
    }
}
