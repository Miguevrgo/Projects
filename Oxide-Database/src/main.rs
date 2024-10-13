use std::{io::Write, process::exit};

const COLUMN_USERNAME_SIZE: usize = 64;
const COLUMN_EMAIL_SIZE: usize = 128;

enum StatementType {
    Insert,
    Select,
}

struct Row {
    id: u32,
    username: [u8; COLUMN_USERNAME_SIZE],
    email: [u8; COLUMN_EMAIL_SIZE],
}

struct Table {
    num_rows: u32,
    pages: Vec<Option<Vec<u8>>>,
}

struct Statement {
    s_type: StatementType,
    row: Option<Row>,
}

fn main() {
    clear_screen();
    println!(
        "╔════════════════════════════╗\n║  Welcome to Oxide Database ║\n╚════════════════════════════╝"
    );
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

        let mut statement: Statement = Statement {
            s_type: StatementType::Select,
            row: None,
        };

        match prepare_statement(&choice, &mut statement) {
            Ok(_) => execute_statement(&statement),
            Err(err) => {
                println!("{err}");
            }
        }
    }
}

fn parse_commmand(command: &str) -> Result<(), String> {
    match command {
        ".exit" => exit(0),
        _ => Err(format!("Error: unrecognized command '{command}'")),
    }
}

fn prepare_statement(choice: &str, statement: &mut Statement) -> Result<(), String> {
    match choice {
        cmd if cmd.starts_with("insert") => {
            statement.s_type = StatementType::Insert;
            let mut arguments = choice.split_whitespace();
            statement.row.id = arguments.next().unwrap().parse().unwrap();
            statement.row.username = arguments.next().unwrap().chars().collect();
            Ok(())
        }
        "select" => {
            statement.s_type = StatementType::Select;
            Ok(())
        }
        _ => Err(format!(
            "Error: unrecognized keyword at start of '{choice}'"
        )),
    }
}

fn execute_statement(statement: &Statement) {
    match statement.s_type {
        StatementType::Insert => println!("This is where insert occurs TODO"),
        StatementType::Select => println!("This is where select occurs TODO"),
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
