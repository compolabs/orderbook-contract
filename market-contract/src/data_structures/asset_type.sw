library;

use std::hash::{Hash, Hasher};

pub enum AssetType {
    Base: (),
    Quote: (),
}

impl Hash for AssetType {
    fn hash(self, ref mut state: Hasher) {
        match self {
            Self::Base => {
                0_u8.hash(state);
            }
            Self::Quote => {
                1_u8.hash(state);
            }
        }
    }
}


impl core::ops::Eq for AssetType {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Self::Base, Self::Base) => true,
            (Self::Quote, Self::Quote) => true,
            _ => false,
        }
    }
}