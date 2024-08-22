library;

use ::errors::OrderError;
use ::data_structures::{asset_type::AssetType, balance::Balance,};

pub struct Account {
    // Available funds
    pub liquid: Balance,
    // Open orders
    pub locked: Balance,
}

impl Account {
    pub fn new() -> Self {
        Self {
            liquid: Balance::new(),
            locked: Balance::new(),
        }
    }

    pub fn lock_amount(ref mut self, amount: u64, asset: AssetType) {
        require(amount != 0, OrderError::ZeroLockAmount);
        self.liquid.debit(amount, asset);
        self.locked.credit(amount, asset);
    }

    pub fn unlock_amount(ref mut self, amount: u64, asset: AssetType) {
        require(amount != 0, OrderError::ZeroLockAmount);
        self.liquid.credit(amount, asset);
        self.locked.debit(amount, asset);
    }

    pub fn transfer_locked_amount(
        ref mut self,
        ref mut to: Account,
        amount: u64,
        asset: AssetType,
) {
        require(amount != 0, OrderError::ZeroLockAmount);
        to.liquid.credit(amount, asset);
        self.locked.debit(amount, asset);
    }
}
