library;

use ::data_structures::asset_type::AssetType;
use ::data_structures::math::{HUNDRED_PERCENT, max, min};
use ::data_structures::order_type::OrderType;
use ::errors::{AssetError, OrderError};

use std::hash::{Hash, sha256};
use std::contract_id::ContractId;

pub struct Order {
    pub amount: u64,
    pub asset_type: AssetType,
    pub order_type: OrderType,
    pub owner: Identity,
    pub price: u64,
    pub block_height: u32,
    pub order_height: u64,
    pub matcher_fee: u64,
    pub protocol_maker_fee: u64,
    pub protocol_taker_fee: u64,
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
        order_height: u64,
        matcher_fee: u64,
        protocol_maker_fee: u64,
        protocol_taker_fee: u64,
    ) -> Self {
        require(amount != 0, OrderError::ZeroOrderAmount);
        require(asset_type == AssetType::Base, AssetError::InvalidAsset);
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
            order_height,
            matcher_fee,
            protocol_maker_fee,
            protocol_taker_fee,
        }
    }

    pub fn id(self) -> b256 {
        sha256((
            sha256((ContractId::this(), self.owner)),
            self.asset_type,
            self.order_type,
            self.price,
            self.order_height,
        ))
    }

    pub fn min_protocol_fee_of_amount(self, amount: u64) -> u64 {
        amount * min(self.protocol_maker_fee, self.protocol_taker_fee) / HUNDRED_PERCENT
    }

    pub fn max_protocol_fee_of_amount(self, amount: u64) -> u64 {
        amount * max(self.protocol_maker_fee, self.protocol_taker_fee) / HUNDRED_PERCENT
    }

    pub fn protocol_maker_fee_of_amount(self, amount: u64) -> u64 {
        amount * self.protocol_maker_fee / HUNDRED_PERCENT
    }

    pub fn protocol_taker_fee_of_amount(self, amount: u64) -> u64 {
        amount * self.protocol_taker_fee / HUNDRED_PERCENT
    }

    pub fn protocol_fee_of_amount(self, counterparty: Self, amount: u64) -> u64 {
        if self.block_height < counterparty.block_height {
            self.protocol_maker_fee_of_amount(amount)
        } else {
            self.protocol_taker_fee_of_amount(amount)
        }
    }

    pub fn matcher_fee_of_amount(self, amount: u64) -> u64 {
        self.matcher_fee * amount / self.amount
    }
}
