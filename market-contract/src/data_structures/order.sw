library;

use ::data_structures::asset_type::AssetType;
use ::data_structures::order_type::OrderType;
use ::errors::OrderError;
use std::hash::{Hash, sha256};

pub struct Order {
    pub amount: u64,
    pub asset_type: AssetType,
    pub order_type: OrderType,
    pub owner: Identity,
    pub price: u64,
}

impl Order {
    pub fn new(
        amount: u64,
        asset_type: AssetType,
        order_type: OrderType,
        owner: Identity,
        price: u64,
    ) -> Self {
        require(amount != 0, OrderError::AmountCannotBeZero);
        require(price != 0, OrderError::PriceCannotBeZero);

        Self {
            amount,
            asset_type,
            order_type,
            owner,
            price,
        }
    }

    pub fn id(self) -> b256 {
        // TODO: it mignt be need include market contract_id here
        sha256((self.amount, self.asset_type, self.order_type, self.owner, self.price))
    }

    pub fn set_amount(ref mut self, amount: u64) {
        require(amount != 0, OrderError::AmountCannotBeZero);
        self.amount = amount;
    }

    pub fn set_price(ref mut self, price: u64) {
        require(price != 0, OrderError::PriceCannotBeZero);
        self.price = price;
    }
}
