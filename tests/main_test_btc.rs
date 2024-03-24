use orderbook::test_utils::*;
pub use pretty_assertions::assert_eq;
const PRICE_DECIMALS: u64 = 9;

struct TestContext {
    admin: WalletUnlocked,
    user: WalletUnlocked,
    usdc: Asset,
    token: Asset,
    orderbook: Orderbook,
}
async fn setup() -> TestContext {
    let (admin, user, _user2) = init_wallets().await;
    let (usdc, token) = init_tokens(&admin, "BTC").await;
    let orderbook = init_orderbook(&admin, &usdc, &token).await;

    TestContext {
        admin,
        user,
        usdc,
        token,
        orderbook,
    }
}
#[tokio::test]
async fn open_base_token_order_cancel_test() {
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

    // SELL 5btc, price 50000
    let price = 50000;
    let btcv: f64 = -5.0;

    let base_price = price * 10u64.pow(PRICE_DECIMALS as u32);
    let base_size_sell1 = context.token.parse_units(btcv) as i64; //? тут мы имеем i64 а не f64 потому что мы уже домнлжили на scale
    let amount_btc = base_size_sell1.abs() as u64;

    // Mint BTC
    context
        .token
        .mint(context.admin.address().into(), amount_btc)
        .await
        .unwrap();

    let balance = context
        .admin
        .get_asset_balance(&context.token.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, amount_btc);

    // Open order
    context
        .orderbook
        .open_order(context.token.asset_id, base_size_sell1, base_price)
        .await
        .unwrap();

    assert_eq!(
        context
            .admin
            .get_asset_balance(&context.token.asset_id)
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
    assert_eq!(base_size_sell1, order.base_size.as_i64());

    // Add btc value to order
    context
        .token
        .mint(context.admin.address().into(), amount_btc)
        .await
        .unwrap();

    context
        .orderbook
        .open_order(context.token.asset_id, base_size_sell1, base_price)
        .await
        .unwrap();

    assert_eq!(
        context
            .admin
            .get_asset_balance(&context.token.asset_id)
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

    let base_size_sell2 = base_size_sell1 * 2;

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_sell2, order.base_size.as_i64());

    // BUY 5btc, price 5000
    let btcv = 5.0;
    let usdv = 5.0 * price as f64; // 250k usdc

    let base_size_buy1 = context.token.parse_units(btcv) as i64;
    let quote_size_buy1 = context.usdc.parse_units(usdv) as i64;
    let amount_usdc = quote_size_buy1 as u64;

    // Mint USDC
    context
        .usdc
        .mint(context.admin.address().into(), amount_usdc)
        .await
        .unwrap();

    let balance = context
        .admin
        .get_asset_balance(&context.usdc.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, amount_usdc);

    // Add usdc value to order
    context
        .orderbook
        .open_order(context.token.asset_id, base_size_buy1, base_price)
        .await
        .unwrap();

    let balance = context
        .admin
        .get_asset_balance(&context.usdc.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, amount_usdc);

    let balance = context
        .admin
        .get_asset_balance(&context.token.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, amount_btc);

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
    assert_eq!(base_size_sell1, order.base_size.as_i64());

    // Mint USDC
    context
        .usdc
        .mint(context.admin.address().into(), amount_usdc)
        .await
        .unwrap();

    let balance = context
        .admin
        .get_asset_balance(&context.usdc.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, amount_usdc * 2);

    // Add more usdc value to order
    let base_size_buy2 = base_size_buy1 * 2;

    context
        .orderbook
        .open_order(context.token.asset_id, base_size_buy2.clone(), base_price)
        .await
        .unwrap();

    let balance = context
        .admin
        .get_asset_balance(&context.usdc.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, amount_usdc);

    let balance = context
        .admin
        .get_asset_balance(&context.token.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, amount_btc * 2);

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
    assert_eq!(base_size_buy1, order.base_size.as_i64());

    // Cancel by not order owner
    context
        .orderbook
        .with_account(&context.user)
        .cancel_order(order_id)
        .await
        .expect_err("Order cancelled by another user");

    // Cancel order
    context.orderbook.cancel_order(order_id).await.unwrap();

    let response = context
        .orderbook
        .orders_by_trader(context.admin.address())
        .await
        .unwrap();
    assert_eq!(0, response.value.len());

    let response = context.orderbook.order_by_id(order_id).await.unwrap();
    assert!(response.value.is_none());

    let balance = context
        .admin
        .get_asset_balance(&context.token.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, 2 * amount_btc);

    let balance = context
        .admin
        .get_asset_balance(&context.usdc.asset_id)
        .await
        .unwrap();
    assert_eq!(balance, 2 * amount_usdc);
}

#[tokio::test]
async fn open_quote_token_order_cancel_by_reverse_order_test() {
    let (admin, _user, _user2) = init_wallets().await;

    let token_contract = deploy_token_contract(&admin).await;
    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    // Create Market
    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .unwrap();

    let response = orderbook.market_exists(btc.asset_id).await.unwrap();
    assert_eq!(true, response.value);

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();

    assert_eq!(0, response.value.len());

    // Mint BTC & USDC

    let usdv = 250000.0;
    let btcv = 5.0;
    let price = 50000;
    let base_price = price * 10u64.pow(PRICE_DECIMALS as u32);
    let base_size_buy1 = btc.parse_units(btcv) as i64;
    let amount_btc = base_size_buy1.abs() as u64;
    let base_size_sell1 = -base_size_buy1;
    let amount_usdc = usdc.parse_units(usdv) as u64;

    usdc.mint(admin.address().into(), amount_usdc)
        .await
        .unwrap();
    btc.mint(admin.address().into(), amount_btc).await.unwrap();

    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );
    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount_btc
    );

    // Open order

    orderbook
        .open_order(btc.asset_id, base_size_buy1.clone(), base_price)
        .await
        .unwrap();

    assert_eq!(admin.get_asset_balance(&usdc.asset_id).await.unwrap(), 0);

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook.order_by_id(order_id).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_buy1, order.base_size.value as i64);
    assert!(!order.base_size.negative);

    orderbook
        .open_order(btc.asset_id, base_size_sell1.clone(), base_price)
        .await
        .unwrap();

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();

    assert_eq!(0, response.value.len());

    let response = orderbook.order_by_id(order_id).await.unwrap();

    assert!(response.value.is_none());

    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );
    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount_btc
    );
}

#[tokio::test]
async fn match_test() {
    let (admin, user1, user2) = init_wallets().await;

    let token_contract = deploy_token_contract(&admin).await;
    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    // Create Market
    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .unwrap();

    // Mint BTC & USDC

    let usdv = 250000.0;
    let btcv = 5.0;
    let price = 50000;
    let base_price = price * 10u64.pow(PRICE_DECIMALS as u32);
    let base_size_buy1 = btc.parse_units(btcv) as i64;
    let amount_btc = base_size_buy1.abs() as u64;
    let base_size_sell1 = -base_size_buy1;
    let amount_usdc = usdc.parse_units(usdv) as u64;

    usdc.mint(user1.address().into(), amount_usdc)
        .await
        .unwrap();
    btc.mint(user2.address().into(), amount_btc).await.unwrap();

    assert_eq!(
        user1.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );
    assert_eq!(
        user2.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount_btc
    );

    // Open USDC order

    orderbook
        .with_account(&user1)
        .open_order(btc.asset_id, base_size_buy1.clone(), base_price)
        .await
        .unwrap();

    assert_eq!(user1.get_asset_balance(&usdc.asset_id).await.unwrap(), 0);

    let response = orderbook.orders_by_trader(user1.address()).await.unwrap();

    assert_eq!(1, response.value.len());

    let order_id_1 = response.value.get(0).unwrap();
    let response = orderbook.order_by_id(order_id_1).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_buy1, order.base_size.value as i64);
    assert!(!order.base_size.negative);

    // Open BTC order

    orderbook
        .with_account(&user2)
        .open_order(btc.asset_id, base_size_sell1.clone(), base_price)
        .await
        .unwrap();

    assert_eq!(user2.get_asset_balance(&btc.asset_id).await.unwrap(), 0);

    let response = orderbook.orders_by_trader(user2.address()).await.unwrap();

    assert_eq!(1, response.value.len());

    let order_id_2 = response.value.get(0).unwrap();
    let response = orderbook.order_by_id(order_id_2).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_sell1, order.base_size.value as i64 * (-1));
    assert!(order.base_size.negative);

    // Match orders
    orderbook
        .match_orders(order_id_2, order_id_1)
        .await
        .unwrap();

    let response = orderbook.orders_by_trader(user1.address()).await.unwrap();

    assert_eq!(0, response.value.len());

    let response = orderbook.orders_by_trader(user2.address()).await.unwrap();

    assert_eq!(0, response.value.len());

    assert_eq!(
        user2.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );
    assert_eq!(
        user1.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount_btc
    );
}
