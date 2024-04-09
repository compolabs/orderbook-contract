pub use fuels::types::Bits256;
pub use fuels::{accounts::wallet::Wallet, prelude::*};
use orderbook::orderbook_utils::Orderbook;
pub use src20_sdk::token_utils::Asset;
pub use std::result::Result;

pub async fn mint_tokens(
    usdc: &Asset,
    token: &Asset,
    alice: &Wallet,
    bob: &Wallet,
    usdc_mint_amount: u64,
    token_mint_amount: u64,
) {
    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();
    token
        .mint(bob.address().into(), token_mint_amount)
        .await
        .unwrap();
}

pub async fn open_orders_match(
    orderbook: &Orderbook,
    alice: &WalletUnlocked,
    bob: &WalletUnlocked,
    token: &Asset,
    buy_size: f64,
    buy_price: f64,
    sell_size: f64,
    sell_price: f64,
) -> Result<(Bits256, Bits256), fuels::types::errors::Error> {
    let alice_order_id = orderbook
        .with_account(&alice)
        .open_order(
            token.asset_id,
            (buy_size * 1e8) as i64,
            (buy_price * 1e9) as u64,
        )
        .await
        .unwrap()
        .value;

    let bob_order_id = orderbook
        .with_account(&bob)
        .open_order(
            token.asset_id,
            (sell_size * 1e8) as i64,
            (sell_price * 1e9) as u64,
        )
        .await
        .unwrap()
        .value;

    let res = orderbook.match_orders(&bob_order_id, &alice_order_id).await;
    if res.is_ok() {
        Ok((alice_order_id, bob_order_id))
    } else {
        Err(res.err().unwrap())
    }
}
