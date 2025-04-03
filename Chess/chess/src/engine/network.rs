use crate::game::{
    piece::{Colour, Piece},
    square::Square,
};
use std::arch::x86_64::*;

// Square: 0-63
// Piece: Pawn = 0, Knight = 1, Bishop = 2, Rook = 3, Queen = 4, King = 5
// Side: White = 0, Black = 1
const INPUT_SIZE: usize = 768; // USIZE? TODO:
const HL_SIZE: usize = 1024;

const SCALE: i32 = 400;
const QA: i32 = 255;
const QB: i32 = 64;
const QAB: i32 = QA * QB;
const NUM_BUCKETS: usize = 4;

#[rustfmt::skip]
static BUCKETS: [usize; 64] = [
    0, 0, 1, 1, 5, 5, 4, 4,
    2, 2, 2, 2, 6, 6, 6, 6,
    3, 3, 3, 3, 7, 7, 7, 7,
    3, 3, 3, 3, 7, 7, 7, 7,
    3, 3, 3, 3, 7, 7, 7, 7,
    3, 3, 3, 3, 7, 7, 7, 7,
    3, 3, 3, 3, 7, 7, 7, 7,
    3, 3, 3, 3, 7, 7, 7, 7,
];

static NNUE: Network =
    unsafe { std::mem::transmute(*include_bytes!("../../resources/nnue-net.bin")) };

#[repr(C)]
pub struct Network {
    accumulator_weights: [Accumulator; INPUT_SIZE * NUM_BUCKETS],
    accumulator_biases: Accumulator,
    output_weights: [Accumulator; 2],
    output_bias: i16,
}

impl Network {
    /// Calculates the feature index for a piece on a square from a given perspective.
    /// - `perspective`: The side evaluating the position (White or Black).
    /// - `square`: The square where the piece resides.
    /// - `piece_type`: The type of piece (Pawn, Knight, etc.).
    /// - `side`: The color of the piece (White or Black).
    /// - `king_sq`: The square of the king (for bucketing).
    pub fn calculate_index(
        perspective: Colour,
        square: Square,
        piece_type: Piece,
        side: Colour,
        king_sq: Square,
    ) -> usize {
        let sq_idx = if perspective == Colour::Black {
            square.index() ^ 56 // Mirror square for Black’s perspective
        } else {
            square.index()
        };

        let piece_idx = piece_type.index(); // 0-5 (Pawn to King)
        let color_idx = side as usize; // 0 (White), 1 (Black)
        let bucket = Self::get_bucket(king_sq, perspective);

        bucket * INPUT_SIZE + color_idx * 384 + piece_idx * 64 + sq_idx
    }

    fn get_bucket(ksq: Square, perspective: Colour) -> usize {
        let ksq_idx = if perspective == Colour::Black {
            ksq.index() ^ 56 // Mirror king square for Black
        } else {
            ksq.index()
        };
        BUCKETS[ksq_idx]
    }

    pub fn update_accumulator(
        &self,
        acc: &mut Accumulator,
        perspective: Colour,
        square: Square,
        piece_type: Piece,
        side: Colour,
        king_sq: Square,
        add: bool,
    ) {
        let idx = Self::calculate_index(perspective, square, piece_type, side, king_sq);
        if add {
            acc.add(&self.accumulator_weights[idx]);
        } else {
            acc.sub(&self.accumulator_weights[idx]);
        }
    }

    pub fn evaluate(&self, white_acc: &Accumulator, black_acc: &Accumulator) -> i32 {
        unsafe {
            let white_sum = flatten(white_acc, &self.output_weights[0]);
            let black_sum = flatten(black_acc, &self.output_weights[1]);
            let sum = white_sum + black_sum;
            (sum + i32::from(self.output_bias)) * SCALE / QAB
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(64))]
pub struct Accumulator {
    vals: [i16; HL_SIZE],
}

impl Accumulator {
    /// Initializes the accumulator with biases.
    pub fn init(&mut self, biases: &Accumulator) {
        self.vals.copy_from_slice(&biases.vals);
    }

    /// Adds a feature’s weights from another accumulator to this one.
    pub fn add(&mut self, weights: &Accumulator) {
        for i in 0..HL_SIZE {
            self.vals[i] = self.vals[i].saturating_add(weights.vals[i]);
        }
    }

    /// Subtracts a feature’s weights from another accumulator from this one.
    pub fn sub(&mut self, weights: &Accumulator) {
        for i in 0..HL_SIZE {
            self.vals[i] = self.vals[i].saturating_sub(weights.vals[i]);
        }
    }

    pub fn default() -> Self {
        let mut acc = Self { vals: [0; HL_SIZE] };
        acc.init(&NNUE.accumulator_biases);
        acc
    }
}

pub fn screlu(x: i16) -> i32 {
    i32::from(x.clamp(0, QA as i16)).pow(2)
}

pub unsafe fn flatten(acc: &Accumulator, weights: &Accumulator) -> i32 {
    const CHUNK: usize = 16;

    let mut sum = _mm256_setzero_si256();
    let min = _mm256_setzero_si256();
    let max = _mm256_set1_epi16(QA as i16);

    for i in 0..HL_SIZE / CHUNK {
        let mut v = load_i16s(acc, i * CHUNK);
        v = _mm256_min_epi16(_mm256_max_epi16(v, min), max);
        let w = load_i16s(weights, i * CHUNK);
        let product = _mm256_madd_epi16(v, _mm256_mullo_epi16(v, w));
        sum = _mm256_add_epi32(sum, product);
    }

    horizontal_sum_i32(sum)
}

#[inline]
unsafe fn load_i16s(acc: &Accumulator, start_idx: usize) -> __m256i {
    _mm256_load_si256(acc.vals.as_ptr().add(start_idx).cast())
}

#[inline]
unsafe fn horizontal_sum_i32(sum: __m256i) -> i32 {
    let upper_128 = _mm256_extracti128_si256::<1>(sum);
    let lower_128 = _mm256_castsi256_si128(sum);
    let sum_128 = _mm_add_epi32(upper_128, lower_128);
    let upper_64 = _mm_unpackhi_epi64(sum_128, sum_128);
    let sum_64 = _mm_add_epi32(upper_64, sum_128);
    let upper_32 = _mm_shuffle_epi32::<0b00_00_00_01>(sum_64);
    let sum_32 = _mm_add_epi32(upper_32, sum_64);

    _mm_cvtsi128_si32(sum_32)
}
