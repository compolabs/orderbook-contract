library;

use ::data_structures::balance::Balance;

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
}
