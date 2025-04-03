use crate::game::{
    piece::{Colour, Piece},
    square::Square,
};

// Square: 0-63
// Piece: Pawn = 0, Knight = 1, Bishop = 2, Rook = 3, Queen = 4, King = 5
// Side: White = 0, Black = 1
const INPUT_SIZE: usize = 768; // USIZE? TODO:
const HL_SIZE: usize = 1024;

const SCALE: i32 = 400;
const QA: i32 = 255;
const QB: i32 = 64;

#[repr(C)]
pub struct Network {
    accumulator_weights: [Accumulator; INPUT_SIZE], //TODO: Buckets
    accumulator_biases: Accumulator,
    output_weights: [Accumulator; 2],
    output_bias: i16,
}

impl Network {
    pub fn calculate_index(
        perspective: Colour,
        square: Square,
        piece_type: Piece,
        side: Colour,
    ) -> i16 {
        let mut side_val: i16 = side as i16;
        let mut sq_val: i16 = square.index() as i16;

        if perspective == Colour::Black {
            side_val = 1 - side_val;
            sq_val ^= 0b111000;
        }
        side_val * 64 * 6 + piece_type.index() as i16 * 64 + sq_val
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(64))]
pub struct Accumulator {
    vals: [i16; HL_SIZE],
}
