library;

use ::data_structures::{order_type::OrderType, order::Order};
use ::errors::TradeError;

use std::u128::U128;

impl u64 {
    pub fn mul_div(self, mul_to: u64, div_to: u64) -> u64 {
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_result = mul_result / U128::from((0, div_to));
        div_result.as_u64().unwrap()
    }

    pub fn mul_div_rounding_up(self, mul_to: u64, div_to: u64) -> u64 {
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_to = U128::from((0, div_to));
        let div_result = mul_result / div_to;
        let add = if div_result * div_to < mul_result {
            1
        } else {
            0
        };
        div_result.as_u64().unwrap() + add
    }
}

pub fn attempt_trade(alice: Order, bob: Order) -> Result<(u64, u64), TradeError> {
    let mut trade = Result::Err(TradeError::CannotTrade);
    match alice.order_type {
        OrderType::Sell => {
            if bob.price < alice.price {
                return trade;
            }

        }
        OrderType::Buy => {
            if alice.price < bob.price {
                return trade;
            }
        }
    }

    // When OrderType::Sell return 
    //  - how much alice has sold in her asset denomination
    //  - how much bob has purchased in his asset denomination
    //  - Result::Ok((alice sold amount, bob purchased amount))
    // When OrderType::Buy return
    //  - how much alice purchased in her asset denomination
    //  - how much bob sold in his asset denomination
    //  - Result::Ok((alice purchased amount, bob sold amount))
    trade
}

pub fn base_to_quote_amount(
    amount: u64, 
    base_asset_decimals: u32, 
    base_price: u64,
    price_decimals: u32,
    quote_asset_decimals: u32,
) -> u64 {
    amount.mul_div(
        base_price,
        10_u64
            .pow(base_asset_decimals + price_decimals - quote_asset_decimals),
    )
}

pub fn quote_to_base_amount(
    amount: u64, 
    base_asset_decimals: u32, 
    base_price: u64,
    price_decimals: u32,
    quote_asset_decimals: u32,
) -> u64 {
    amount.mul_div(
        10_u64.pow(base_asset_decimals + price_decimals - quote_asset_decimals),
        base_price,
    )
}
