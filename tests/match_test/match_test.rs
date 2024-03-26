use fuels::accounts::{wallet::WalletUnlocked, ViewOnlyAccount};
use orderbook::orderbook_utils::Orderbook;
pub use pretty_assertions::assert_eq;
use src20_sdk::token_utils::Asset;

use crate::utils::setup::{init_orderbook, init_tokens, init_wallets};

const PRICE_DECIMALS: u64 = 9;

struct TestContext {
    user1: WalletUnlocked,
    user2: WalletUnlocked,
    usdc: Asset,
    token: Asset,
    orderbook: Orderbook,
}
async fn setup() -> TestContext {
    let (admin, user1, user2) = init_wallets().await;
    let (usdc, token) = init_tokens(&admin, "BTC").await;
    let orderbook = init_orderbook(&admin, &usdc, &token).await;

    TestContext {
        user1,
        user2,
        usdc,
        token,
        orderbook,
    }
}

#[tokio::test]
async fn match_test() {
    let context = setup().await;

    let usdv = 250000.0;
    let btcv = 5.0;
    let price = 50000;
    let base_price = price * 10u64.pow(PRICE_DECIMALS as u32);
    let base_size_buy1 = context.token.parse_units(btcv) as i64;
    let amount_token = base_size_buy1.abs() as u64;
    let base_size_sell1 = -base_size_buy1;
    let amount_usdc = context.usdc.parse_units(usdv) as u64;

    context
        .usdc
        .mint(context.user1.address().into(), amount_usdc)
        .await
        .unwrap();
    context
        .token
        .mint(context.user2.address().into(), amount_token)
        .await
        .unwrap();

    assert_eq!(
        context
            .user1
            .get_asset_balance(&context.usdc.asset_id)
            .await
            .unwrap(),
        amount_usdc
    );
    assert_eq!(
        context
            .user2
            .get_asset_balance(&context.token.asset_id)
            .await
            .unwrap(),
        amount_token
    );

    // Open USDC order

    context
        .orderbook
        .with_account(&context.user1)
        .open_order(context.token.asset_id, base_size_buy1.clone(), base_price)
        .await
        .unwrap();

    assert_eq!(
        context
            .user1
            .get_asset_balance(&context.usdc.asset_id)
            .await
            .unwrap(),
        0
    );

    let response = context
        .orderbook
        .orders_by_trader(context.user1.address())
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id_1 = response.value.get(0).unwrap();
    let response = context.orderbook.order_by_id(order_id_1).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_buy1, order.base_size.value as i64);
    assert!(!order.base_size.negative);

    // Open BTC order

    context
        .orderbook
        .with_account(&context.user2)
        .open_order(context.token.asset_id, base_size_sell1.clone(), base_price)
        .await
        .unwrap();

    assert_eq!(
        context
            .user2
            .get_asset_balance(&context.token.asset_id)
            .await
            .unwrap(),
        0
    );

    let response = context
        .orderbook
        .orders_by_trader(context.user2.address())
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id_2 = response.value.get(0).unwrap();
    let response = context.orderbook.order_by_id(order_id_2).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_sell1, order.base_size.value as i64 * (-1));
    assert!(order.base_size.negative);

    // Match orders
    context
        .orderbook
        .match_orders(order_id_2, order_id_1)
        .await
        .unwrap();

    let response = context
        .orderbook
        .orders_by_trader(context.user1.address())
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    let response = context
        .orderbook
        .orders_by_trader(context.user2.address())
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    assert_eq!(
        context
            .user2
            .get_asset_balance(&context.usdc.asset_id)
            .await
            .unwrap(),
        amount_usdc
    );
    assert_eq!(
        context
            .user1
            .get_asset_balance(&context.token.asset_id)
            .await
            .unwrap(),
        amount_token
    );
}
