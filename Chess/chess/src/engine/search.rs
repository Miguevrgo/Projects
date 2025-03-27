use super::evaluation::evaluate;
use crate::game::{board::Board, moves::Move, piece::Colour};
use std::thread;

#[derive(Debug)]
pub struct MinimaxEngine {
    depth: usize,
    colour: Colour,
}

impl MinimaxEngine {
    pub fn new(depth: usize, colour: Colour) -> Self {
        MinimaxEngine { depth, colour }
    }

    pub fn find_best_move(&self, board: &mut Board) -> (i32, Move) {
        let moves = board.generate_legal_moves();
        if moves.is_empty() {
            return (evaluate(board), Move::default());
        }

        let mut moves_with_scores: Vec<(Move, i32)> = moves
            .into_iter()
            .map(|m| (m, move_score(&m, board)))
            .collect();
        moves_with_scores.sort_by_key(|&(_, score)| std::cmp::Reverse(score));

        let mut handles = Vec::new();
        let mut results = Vec::new();

        for (m, _) in moves_with_scores {
            let board_clone = *board;
            let depth = self.depth;
            let colour = self.colour;

            let handle = thread::spawn(move || {
                let mut new_board = board_clone;
                new_board.make_move(m);
                let eval = -MinimaxEngine::new(depth - 1, colour).alpha_beta(
                    &new_board,
                    depth - 1,
                    i32::MIN,
                    i32::MAX,
                    !colour,
                );

                (eval, m)
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.join().unwrap();
            results.push(result);
        }

        if self.colour == Colour::White {
            results
                .into_iter()
                .max_by_key(|&(_, eval)| eval)
                .unwrap_or((0, Move::default()))
        } else {
            results
                .into_iter()
                .min_by_key(|&(_, eval)| eval)
                .unwrap_or((0, Move::default()))
        }
    }

    fn alpha_beta(
        &self,
        board: &Board,
        depth: usize,
        mut alpha: i32,
        mut beta: i32,
        turn: Colour,
    ) -> i32 {
        if depth == 0 || board.generate_legal_moves().is_empty() {
            return evaluate(board);
        }

        let mut moves = board.generate_legal_moves();
        moves.sort_by_key(|b| std::cmp::Reverse(move_score(b, board)));

        if turn == Colour::White {
            let mut max_eval = i32::MIN;
            for m in &moves {
                let mut new_board = *board;
                new_board.make_move(*m);
                let eval = self.alpha_beta(&mut new_board, depth - 1, alpha, beta, Colour::Black);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for m in &moves {
                let mut new_board = *board;
                new_board.make_move(*m);
                let eval = self.alpha_beta(&mut new_board, depth - 1, alpha, beta, Colour::White);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }
}

fn move_score(m: &Move, board: &Board) -> i32 {
    let mut score = match m.get_type() {
        crate::game::moves::MoveKind::Castle => 300,
        crate::game::moves::MoveKind::Capture => 500,
        crate::game::moves::MoveKind::QueenCapPromo => 1200,
        crate::game::moves::MoveKind::RookCapPromo => 900,
        crate::game::moves::MoveKind::BishopCapPromo => 600,
        crate::game::moves::MoveKind::KnightCapPromo => 600,
        crate::game::moves::MoveKind::QueenPromotion => 1000,
        crate::game::moves::MoveKind::RookPromotion => 800,
        crate::game::moves::MoveKind::KnightPromotion => 500,
        crate::game::moves::MoveKind::BishopPromotion => 500,
        _ => 0,
    };
    let mut new_board = *board;
    new_board.make_move(*m);
    if new_board.is_attacked_by(new_board.king_square(!new_board.side), new_board.side) {
        score += 300;
    }

    score
}
