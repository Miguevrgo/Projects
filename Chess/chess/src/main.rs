mod engine;
mod game;
mod uci;

use game::piece::Colour;

use crate::uci::game::Game;
use crate::uci::menu::*;

const TIME_FIVE_MIN: u64 = 300;
const TIME_THREE_MIN: u64 = 180;
const TIME_ONE_MIN: u64 = 60;

fn main() {
    loop {
        match show_menu() {
            MenuOption::StartLocal => {
                let mut game = Game::new(TIME_FIVE_MIN, 5, None, 0);
                game.play(None);
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
            MenuOption::StartComputer => {
                let mut game = Game::new(TIME_FIVE_MIN, 5, Some(Colour::Black), 6);
                game.play(Some(Colour::White));
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
            MenuOption::Quit => break,
        }
    }
}
