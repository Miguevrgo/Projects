use super::evaluation::evaluate;
use crate::game::{board::Board, moves::Move, piece::Colour};

#[derive(Debug)]
pub struct MinimaxEngine {
    depth: usize,
    colour: Colour,
}

impl MinimaxEngine {
    pub fn new(depth: usize, colour: Colour) -> Self {
        MinimaxEngine { depth, colour }
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
            let eval = evaluate(board);
            if eval.abs() >= 30_000 - 100 {
                return if eval > 0 {
                    eval - (self.depth - depth) as i32
                } else {
                    eval + (self.depth - depth) as i32
                };
            }
            return eval;
        }

        let mut moves = board.generate_legal_moves();
        moves.sort_by_key(|b| std::cmp::Reverse(move_score(b)));

        if turn == Colour::White {
            let mut max_eval = i32::MIN;
            for m in moves {
                let mut new_board = *board;
                new_board.make_move(m);
                let eval = self.alpha_beta(&new_board, depth - 1, alpha, beta, Colour::Black);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break;
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for m in moves {
                let mut new_board = *board;
                new_board.make_move(m);
                let eval = self.alpha_beta(&new_board, depth - 1, alpha, beta, Colour::White);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break;
                }
            }
            min_eval
        }
    }

    pub fn find_best_move(&mut self, board: &mut Board) -> (i32, Move) {
        let mut moves = board.generate_legal_moves();
        if moves.is_empty() {
            return (evaluate(board), Move::default());
        }
        let occupied = board.sides[Colour::White as usize] | board.sides[Colour::Black as usize];
        if occupied.count_bits() < 6 {
            self.depth = 8;
        }

        let mut best_score = if self.colour == board.side {
            i32::MIN
        } else {
            i32::MAX
        };
        let mut best_move = moves[0];
        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        moves.sort_by_key(|b| std::cmp::Reverse(move_score(b)));

        for m in moves {
            let mut new_board = *board;
            new_board.make_move(m);
            let score = self.alpha_beta(&new_board, self.depth - 1, alpha, beta, !board.side);
            if self.colour == board.side {
                if score > best_score {
                    best_score = score;
                    best_move = m;
                    alpha = alpha.max(score);
                }
            } else if score < best_score {
                best_score = score;
                best_move = m;
                beta = beta.min(score);
            }
        }

        (best_score, best_move)
    }
}

fn move_score(m: &Move) -> i32 {
    let move_type = m.get_type();
    if move_type.is_capture() {
        if move_type.is_promotion() {
            return 1000;
        }
        return 500;
    } else if move_type.is_promotion() {
        return 700;
    }
    0
}
