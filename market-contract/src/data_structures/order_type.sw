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


// impl u64 {
//     pub fn flip(self) -> Self {
//         self * Self::neg_from(1)
//     }

//     pub fn is_same_sign(self, value: u64) -> bool {
//         self.negative && value.negative || !self.negative && !value.negative
//     }  

//     pub fn mul_div(self, mul_to: u64, div_to: u64) -> Self {
//         u64 {
//             value: (self.value.overflowing_mul(mul_to) / U128::from((0, div_to))).as_u64().unwrap(),
//             negative: self.negative
//         }
//     }
// }