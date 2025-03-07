use crate::game::chess::Game;
use crate::game::piece::*;

pub fn evaluate(game: &Game) -> i32 {
    let mut score = 0;
    let board = &game.board;

    let piece_values = [
        0,     // Empty
        100,   // Pawn
        300,   // Knight
        300,   // Bishop
        500,   // Rook
        900,   // Queen
        10000, // King
    ];

    let pawn_table_white: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],         // Rank 1
        [0, 0, 0, 0, 0, 0, 0, 0],         // Rank 2
        [5, 5, 10, 20, 20, 10, 5, 5],     // Rank 3
        [10, 10, 20, 30, 30, 20, 10, 10], // Rank 4
        [20, 20, 30, 40, 40, 30, 20, 20], // Rank 5
        [30, 30, 40, 50, 50, 40, 30, 30], // Rank 6
        [50, 50, 50, 50, 50, 50, 50, 50], // Rank 7 (promotion)
        [0, 0, 0, 0, 0, 0, 0, 0],         // Rank 8
    ];

    let mut pawn_table_black = pawn_table_white;
    pawn_table_black.reverse();

    for row in 0..8 {
        for col in 0..8 {
            let (colour, piece) = board.get_piece(row, col);
            if piece == Piece::Empty {
                continue;
            }

            let base_value = piece_values[piece as usize];
            let mut positional_bonus = 0;

            if piece == Piece::Pawn {
                positional_bonus = if colour == Colour::White {
                    pawn_table_white[row][col]
                } else {
                    pawn_table_black[row][col]
                };
            }

            score += if colour == Colour::White {
                base_value
                    + positional_bonus
                    + game.get_valid_moves(row, col, colour, piece).len() as i32
            } else {
                -(base_value
                    + positional_bonus
                    + game.get_valid_moves(row, col, colour, piece).len() as i32)
            };
        }
    }

    score
}
