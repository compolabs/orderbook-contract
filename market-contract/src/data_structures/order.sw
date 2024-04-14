library;

use ::data_structures::asset_type::AssetType;
use ::data_structures::order_type::OrderType;
use ::errors::OrderError;
use std::hash::{Hash, sha256};

pub struct Order {
    base_amount: u64,
    order_type: OrderType,
    owner: Identity,
    price: u64,
}

impl Order {
    pub fn new(
        base_amount: u64,
        order_type: OrderType,
        owner: Identity,
        price: u64,
    ) -> Self {
        require(base_amount != 0, OrderError::AmountCannotBeZero);
        require(price != 0, OrderError::PriceCannotBeZero);

        Self {
            base_amount,
            order_type,
            owner,
            price,
        }
    }

    pub fn id(self) -> b256 {
        // TODO: include asset type in id?
        sha256((self.base_amount, self.order_type, self.owner, self.price))
    }

    pub fn set_amount(ref mut self, amount: u64) {
        require(amount != 0, OrderError::AmountCannotBeZero);
        self.base_amount = amount;
    }

    pub fn set_price(ref mut self, price: u64) {
        require(price != 0, OrderError::PriceCannotBeZero);
        self.price = price;
    }

    //     #[storage(read)]
    //     pub fn calculate_deposit(
    //         self,
    //         BASE_ASSET_DECIMALS: u32,
    //         PRICE_DECIMALS: u32,
    //         QUOTE_TOKEN_DECIMALS: u32,
    //         QUOTE_TOKEN: AssetId,
    // ) -> Asset {
    //         match self.order_type {
    //             OrderType::Sell => Asset::new(self.amount, self.asset),
    //             OrderType::Buy => {
    //                 Asset::new(
    //                     quote(
    //                         self.amount,
    //                         BASE_ASSET_DECIMALS,
    //                         self.price,
    //                         PRICE_DECIMALS,
    //                         QUOTE_TOKEN_DECIMALS,
    //                     ),
    //                     QUOTE_TOKEN,
    //                 )
    //             },
    //         }
    //     }
}
