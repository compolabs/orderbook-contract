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
    pub block_height: u32,
}

impl Order {
    pub fn new(
        amount: u64,
        asset_type: AssetType,
        order_type: OrderType,
        owner: Identity,
        price: u64,
        price_decimals: u32,
        block_height: u32,
    ) -> Self {
        require(amount != 0, OrderError::AmountCannotBeZero);
        require(
            price >= 10_u64
                .pow(price_decimals),
            OrderError::PriceTooSmall((price, 10_u64.pow(price_decimals))),
        );

        Self {
            amount,
            asset_type,
            order_type,
            owner,
            price,
            block_height,
        }
    }

    pub fn id(self) -> b256 {
        sha256((
            sha256((ContractId::this(), self.owner)),
            self.asset_type,
            self.order_type,
            self.price,
            self.block_height,
        ))
    }
}
