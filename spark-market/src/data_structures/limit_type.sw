library;

use std::hash::{Hash, Hasher};

pub enum LimitType {
    GTC: (),
    IOC: (),
    FOK: (),
}

impl core::ops::Eq for LimitType {
    fn eq(self, other: Self) -> bool {
        match (self, other) {
            (Self::GTC, Self::GTC) => true,
            (Self::IOC, Self::IOC) => true,
            (Self::FOK, Self::FOK) => true,
            _ => false,
        }
    }
}

impl Hash for LimitType {
    fn hash(self, ref mut state: Hasher) {
        match self {
            Self::GTC => {
                0_u8.hash(state);
            }
            Self::IOC => {
                1_u8.hash(state);
            }
            Self::FOK => {
                2_u8.hash(state);
            }
        }
    }
}
