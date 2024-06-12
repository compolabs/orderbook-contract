library;

use ::data_structures::asset_type::AssetType;
use ::data_structures::order_type::OrderType;
use ::errors::OrderError;
use std::hash::{Hash, sha256};
use std::contract_id::ContractId;

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
        sha256((sha256((ContractId::this(), self.owner)), self.amount, self.asset_type, self.order_type, self.price))
    }
}
