use database::Database;

use crate::table::*; // TODO: Refactor so that it is not necessary when dealing with Database
use std::io::Write;

mod cache;
mod database;
mod log;
mod table;

const DATA_FILE: &str = "tables.txt";

pub enum CommandType {
    Exit,
    Clear,
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

fn parse_commmand(command: &str) -> Result<CommandType, String> {
    match command {
        ".exit" => Ok(CommandType::Exit),
        ".clear" => Ok(CommandType::Clear),
        _ => Err(format!("Error: unrecognized command: {command}")),
    }
}

fn prepare_statement(input: &str) -> Result<StatementType, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err("No command provided".to_string());
    }

    match parts[0] {
        "insert" => Ok(StatementType::Insert),
        "select" => Ok(StatementType::Select),
        _ => Err(format!("Error: unrecognized keyword at start of '{input}'")),
    }
}

fn main() {
    clear_screen();
    println!(
    "╔════════════════════════════╗\n║  Welcome to Oxide Database ║\n╚════════════════════════════╝"
);
    let mut database = Database::new();
    database.load(DATA_FILE).unwrap();

    loop {
        let choice = read_input("➤ ");

        if choice.starts_with('.') {
            match parse_commmand(&choice) {
                Ok(CommandType::Exit) => {
                    database.save(DATA_FILE).unwrap();
                    std::process::exit(0);
                }
                Ok(CommandType::Clear) => clear_screen(),
                Err(err) => eprintln!("{err}"),
            }
        }

        match prepare_statement(&choice) {
            Ok(StatementType::Insert) => match database.execute(&choice, &StatementType::Insert) {
                Ok(_) => (),
                Err(err) => println!("Execution error: {err}"),
            },
            Ok(StatementType::Select) => match database.execute(&choice, &StatementType::Select) {
                Ok(_) => (),
                Err(err) => println!("Execution error: {err}"),
            },
            Err(err) => println!("Error: {err}"),
        }
    }
}
