library;

use std::hash::{Hash, Hasher};

pub enum AssetType {
    Base: (),
    Quote: (),
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

impl core::ops::Not for AssetType {
    fn not(self) -> Self {
        match self {
            Self::Base => Self::Quote,
            Self::Quote => Self::Base,
        }
    }
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
