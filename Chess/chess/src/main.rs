mod game;
mod ui;
use crate::game::chess::Game;
use crate::ui::menu;

fn main() {
    let mut game = Game::new();
    game.play();
}
