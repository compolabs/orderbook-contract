library;

use ::data_structures::balance::Balance;

pub struct Account {
    // Available funds
    pub liquid: Balance,
    // Open orders
    pub locked: Balance,
}

impl Account {
    //? for waht we need to store locked
    // I suggest to store accounts like StorageMap<Address, u64>, where u64 is liquid balance
    // If you place it here to calculate tvl, we can just get a balance of contarct
    pub fn new() -> Self {
        Self {
            liquid: Balance::new(),
            locked: Balance::new(),
        }
    }
}
