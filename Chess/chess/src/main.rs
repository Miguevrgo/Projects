mod game;

fn main() {
    let board = game::board::Board::new();
    board.draw();
}
