use crate::engine::evaluation::evaluate;
use crate::game::chess::{Game, GameState};
use crate::game::moves::Move;
use crate::game::piece::Piece;

/// Alpha - Beta Negamax
pub fn negamax(game: &mut Game, depth: u32, alpha: i32, beta: i32) -> (i32, Option<Move>) {
    if depth == 0 || matches!(game.state, GameState::Over(_)) {
        return (evaluate(game), None);
    }

    let mut alpha = alpha;
    let mut best_move = None;
    let mut best_score = -1000;

    for row in 0..8 {
        for col in 0..8 {
            let (colour, piece) = game.board.get_piece(row, col);
            if colour == game.current_colour() && piece != Piece::Empty {
                let moves = game.get_valid_moves(row, col, colour, piece);
                for &(new_row, new_col) in &moves {
                    let (orig_colour, orig_piece) = game.board.get_piece(row, col);
                    game.board.move_piece(row, col, new_row, new_col);
                    game.turn += 1;

                    let (score, _) = negamax(game, depth - 1, -beta, -alpha);
                    let score = -score;

                    game.board.move_piece(new_row, new_col, row, col);
                    game.board
                        .set_piece(new_row, new_col, orig_colour, orig_piece);
                    game.turn -= 1;

                    if score > best_score {
                        best_score = score;
                        best_move = Some(Move::from(piece, row, col, new_row, new_col));
                    }

                    alpha = alpha.max(score);
                    if alpha >= beta {
                        break;
                    }
                }
            }
        }
    }

    (alpha, best_move)
}
