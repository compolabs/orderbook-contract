library;

use ::data_structures::asset_type::AssetType;
use ::errors::AccountError;

pub struct Balance {
    base: u64,
    quote: u64,
}

impl Balance {
    pub fn new() -> Self {
        Self {
            base: 0,
            quote: 0,
        }
    }

    pub fn credit(ref mut self, amount: u64, asset: AssetType) {
        match asset {
            AssetType::Base => self.base += amount,
            AssetType::Quote => self.quote += amount,
        };
    }

    pub fn debit(ref mut self, amount: u64, asset: AssetType) {
        match asset {
            AssetType::Base => {
                require(amount <= self.base, AccountError::InsufficientBalance((self.base, amount)));
                self.base -= amount;
            },
            AssetType::Quote => {
                require(amount <= self.quote, AccountError::InsufficientBalance((self.quote, amount)));
                self.quote -= amount;
            }
        };
    }
}
