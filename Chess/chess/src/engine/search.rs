use super::evaluation::evaluate;
use crate::engine::evaluation::PIECE_VALUES;
use crate::game::moves::MoveKind;
use crate::game::{board::Board, moves::Move, piece::Colour};
use std::sync::mpsc;
use std::thread;

const INF: i32 = 1000000;
const MATE: i32 = 100000;

#[derive(Debug)]
pub struct MinimaxEngine {
    depth: usize,
    colour: Colour,
}

impl MinimaxEngine {
    pub fn new(depth: usize, colour: Colour) -> Self {
        MinimaxEngine { depth, colour }
    }

    pub fn find_best_move(&self, board: &Board) -> (i32, Move) {
        let moves = board.generate_legal_moves();
        if moves.is_empty() {
            return (evaluate(board), Move::default());
        }

        let mut moves_with_scores: Vec<(Move, i32)> = moves
            .into_iter()
            .map(|m| (m, self.move_score(&m, board)))
            .collect();
        moves_with_scores.sort_by_key(|&(_, score)| std::cmp::Reverse(score));

        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        for (m, _) in moves_with_scores {
            let board_clone = *board;
            let tx_clone = tx.clone();
            let depth = self.depth;
            let colour = self.colour;

            let handle = thread::spawn(move || {
                let mut new_board = board_clone;
                new_board.make_move(m);
                let eval = -MinimaxEngine::new(depth - 1, colour).negamax(
                    &mut new_board,
                    depth - 1,
                    -INF,
                    INF,
                    !colour,
                );
                tx_clone.send((eval, m)).unwrap();
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for _ in 0..handles.len() {
            results.push(rx.recv().unwrap());
        }

        for handle in handles {
            handle.join().unwrap();
        }

        if self.colour == Colour::White {
            results.into_iter().max_by_key(|&(eval, _)| eval)
        } else {
            results.into_iter().min_by_key(|&(eval, _)| eval)
        }
        .unwrap_or((0, Move::default()))
    }

    fn negamax(
        &self,
        board: &mut Board,
        depth: usize,
        mut alpha: i32,
        beta: i32,
        turn: Colour,
    ) -> i32 {
        if depth == 0 {
            return evaluate(board);
        }

        let moves = board.generate_legal_moves();
        if moves.is_empty() {
            let king_square = board.king_square(turn);
            return if board.is_attacked_by(king_square, !turn) {
                -MATE + (self.depth - depth) as i32
            } else {
                0 // Draw
            };
        }

        let mut moves_with_scores: Vec<(Move, i32)> = moves
            .into_iter()
            .map(|m| (m, self.move_score(&m, board)))
            .collect();
        moves_with_scores.sort_by_key(|&(_, score)| std::cmp::Reverse(score));

        let mut max_score = -INF;
        for (m, _) in moves_with_scores {
            let mut new_board = *board;
            new_board.make_move(m);
            let score = -self.negamax(&mut new_board, depth - 1, -beta, -alpha, !turn);

            if score > max_score {
                max_score = score;
            }

            if score > alpha {
                alpha = score;
            }

            if alpha >= beta {
                break; // Beta cutoff
            }
        }

        max_score
    }

    fn move_score(&self, m: &Move, board: &Board) -> i32 {
        let mut score = 0;

        if m.get_type().is_capture() {
            let src_piece = board.piece_at(m.get_source()).unwrap();
            let dest_piece = board.piece_at(m.get_dest());

            if let Some(dest_piece) = dest_piece {
                score += 10 * PIECE_VALUES[dest_piece.index()] - PIECE_VALUES[src_piece.index()];
            } else if m.get_type() == MoveKind::EnPassant {
                score += PIECE_VALUES[0];
            }
        }

        if m.get_type().is_promotion() {
            let promo_piece = m.get_type().get_promotion(board.side);
            score += PIECE_VALUES[promo_piece.index()];
        }

        let mut new_board = *board;
        new_board.make_move(*m);
        if new_board.is_attacked_by(new_board.king_square(!board.side), board.side) {
            score += 50;
        }

        score
    }
}
