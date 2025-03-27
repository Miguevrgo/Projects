use crate::game::{bitboard::BitBoard, board::Board, piece::Colour};

// Improved material values (centipawns)
pub const PIECE_VALUES: [i32; 6] = [
    100,   // Pawn
    320,   // Knight
    330,   // Bishop
    500,   // Rook
    900,   // Queen
    20000, // King (not actually used in evaluation)
];

#[rustfmt::skip]
const PST: [[i32; 64]; 6] = [
    // Pawn
    [
         0,   0,   0,   0,   0,   0,   0,   0,
        50,  50,  50,  50,  50,  50,  50,  50,
        10,  10,  20,  30,  30,  20,  10,  10,
         5,   5,  10,  25,  25,  10,   5,   5,
         0,   0,   0,  20,  20,   0,   0,   0,
         5,  -5, -10,   0,   0, -10,  -5,   5,
         5,  10,  10, -20, -20,  10,  10,   5,
         0,   0,   0,   0,   0,   0,   0,   0,
    ],
    // Knight
    [
        -50, -40, -30, -30, -30, -30, -40, -50,
        -40, -20,   0,   0,   0,   0, -20, -40,
        -30,   0,  10,  15,  15,  10,   0, -30,
        -30,   5,  15,  20,  20,  15,   5, -30,
        -30,   0,  15,  20,  20,  15,   0, -30,
        -30,   5,  10,  15,  15,  10,   5, -30,
        -40, -20,   0,   5,   5,   0, -20, -40,
        -50, -40, -30, -30, -30, -30, -40, -50,
    ],
    // Bishop
    [
        -20, -10, -10, -10, -10, -10, -10, -20,
        -10,   0,   0,   0,   0,   0,   0, -10,
        -10,   0,   5,  10,  10,   5,   0, -10,
        -10,   5,   5,  15,  15,   5,   5, -10,
        -10,   0,  10,  15,  15,  10,   0, -10,
        -10,  10,  10,  10,  10,  10,  10, -10,
        -10,   5,   0,   0,   0,   0,   5, -10,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ],
    // Rook
    [
         0,   0,   0,   0,   0,   0,   0,   0,
         5,  10,  10,  10,  10,  10,  10,   5,
        -5,   0,   0,   0,   0,   0,   0,  -5,
        -5,   0,   0,   0,   0,   0,   0,  -5,
        -5,   0,   0,   0,   0,   0,   0,  -5,
        -5,   0,   0,   0,   0,   0,   0,  -5,
        -5,   0,   0,   0,   0,   0,   0,  -5,
         0,   0,   0,   5,   5,   0,   0,   0,
    ],
    // Queen
    [
        -20, -10, -10,  -5,  -5, -10, -10, -20,
        -10,   0,   0,   0,   0,   0,   0, -10,
        -10,   0,   5,   5,   5,   5,   0, -10,
         -5,   0,   5,  10,  10,   5,   0,  -5,
          0,   0,   5,  10,  10,   5,   0,  -5,
        -10,   5,   5,   5,   5,   5,   0, -10,
        -10,   0,   5,   0,   0,   0,   0, -10,
        -20, -10, -10,  -5,  -5, -10, -10, -20,
    ],
    // King (midgame)
    [
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -30, -40, -40, -50, -50, -40, -40, -30,
        -20, -30, -30, -40, -40, -30, -30, -20,
        -10, -20, -20, -20, -20, -20, -20, -10,
         10,  10, -10, -10, -10, -10,  10,  10,
         20,  30,  10,   0,   0,  10,  30,  20,
    ],
];

const MATE_SCORE: i32 = 100000;
const DRAW_SCORE: i32 = 0;

pub fn evaluate(board: &Board) -> i32 {
    // Check for mate or stalemate
    let legal_moves = board.generate_legal_moves();
    if legal_moves.is_empty() {
        let king_square = board.king_square(board.side);
        return if board.is_attacked_by(king_square, !board.side) {
            if board.side == Colour::White {
                -MATE_SCORE
            } else {
                MATE_SCORE
            }
        } else {
            DRAW_SCORE
        };
    }

    let mut score = 0;
    let occupied = board.sides[Colour::White as usize] | board.sides[Colour::Black as usize];
    let mut occupied_bb = occupied;

    while occupied_bb != BitBoard::EMPTY {
        let square = occupied_bb.lsb();
        if let Some(piece) = board.piece_at(square) {
            let piece_idx = piece.index();
            let material = PIECE_VALUES[piece_idx];
            let pst_value = if piece.colour() == Colour::White {
                PST[piece_idx][square.index()]
            } else {
                -PST[piece_idx][63 - square.index()]
            };

            if piece.colour() == Colour::White {
                score += material + pst_value;
            } else {
                score -= material + pst_value;
            }
        }
        occupied_bb = occupied_bb.pop_bit(square);
    }

    let mobility = legal_moves.len() as i32;
    score += if board.side == Colour::White {
        mobility
    } else {
        -mobility
    };

    let white_king = board.king_square(Colour::White);
    let black_king = board.king_square(Colour::Black);

    if board.is_attacked_by(white_king, Colour::Black) {
        score -= 50;
    }
    if board.is_attacked_by(black_king, Colour::White) {
        score += 50;
    }

    if board.side == Colour::Black {
        -score
    } else {
        score
    }
}
