mod game;

use crate::game::board::Board;

fn main() {
    let board = Board::default();
    loop {
        board.draw();
        std::thread::sleep(std::time::Duration::from_secs(500));
    }
}
