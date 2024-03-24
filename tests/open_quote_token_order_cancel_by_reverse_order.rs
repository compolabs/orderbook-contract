use orderbook::test_utils::*;
pub use pretty_assertions::assert_eq;
const PRICE_DECIMALS: u64 = 9;

struct TestContext {
    admin: WalletUnlocked,
    usdc: Asset,
    token: Asset,
    orderbook: Orderbook,
}
async fn setup() -> TestContext {
    let (admin, _user, _user2) = init_wallets().await;
    let (usdc, token) = init_tokens(&admin, "BTC").await;
    let orderbook = init_orderbook(&admin, &usdc, &token).await;

    TestContext {
        admin,
        usdc,
        token,
        orderbook,
    }
}

#[tokio::test]
async fn open_quote_token_order_cancel_by_reverse_order_test() {
    let context = setup().await;

    let response = context
        .orderbook
        .market_exists(context.token.asset_id)
        .await
        .unwrap();
    assert_eq!(true, response.value);

    let response = context
        .orderbook
        .orders_by_trader(context.admin.address())
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    // Mint BTC & USDC

    let usdv = 250000.0;
    let tokenv = 5.0;
    let price = 50000;
    let base_price = price * 10u64.pow(PRICE_DECIMALS as u32);
    let base_size_buy1 = context.token.parse_units(tokenv) as i64;
    let amount_token = base_size_buy1.abs() as u64;
    let base_size_sell1 = -base_size_buy1;
    let amount_usdc = context.usdc.parse_units(usdv) as u64;

    context
        .usdc
        .mint(context.admin.address().into(), amount_usdc)
        .await
        .unwrap();
    context
        .token
        .mint(context.admin.address().into(), amount_token)
        .await
        .unwrap();

    assert_eq!(
        context
            .admin
            .get_asset_balance(&context.usdc.asset_id)
            .await
            .unwrap(),
        amount_usdc
    );
    assert_eq!(
        context
            .admin
            .get_asset_balance(&context.token.asset_id)
            .await
            .unwrap(),
        amount_token
    );

    // Open order

    context
        .orderbook
        .open_order(context.token.asset_id, base_size_buy1.clone(), base_price)
        .await
        .unwrap();

    assert_eq!(
        context
            .admin
            .get_asset_balance(&context.usdc.asset_id)
            .await
            .unwrap(),
        0
    );

    let response = context
        .orderbook
        .orders_by_trader(context.admin.address())
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = context.orderbook.order_by_id(order_id).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_buy1, order.base_size.value as i64);
    assert!(!order.base_size.negative);

    context
        .orderbook
        .open_order(context.token.asset_id, base_size_sell1.clone(), base_price)
        .await
        .unwrap();

    let response = context
        .orderbook
        .orders_by_trader(context.admin.address())
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    let response = context.orderbook.order_by_id(order_id).await.unwrap();

    assert!(response.value.is_none());

    assert_eq!(
        context
            .admin
            .get_asset_balance(&context.usdc.asset_id)
            .await
            .unwrap(),
        amount_usdc
    );
    assert_eq!(
        context
            .admin
            .get_asset_balance(&context.token.asset_id)
            .await
            .unwrap(),
        amount_token
    );
}
