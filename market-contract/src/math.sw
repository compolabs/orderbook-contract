library;

use ::data_structures::{
    account::Account,
    asset_type::AssetType,
    order::Order,
    order_type::OrderType,
};
use std::u128::U128;
use fixed_point::ufp128::UFP128;

impl u64 {
    pub fn mul_div(self, mul_to: u64, div_to: u64) -> u64 {
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_result = mul_result / U128::from((0, div_to));
        div_result.as_u64().unwrap()
    }
}

fn calc_amount(buy_amount: u64, buy_price: u64, sell_price: u64) -> u64 {
    let price_ratio = UFP128::from((0, buy_price)) / UFP128::from((0, sell_price));
    let amount = price_ratio * UFP128::from((0, buy_amount));
    U128::from(amount.into()).as_u64().unwrap()
}

pub fn attempt_trade(
    alice: Order,
    bob: Order,
    base_asset_decimals: u32,
    quote_asset_decimals: u32,
    price_decimals: u32,
    bob_account: Account,
) -> (u64, u64, u64, u64, u64) {
    // In this function:   
    //  Decrease the order size for alice and bob until they are 0 == their orders are fulfilled
    //  Track the amount that each account has to transfer for their trade
    let mut alice_order_amount_decrease = 0;
    let mut alice_account_delta = 0;
    let mut bob_order_amount_decrease = 0;
    let mut bob_account_delta = 0;
    let mut bob_unlock_amount = 0;

    match alice.asset_type {
        AssetType::Base => {
            let buyer_buy_amount = calc_amount(bob.amount, bob.price, alice.price);
            match alice.amount <= buyer_buy_amount {
                true => {
                    alice_order_amount_decrease = alice.amount;
                    bob_order_amount_decrease = alice.amount;
                    bob_account_delta = base_to_quote_amount(
                        bob_order_amount_decrease,
                        base_asset_decimals,
                        alice.price,
                        price_decimals,
                        quote_asset_decimals,
                    );
                    bob_unlock_amount = bob.price - alice.price;
                }
                false => {
                    // buyer_buy_amount < alice.amount
                    bob_account_delta = base_to_quote_amount(
                        buyer_buy_amount,
                        base_asset_decimals,
                        bob.price,
                        price_decimals,
                        quote_asset_decimals,
                    );
                    alice_order_amount_decrease = buyer_buy_amount;
                    bob_order_amount_decrease = bob.amount;

                    if bob_account.locked.quote < bob_account_delta {
                        let buyer_buy_amount = quote_to_base_amount(
                            bob_account
                                .locked
                                .quote,
                            base_asset_decimals,
                            bob.price,
                            price_decimals,
                            quote_asset_decimals,
                        );
                        alice_order_amount_decrease = buyer_buy_amount;
                        bob_order_amount_decrease = buyer_buy_amount;
                        bob_account_delta = base_to_quote_amount(
                            buyer_buy_amount,
                            base_asset_decimals,
                            bob.price,
                            price_decimals,
                            quote_asset_decimals,
                        );
                    }
                }
            }

            alice_account_delta = alice_order_amount_decrease;
        }
        AssetType::Quote => {
            let buyer_buy_amount = calc_amount(bob.amount, bob.price, alice.price);
            match alice.amount <= buyer_buy_amount {
                true => {
                    alice_order_amount_decrease = alice.amount;
                    bob_order_amount_decrease = alice.amount;
                    bob_account_delta = quote_to_base_amount(
                        bob_order_amount_decrease,
                        base_asset_decimals,
                        alice.price,
                        price_decimals,
                        quote_asset_decimals,
                    );
                    bob_unlock_amount = bob.price - alice.price;
                }
                false => {
                    // buyer_buy_amount < alice.amount
                    bob_account_delta = quote_to_base_amount(
                        buyer_buy_amount,
                        base_asset_decimals,
                        bob.price,
                        price_decimals,
                        quote_asset_decimals,
                    );
                    alice_order_amount_decrease = buyer_buy_amount;
                    bob_order_amount_decrease = bob.amount;

                    if bob_account.locked.base < bob_account_delta {
                        let buyer_buy_amount = base_to_quote_amount(
                            bob_account
                                .locked
                                .base,
                            base_asset_decimals,
                            bob.price,
                            price_decimals,
                            quote_asset_decimals,
                        );
                        alice_order_amount_decrease = buyer_buy_amount;
                        bob_order_amount_decrease = buyer_buy_amount;
                        bob_account_delta = quote_to_base_amount(
                            buyer_buy_amount,
                            base_asset_decimals,
                            bob.price,
                            price_decimals,
                            quote_asset_decimals,
                        );
                    }
                }
            }

            alice_account_delta = alice_order_amount_decrease;
        }
    };
    (
        alice_order_amount_decrease,
        alice_account_delta,
        bob_order_amount_decrease,
        bob_account_delta,
        bob_unlock_amount,
    )
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
        10_u64
            .pow(base_asset_decimals + price_decimals - quote_asset_decimals),
        base_price,
    )
}
