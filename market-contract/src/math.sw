library;

use ::data_structures::{asset_type::AssetType, order::Order, order_type::OrderType};

use std::u128::U128;

impl u64 {
    pub fn mul_div(self, mul_to: u64, div_to: u64) -> u64 {
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_result = mul_result / U128::from((0, div_to));
        div_result.as_u64().unwrap()
    }

    pub fn mul_div_rounding_up(self, mul_to: u64, div_to: u64) -> u64 {
        let div_to = U128::from((0, div_to));
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_result = mul_result / div_to;
        let add = if div_result * div_to < mul_result {
            1
        } else {
            0
        };
        div_result.as_u64().unwrap() + add
    }
    pub fn try_as_u32(self) -> Option<u32> {
        if self <= u32::max().as_u64() {
            Some(asm(input: self) {
                input: u32
            })
        } else {
            None
        }
    }
}

pub fn convert(
    amount: u64,
    base_asset_decimals: u32,
    base_price: u64,
    price_decimals: u32,
    quote_asset_decimals: u32,
    base_to_quote: bool,
) -> u64 {
    let (op1, op2) = (
        base_price,
        10_u64.pow(base_asset_decimals + price_decimals - quote_asset_decimals),
    );
    if base_to_quote {
        amount.mul_div(op1, op2)
    } else {
        amount.mul_div(op2, op1)
    }
}

pub fn lts(i: u64, k: u64, len: u64) -> bool {
    (i < len && k < len)
}

pub fn min(a: u64, b: u64) -> u64 {
    if a < b { a } else { b }
}

pub fn max(a: u64, b: u64) -> u64 {
    if a > b { a } else { b }
}

pub fn distance(a: u64, b: u64) -> u64 {
    if a > b { a - b } else { b - a }
}
