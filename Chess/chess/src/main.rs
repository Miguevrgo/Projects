mod engine;
mod game;
mod uci;

use crate::uci::game::Game;

const TIME_THREE_MIN: u64 = 180;
const TIME_ONE_MIN: u64 = 60;

fn main() {
    let mut game = Game::new(TIME_THREE_MIN, 3); // 3 minutes + 3 seconds/move
    game.play();
}
