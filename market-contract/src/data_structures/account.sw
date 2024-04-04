library;

use ::data_structures::balance::Balance;

pub struct Account {
    // Available funds
    liquid: Balance,
    // Open orders
    locked: Balance,
}

impl Account {
    pub fn new() -> Self {
        Self {
            liquid: Balance::new(),
            locked: Balance::new(),
        }
    }
}
