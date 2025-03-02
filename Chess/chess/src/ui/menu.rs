use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub enum MenuOption {
    StartLocal,    // PvP
    StartNetwork,  // Using LAN
    StartComputer, // Against an AI
    //TODO: Resume
    Quit,
}

pub fn show_menu() -> MenuOption {
    println!("\x1B[2J\x1B[1;1H");
    println!(
        r"
  ______             __        __                   ______   __                                    
 /      \           /  |      /  |                 /      \ /  |                                   
/$$$$$$  | __    __ $$/   ____$$ |  ______        /$$$$$$  |$$ |____    ______    _______  _______ 
$$ |  $$ |/  \  /  |/  | /    $$ | /      \       $$ |  $$/ $$      \  /      \  /       |/       |
$$ |  $$ |$$  \/$$/ $$ |/$$$$$$$ |/$$$$$$  |      $$ |      $$$$$$$  |/$$$$$$  |/$$$$$$$//$$$$$$$/ 
$$ |  $$ | $$  $$<  $$ |$$ |  $$ |$$    $$ |      $$ |   __ $$ |  $$ |$$    $$ |$$      \$$      \ 
$$ \__$$ | /$$$$  \ $$ |$$ \__$$ |$$$$$$$$/       $$ \__/  |$$ |  $$ |$$$$$$$$/  $$$$$$  |$$$$$$  |
$$    $$/ /$$/ $$  |$$ |$$    $$ |$$       |      $$    $$/ $$ |  $$ |$$       |/     $$//     $$/ 
 $$$$$$/  $$/   $$/ $$/  $$$$$$$/  $$$$$$$/        $$$$$$/  $$/   $$/  $$$$$$$/ $$$$$$$/ $$$$$$$/  
    
    By Miguevrgo
"
    );
    println!("[1] Local Game");
    println!("[2] Network Game");
    println!("[3] Play vs Computer");
    println!("[4] Quit");
    println!("Use 1-4 to select");

    enable_raw_mode().unwrap();
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            disable_raw_mode().unwrap();
            match event.code {
                KeyCode::Char('1') => return MenuOption::StartLocal,
                KeyCode::Char('2') => return MenuOption::StartNetwork,
                KeyCode::Char('3') => return MenuOption::StartComputer,
                KeyCode::Char('4') => return MenuOption::Quit,
                _ => continue,
            }
        }
    }
}
