use crate::game::bitboard::BitBoard;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
pub struct Square(u8);

impl Square {
    pub const COUNT: usize = 64;
    #[rustfmt::skip]
    const STR: [&str; Self::COUNT] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    ];

    pub fn from(pos: &str) -> Self {
        Self::new(
            Self::STR
                .iter()
                .position(|&coord| coord == pos)
                .expect("Invalid pos"),
        )
    }

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
        if (0..8).contains(&file) && (0..8).contains(&rank) {
            Some(Self((rank * 8 + file) as u8))
        } else {
            None
        }
    }
}
