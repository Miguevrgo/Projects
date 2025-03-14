mod game;

use crate::game::board::Board;

fn main() {
    let board = Board::default();
    board.draw()
}
