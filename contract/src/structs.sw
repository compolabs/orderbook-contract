library;
use i64::I64;
use std::hash::{Hash, sha256};
use ::errors::Error;

pub struct Order {
    // id: b256,
    trader: Address,
    base_token: AssetId,
    base_size: I64,
    base_price: u64,
}

impl Order {
    pub fn new(
        trader: Address,
        base_token: AssetId,
        base_size: I64,
        base_price: u64,
    
    ) -> Self {
        require(base_size.value != 0, Error::AmountCannotBeZero);
        require(base_price != 0, Error::PriceCannotBeZero);

        Self {
            trader,
            base_token,
            base_size,
            base_price,
           
        }
    }
        
    pub fn id(self) -> b256 {
        sha256((self.trader, self.base_token, self.base_price))
    }
}


pub struct Market {
    asset_id: AssetId,
    asset_decimals: u32,
}
