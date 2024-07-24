library;

use std::hash::{Hash, Hasher};

pub enum LimitType {
    IOC: (),
    FOK: (),
}

impl core::ops::Eq for LimitType {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Self::IOC, Self::IOC) => true,
            (Self::FOK, Self::FOK) => true,
            _ => false,
        }
    }
}

impl Hash for LimitType {
    fn hash(self, ref mut state: Hasher) {
        match self {
            Self::IOC => {
                0_u8.hash(state);
            }
            Self::FOK => {
                1_u8.hash(state);
            }
        }
    }
}
