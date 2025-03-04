mod game;
mod ui;

use crate::game::chess::Game;
use ui::menu::{show_menu, MenuOption};

fn main() {
    let mut game = Game::new();
    loop {
        match show_menu() {
            MenuOption::StartLocal => {
                game = Game::new();
                game.play();
            }
            MenuOption::StartNetwork => {
                unimplemented!();
            }
            MenuOption::StartComputer => {
                unimplemented!();
            }
            MenuOption::Resume => {
                game.play();
            }
            MenuOption::Quit => break,
        }
    }
}
