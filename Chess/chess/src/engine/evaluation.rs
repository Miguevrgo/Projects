use crate::game::{
    bitboard::BitBoard,
    board::Board,
    piece::{Colour, Piece},
};

/// Material values for each piece
const PIECE_VALUES: [(i32, i32); 6] = [
    (82, 100),   // Pawn
    (330, 280),  // Knight
    (365, 300),  // Bishop
    (480, 510),  // Rook
    (1030, 930), // Queen
    (0, 0),      // King
];

#[rustfmt::skip]
const PST: [[i32; 64]; 6] = [
    // Pawn (midgame-focused, encouraging central control and advancement)
    [
         0,  0,  0,  0,  0,  0,  0,  0, // Rank 8
        50, 50, 50, 50, 50, 50, 50, 50, // Rank 7
        10, 10, 20, 30, 30, 20, 10, 10, // Rank 6
         5,  5, 10, 25, 25, 10,  5,  5, // Rank 5
         0,  0,  0, 20, 20,  0,  0,  0, // Rank 4
         5, -5,-10,  0,  0,-10, -5,  5, // Rank 3
         5, 10, 10,-20,-20, 10, 10,  5, // Rank 2
         0,  0,  0,  0,  0,  0,  0,  0, // Rank 1
    ],
    // Knight (favoring center)
    [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50,
    ],
    // Bishop (favoring center and diagonals)
    [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  5,  5, 15, 15,  5,  5,-10,
        -10,  0, 10, 15, 15, 10,  0,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20,
    ],
    // Rook (favoring 7th rank and central files)
    [
         0,  0,  0,  0,  0,  0,  0,  0,
         5, 10, 10, 10, 10, 10, 10,  5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
         0,  0,  0,  5,  5,  0,  0,  0,
    ],
    // Queen (favoring center)
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
         -5,  0,  5, 10, 10,  5,  0, -5,
          0,  0,  5, 10, 10,  5,  0, -5,
        -10,  5,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20,
    ],
    // King (corner in midgame, center in endgame)
    [
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -10,-20,-20,-20,-20,-20,-20,-10,
         10, 10,-10,-10,-10,-10, 10, 10,
         20, 30, 10,  0,  0, 10, 30, 20,
    ],
];
const MATE: i32 = 30000;

pub fn evaluate(board: &Board) -> i32 {
    let legal_moves = board.generate_legal_moves();
    if legal_moves.is_empty() {
        let king_square = board.king_square(board.side);
        if board.is_attacked_by(king_square, !board.side) {
            return if board.side == Colour::White {
                -MATE
            } else {
                MATE
            };
        } else {
            return 0; // Draw
        }
    }

    let mut score = 0;

    let occupied = board.sides[Colour::White as usize] | board.sides[Colour::Black as usize];
    let game_phase = if occupied.count_bits() < 16 { 1 } else { 0 };

    let mut occupied_bb = occupied;
    while occupied_bb != BitBoard::EMPTY {
        let square = occupied_bb.0.trailing_zeros() as usize;
        if let Some(piece) = board.piece_map[square] {
            let piece_idx = match piece {
                Piece::WP | Piece::BP => 0, // Pawn
                Piece::WN | Piece::BN => 1, // Knight
                Piece::WB | Piece::BB => 2, // Bishop
                Piece::WR | Piece::BR => 3, // Rook
                Piece::WQ | Piece::BQ => 4, // Queen
                Piece::WK | Piece::BK => 5, // King
            };
            let (midgame_value, endgame_value) = PIECE_VALUES[piece_idx];
            let material = if game_phase == 0 {
                midgame_value
            } else {
                endgame_value
            };

            let pst_value = if piece.colour() == Colour::White {
                PST[piece_idx][square]
            } else {
                PST[piece_idx][63 - square] // Mirror for Black
            };

            score += if piece.colour() == Colour::White {
                material + pst_value
            } else {
                -(material + pst_value)
            };
        }
        occupied_bb = occupied_bb.pop_bit(occupied_bb.lsb());
    }

    for m in legal_moves {
        let mut new_board = *board;
        new_board.make_move(m);
        let king_square = new_board.king_square(!board.side);
        if new_board.is_attacked_by(king_square, board.side) {
            score += if board.side == Colour::White { 50 } else { -50 };
        }
    }

    let white_king = board.king_square(Colour::White);
    let black_king = board.king_square(Colour::Black);
    if board.is_attacked_by(white_king, Colour::Black) {
        score -= 20;
    }
    if board.is_attacked_by(black_king, Colour::White) {
        score += 20;
    }

    if board.side == Colour::White {
        score
    } else {
        -score
    }
}
