library;

use std::hash::{Hash, Hasher};

pub enum OrderType {
    Buy: (),
    Sell: (),
}

impl core::ops::Eq for OrderType {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Self::Buy, Self::Buy) => true,
            (Self::Sell, Self::Sell) => true,
            _ => false,
        }
    }
}

impl Hash for OrderType {
    fn hash(self, ref mut state: Hasher) {
        match self {
            Self::Buy => {
                0_u8.hash(state);
            }
            Self::Sell => {
                1_u8.hash(state);
            }
        }
    }
}
