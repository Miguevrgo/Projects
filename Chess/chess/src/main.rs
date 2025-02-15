mod game;
use crate::game::chess::Game;

fn main() {
    let mut board = Game::new();
    loop {
        board.draw();
        board.next_move();
        if board.turn == 3 {
            break;
        }
    }
}
