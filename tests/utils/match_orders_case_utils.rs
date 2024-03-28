use crate::utils::mint_tokens::{mint_tokens, open_orders_match};
use fuels::accounts::wallet::WalletUnlocked;
pub use fuels::types::Bits256;
use orderbook::orderbook_utils::Orderbook;
pub use src20_sdk::token_utils::Asset;

use crate::utils::setup::{init_orderbook, init_tokens, init_wallets};

pub struct TestContext {
    pub(crate) alice: WalletUnlocked,
    pub(crate) bob: WalletUnlocked,
    pub(crate) usdc: Asset,
    pub(crate) token: Asset,
    pub(crate) orderbook: Orderbook,
    pub(crate) alice_order_id: Bits256,
    pub(crate) bob_order_id: Bits256,
}

pub(crate) async fn match_orders_setup(
    buy_price: f64,
    sell_price: f64,
    buy_size: f64,
    sell_size: f64,
) -> TestContext {
    let (admin, alice, bob) = init_wallets().await;
    let (usdc, token) = init_tokens(&admin, "BTC").await;
    let orderbook = init_orderbook(&admin, &usdc, &token).await;

    let usdc_mint_amount = usdc.parse_units(buy_price * buy_size) as u64;
    let token_mint_amount = token.parse_units(sell_size).abs() as u64;
    mint_tokens(
        &usdc,
        &token,
        &alice,
        &bob,
        usdc_mint_amount,
        token_mint_amount,
    )
    .await;

    let (alice_order_id, bob_order_id) = open_orders_match(
        &orderbook, &alice, &bob, &token, buy_size, buy_price, sell_size, sell_price,
    )
    .await
    .expect("Failed to open and match orders");

    TestContext {
        alice,
        bob,
        usdc,
        token,
        orderbook,
        alice_order_id,
        bob_order_id,
    }
}
