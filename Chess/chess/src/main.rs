mod game;
mod ui;

use crate::game::chess::Game;
use ui::menu::{show_menu, MenuOption};

fn main() {
    loop {
        match show_menu() {
            MenuOption::StartLocal => {
                let mut game = Game::new();
                game.play();
            }
            MenuOption::StartNetwork => {
                unimplemented!();
            }
            MenuOption::StartComputer => {
                unimplemented!();
            }
            MenuOption::Quit => {
                break;
            }
        }
    }
}
