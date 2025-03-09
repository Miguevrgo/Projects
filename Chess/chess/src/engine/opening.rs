use crate::game::moves::Move;
use crate::game::piece::Piece;
use crate::Game;
use std::collections::HashMap;

#[derive(Debug)]
pub struct OpeningBook {
    openings: HashMap<String, Vec<String>>,
}

impl OpeningBook {
    pub fn new() -> Self {
        let mut openings = HashMap::new();

        openings.insert(
            "Italian Opening".to_string(),
            vec![
                "e2e4".to_string(),
                "e7e5".to_string(),
                "g1f3".to_string(),
                "b8c6".to_string(),
                "f1c4".to_string(),
            ],
        );

        openings.insert(
            "Sicilian Defense".to_string(),
            vec![
                "e2e4".to_string(),
                "c7c5".to_string(),
                "g1f3".to_string(),
                "d7d6".to_string(),
                "d2d4".to_string(),
            ],
        );

        openings.insert(
            "Queen's Gambit".to_string(),
            vec!["d2d4".to_string(), "d7d5".to_string(), "c2c4".to_string()],
        );

        OpeningBook { openings }
    }

    pub fn get_next_move(&self, opening_name: &str, game: &Game) -> Option<Move> {
        if let Some(moves) = self.openings.get(opening_name) {
            let turn_index = game.turn as usize - 1;
            if turn_index < moves.len() {
                return Self::parse_move(&moves[turn_index], game);
            }
        }
        None
    }

    fn parse_move(move_str: &str, game: &Game) -> Option<Move> {
        if move_str.len() < 4 {
            return None;
        }

        let prev_col = (move_str.chars().nth(0)? as u8 - b'a') as usize;
        let prev_row = (move_str.chars().nth(1)? as u8 - b'1') as usize;
        let new_col = (move_str.chars().nth(2)? as u8 - b'a') as usize;
        let new_row = (move_str.chars().nth(3)? as u8 - b'1') as usize;

        let (colour, piece) = game.board.get_piece(prev_row, prev_col);
        if piece == Piece::Empty || colour != game.current_colour() {
            return None;
        }

        let is_capture = game.board.get_piece(new_row, new_col).1 != Piece::Empty;
        let promotion = if move_str.len() > 4 && move_str.chars().nth(4)? == '=' {
            match move_str.chars().nth(5)? {
                'Q' => Some(Piece::Queen),
                'R' => Some(Piece::Rook),
                'B' => Some(Piece::Bishop),
                'N' => Some(Piece::Knight),
                _ => None,
            }
        } else {
            None
        };

        Some(Move::from(
            piece, prev_row, prev_col, new_row, new_col, is_capture, promotion, false,
        ))
    }
}
