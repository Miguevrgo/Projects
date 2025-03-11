use crate::game::piece::Colour;
use crate::game::square::Square;

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

impl std::ops::BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        BitBoard(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl std::ops::Shl<u8> for BitBoard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self {
        BitBoard(self.0 << rhs)
    }
}

impl std::ops::Shr<u8> for BitBoard {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self {
        BitBoard(self.0 >> rhs)
    }
}

impl std::ops::Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl std::ops::BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl BitBoard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(0xFFFFFFFFFFFFFFFF);

    pub const START_RANKS: [Self; 2] = [Self(0x00FF000000000000), Self(0x000000000000FF00)];
    pub const EP_RANKS: [Self; 2] = [Self(0x00000000FF000000), Self(0x000000FF00000000)];
    pub const PROMO_RANKS: [Self; 2] = [Self(0x000000000000FF00), Self(0x00FF000000000000)];

    pub fn get_bit(self, square: Square) -> bool {
        self.0 & (1u64 << square.index()) != 0
    }

    pub fn set_bit(self, square: Square) -> Self {
        Self(self.0 | (1u64 << square.index()))
    }

    pub fn pop_bit(self, square: Square) -> Self {
        Self(self.0 & !(1u64 << square.index()))
    }

    pub fn forward(self, side: Colour) -> Self {
        match side {
            Colour::White => Self(self.0 >> 8),
            Colour::Black => Self(self.0 << 8),
        }
    }

    pub fn count_bits(self) -> u32 {
        self.0.count_ones()
    }

    pub fn lsb(self) -> Square {
        Square::new(self.0.trailing_zeros() as usize)
    }
}
