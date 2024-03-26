library;

use ::errors::AuthError;
use std::hash::{Hash, sha256};

pub fn create_id(trader: Address, asset: AssetId, price: u64) -> b256 {
    sha256((trader, asset, price))
}

pub fn trader() -> Address {
    let trader = msg_sender().unwrap().as_address();
    require(trader.is_some(), AuthError::EOAOnly);
    trader.unwrap()
}
