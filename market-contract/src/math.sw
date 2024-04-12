library;

use ::data_structures::{asset_type::AssetType, order::Order, order_type::OrderType};
use ::errors::TradeError;

use std::u128::U128;

use i64::I64;

impl u64 {
    pub fn mul_div(self, mul_to: u64, div_to: u64) -> u64 {
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_result = mul_result / U128::from((0, div_to));
        div_result.as_u64().unwrap()
    }

    pub fn mul_div_rounding_up(self, mul_to: u64, div_to: u64) -> u64{
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_to = U128::from((0, div_to));
        let div_result = mul_result / div_to;
        let add = if div_result * div_to < mul_result {1} else {0};
        div_result.as_u64().unwrap() + add
    }

    pub fn as_i64(self) -> I64 {
        I64::from(self)
    }

    pub fn as_neg_i64(self) -> I64 {
        I64::neg_from(self)
    }
}

pub fn max(a: u64, b: u64) -> u64 {
    if a > b { a } else { b }
}

pub fn min(a: u64, b: u64) -> u64 {
    if a < b { a } else { b }
}

// impl u64 {
//     pub fn mul_div(self, mul_to: u64, div_to: u64) -> u64 {
//         let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
//         let div_result = mul_result / U128::from((0, div_to));
//         div_result.as_u64().unwrap()
//     }

//     pub fn mul_div_rounding_up(self, mul_to: u64, div_to: u64) -> u64 {
//         let div_to = U128::from((0, div_to));
//         let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
//         let div_result = mul_result / div_to;
//         let add = if div_result * div_to < mul_result {
//             1
//         } else {
//             0
//         };
//         div_result.as_u64().unwrap() + add
//     }
// }

// fn calc_amount(buy_amount: u64, buy_price: u64, sell_price: u64) -> u64 {
//     let price_ratio = U128::from((0, buy_price)) / U128::from((0, sell_price));
//     let amount = price_ratio * U128::from((0, buy_amount));
//     amount.as_u64().unwrap()
// }

// pub fn attempt_trade(
//     alice: Order,
//     bob: Order,
//     base_asset_decimals: u32,
//     quote_asset_decimals: u32,
//     price_decimals: u32,
// ) -> Result<(u64, u64, u64, u64), TradeError> {
//     // In this function:   
//     //  Decrease the order size for alice and bob until they are 0 == their orders are fulfilled
//     //  Track the amount that each account has to transfer for their trade

//     // To keep the code DRY (do not repeat yourself) force the seller to be left side, buyer right
//     let (
//         seller,
//         buyer,
//         mut seller_order_amount_decrease,
//         mut seller_account_delta,
//         mut buyer_order_amount_decrease,
//         mut buyer_account_delta,
//     ) = match alice.order_type {
//         OrderType::Sell => (alice, bob, 0, 0, 0, 0),
//         OrderType::Buy => (bob, alice, 0, 0, 0, 0),
//     };

//     if buyer.price < seller.price {
//         return Result::Err(TradeError::CannotTrade);
//     }

//     match seller.asset_type {
//         AssetType::Base => {
//             let buyer_buy_amount = calc_amount(buyer.amount, buyer.price, seller.price);
//             if seller.amount < buyer_buy_amount {
//                 seller_order_amount_decrease = seller.amount;
//                 buyer_order_amount_decrease = seller.amount;
//                 buyer_account_delta = base_to_quote_amount(
//                     buyer_order_amount_decrease,
//                     base_asset_decimals,
//                     seller
//                         .price,
//                     price_decimals,
//                     quote_asset_decimals,
//                 ) * quote_asset_decimals.as_u64();
//             } else if buyer_buy_amount < seller.amount {
//                 seller_order_amount_decrease = buyer_buy_amount;
//                 buyer_order_amount_decrease = buyer_buy_amount;
//                 buyer_account_delta = base_to_quote_amount(
//                     buyer_order_amount_decrease,
//                     base_asset_decimals,
//                     buyer.price,
//                     price_decimals,
//                     quote_asset_decimals,
//                 ) * quote_asset_decimals.as_u64();
//             } else {
//                 seller_order_amount_decrease = buyer_buy_amount;
//                 buyer_order_amount_decrease = buyer_buy_amount;
//                 buyer_account_delta = base_to_quote_amount(
//                     buyer_order_amount_decrease,
//                     base_asset_decimals,
//                     buyer.price,
//                     price_decimals,
//                     quote_asset_decimals,
//                 ) * quote_asset_decimals.as_u64();
//             }

//             seller_account_delta = seller_order_amount_decrease * base_asset_decimals.as_u64();
//         }
//         AssetType::Quote => {
//             let buyer_buy_amount = calc_amount(buyer.amount, buyer.price, seller.price);
//             if seller.amount < buyer_buy_amount {
//                 seller_order_amount_decrease = seller.amount;
//                 buyer_order_amount_decrease = seller.amount;
//                 buyer_account_delta = quote_to_base_amount(
//                     buyer_order_amount_decrease,
//                     base_asset_decimals,
//                     seller
//                         .price,
//                     price_decimals,
//                     quote_asset_decimals,
//                 ) * base_asset_decimals.as_u64();
//             } else if buyer_buy_amount < seller.amount {
//                 seller_order_amount_decrease = buyer_buy_amount;
//                 buyer_order_amount_decrease = buyer_buy_amount;
//                 buyer_account_delta = quote_to_base_amount(
//                     buyer_order_amount_decrease,
//                     base_asset_decimals,
//                     buyer.price,
//                     price_decimals,
//                     quote_asset_decimals,
//                 ) * base_asset_decimals.as_u64();
//             } else {
//                 seller_order_amount_decrease = buyer_buy_amount;
//                 buyer_order_amount_decrease = buyer_buy_amount;
//                 buyer_account_delta = quote_to_base_amount(
//                     buyer_order_amount_decrease,
//                     base_asset_decimals,
//                     buyer.price,
//                     price_decimals,
//                     quote_asset_decimals,
//                 ) * base_asset_decimals.as_u64();
//             }

//             seller_account_delta = seller_order_amount_decrease * quote_asset_decimals.as_u64();
//         }
//     };

//     // Alice must be returned first (left side arguments) then bob after for caller logic to work
//     match alice.order_type {
//         OrderType::Sell => Result::Ok((
//             seller_order_amount_decrease,
//             seller_account_delta,
//             buyer_order_amount_decrease,
//             buyer_account_delta,
//         )),
//         OrderType::Buy => Result::Ok((
//             buyer_order_amount_decrease,
//             buyer_account_delta,
//             seller_order_amount_decrease,
//             seller_account_delta,
//         )),
//     }
// }

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
        10_u64
            .pow(base_asset_decimals + price_decimals - quote_asset_decimals),
        base_price,
    )
}
