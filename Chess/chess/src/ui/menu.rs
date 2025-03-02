use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub enum MenuOption {
    StartLocal,    // PvP
    StartNetwork,  // Using LAN
    StartComputer, // Against an AI
    Resume,        // Resume current game
    Quit,
}

pub fn show_menu() -> MenuOption {
    println!("\x1B[2J\x1B[1;1H");
    println!(
        r"
  ___       _     _         ____ _                   
 / _ \__  _(_) __| | ___   / ___| |__   ___  ___ ___ 
| | | \ \/ / |/ _` |/ _ \ | |   | '_ \ / _ \/ __/ __|
| |_| |>  <| | (_| |  __/ | |___| | | |  __/\__ \__ \
 \___//_/\_\_|\__,_|\___|  \____|_| |_|\___||___/___/


    By Miguevrgo
"
    );
    println!("[1] Local Game");
    println!("[2] Network Game");
    println!("[3] Play vs Computer");
    println!("[4] Resume");
    println!("[5] Quit");
    println!("Use 1-5 to select");

    enable_raw_mode().unwrap();
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            disable_raw_mode().unwrap();
            match event.code {
                KeyCode::Char('1') => return MenuOption::StartLocal,
                KeyCode::Char('2') => return MenuOption::StartNetwork,
                KeyCode::Char('3') => return MenuOption::StartComputer,
                KeyCode::Char('4') => return MenuOption::Resume,
                KeyCode::Char('5') => return MenuOption::Quit,
                _ => continue,
            }
        }
    }
}
