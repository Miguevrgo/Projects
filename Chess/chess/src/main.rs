mod game;
use crate::game::chess::Game;

fn main() {
    let mut board = Game::new();
    board.draw();
    loop {
        board.next_move();
        if board.turn == 3 {
            break;
        }
    }
}
