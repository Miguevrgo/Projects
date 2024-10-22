use crate::table::*; // TODO: Refactor so that it is not necessary when dealing with Database
use std::io::Write;
use std::process::exit;

mod cache;
mod database;
mod log;
mod table;

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

fn parse_commmand(command: &str) -> Result<(), String> {
    match command {
        ".exit" => {
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
fn main() {
    clear_screen();
    println!(
    "╔════════════════════════════╗\n║  Welcome to Oxide Database ║\n╚════════════════════════════╝"
);
    let mut current_table = Table::new("Table1");

    loop {
        let choice = read_input("➤ ");

        if choice.starts_with('.') {
            match parse_commmand(&choice) {
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
