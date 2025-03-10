use crate::game::bitboard::BitBoard;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
pub struct Square(u8);

impl Square {
    pub const COUNT: usize = 64;

    pub fn new(index: usize) -> Self {
        Self(index as u8)
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn from_row_col(row: usize, col: usize) -> Self {
        Self((row * 8 + col) as u8)
    }

    pub fn row(&self) -> usize {
        self.0 as usize / 8
    }

    pub fn col(&self) -> usize {
        self.0 as usize % 8
    }

    pub fn to_board(self) -> BitBoard {
        BitBoard(1 << self.0)
    }

    pub fn jump(self, file_delta: i8, rank_delta: i8) -> Option<Self> {
        let file = (self.0 % 8) as i8 + file_delta;
        let rank = (self.0 / 8) as i8 - rank_delta;
        if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
            Some(Self((rank * 8 + file) as u8))
        } else {
            None
        }
    }
}
