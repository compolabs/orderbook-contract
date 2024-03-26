library;

use i64::I64;

pub struct Asset {
    amount: u64,
    id: AssetId,
}

impl Asset {
    pub fn new(amount: u64, id: AssetId) -> Self {
        Self { amount, id }
    }
}

pub struct Order {
    trader: Address,
    asset: AssetId,
    size: I64,
    price: u64,
}

impl Order {
    pub fn new(trader: Address, asset: AssetId, size: I64, price: u64) -> Self {
        Self {
            trader,
            asset,
            size,
            price,
        }
    }

    pub fn flip(ref mut self) {
        self.size = self.size.flip();
    }
}
