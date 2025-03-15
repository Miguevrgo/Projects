mod game;
mod uci;

use crate::uci::game::Game;

fn main() {
    let mut game = Game::new(300, 3); // 5 minutes + 3 seconds/move
    game.play();
}
