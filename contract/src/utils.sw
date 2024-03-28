library;

use ::errors::AuthError;
use std::hash::{Hash, sha256};

pub fn trader() -> Address {
    let trader = msg_sender().unwrap().as_address();
    require(trader.is_some(), AuthError::EOAOnly);
    trader.unwrap()
}
