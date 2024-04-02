library;

use std::hash::{Hash, Hasher};

pub enum AssetType {
    Base: (),
    Quote: (),
}

//!for waht we are using hash impl here?
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
