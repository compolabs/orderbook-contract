use orderbook::test_utils::*;
pub use pretty_assertions::assert_eq;

const PRICE_DECIMALS: u64 = 9;
#[tokio::test]
async fn open_base_token_order_cancel_test() {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];
    let user = &wallets[1];

    let token_contract = deploy_token_contract(&admin).await;
    let uni = Asset::new(admin.clone(), token_contract.contract_id().into(), "UNI");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    // Create Market
    orderbook
        ._create_market(uni.asset_id, uni.decimals as u32)
        .await
        .unwrap();

    let response = orderbook.market_exists(uni.asset_id).await.unwrap();
    assert_eq!(true, response.value);

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();
    assert_eq!(0, response.value.len());

    // SELL 5uni, price 50000
    let price = 50000;
    let univ: f64 = -5.0;

    let base_price = price * 10u64.pow(PRICE_DECIMALS as u32);
    let base_size_sell1 = uni.parse_units(univ) as i64; //? тут мы имеем i64 а не f64 потому что мы уже домнлжили на scale
    let amount_uni = base_size_sell1.abs() as u64;

    // Mint uni
    uni.mint(admin.address().into(), amount_uni).await.unwrap();
    let balance = admin.get_asset_balance(&uni.asset_id).await.unwrap();
    assert_eq!(balance, amount_uni);

    // Open order
    orderbook
        .open_order(uni.asset_id, base_size_sell1, base_price)
        .await
        .unwrap();

    assert_eq!(admin.get_asset_balance(&uni.asset_id).await.unwrap(), 0);

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook.order_by_id(order_id).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_sell1, order.base_size.as_i64());

    // Add uni value to order
    uni.mint(admin.address().into(), amount_uni).await.unwrap();

    orderbook
        .open_order(uni.asset_id, base_size_sell1, base_price)
        .await
        .unwrap();

    assert_eq!(admin.get_asset_balance(&uni.asset_id).await.unwrap(), 0);

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook.order_by_id(order_id).await.unwrap();

    let base_size_sell2 = base_size_sell1 * 2;

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_sell2, order.base_size.as_i64());

    // BUY 5uni, price 5000
    let univ = 5.0;
    let usdv = 5.0 * price as f64; // 250k usdc

    let base_size_buy1 = uni.parse_units(univ) as i64;
    let quote_size_buy1 = usdc.parse_units(usdv) as i64;
    let amount_usdc = quote_size_buy1 as u64;

    // Mint USDC
    usdc.mint(admin.address().into(), amount_usdc)
        .await
        .unwrap();

    let balance = admin.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert_eq!(balance, amount_usdc);

    // Add usdc value to order
    orderbook
        .open_order(uni.asset_id, base_size_buy1, base_price)
        .await
        .unwrap();

    let balance = admin.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert_eq!(balance, amount_usdc);

    let balance = admin.get_asset_balance(&uni.asset_id).await.unwrap();
    assert_eq!(balance, amount_uni);

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook.order_by_id(order_id).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_sell1, order.base_size.as_i64());

    // Mint USDC
    usdc.mint(admin.address().into(), amount_usdc)
        .await
        .unwrap();

    let balance = admin.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert_eq!(balance, amount_usdc * 2);

    // Add more usdc value to order
    let base_size_buy2 = base_size_buy1 * 2;

    orderbook
        .open_order(uni.asset_id, base_size_buy2.clone(), base_price)
        .await
        .unwrap();

    let balance = admin.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert_eq!(balance, amount_usdc);

    let balance = admin.get_asset_balance(&uni.asset_id).await.unwrap();
    assert_eq!(balance, amount_uni * 2);

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();
    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook.order_by_id(order_id).await.unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_buy1, order.base_size.as_i64());

    // Cancel by not order owner
    orderbook
        .with_account(user)
        .cancel_order(order_id)
        .await
        .expect_err("Order cancelled by another user");

    // Cancel order
    orderbook.cancel_order(order_id).await.unwrap();

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();
    assert_eq!(0, response.value.len());

    let response = orderbook.order_by_id(order_id).await.unwrap();
    assert!(response.value.is_none());

    let balance = admin.get_asset_balance(&uni.asset_id).await.unwrap();
    assert_eq!(balance, 2 * amount_uni);

    let balance = admin.get_asset_balance(&usdc.asset_id).await.unwrap();
    assert_eq!(balance, 2 * amount_usdc);
}

#[tokio::test]
async fn open_quote_token_order_cancel_by_reverse_order_test() {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];

    let token_contract = deploy_token_contract(&admin).await;
    let uni = Asset::new(admin.clone(), token_contract.contract_id().into(), "UNI");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    // Create Market
    orderbook
        ._create_market(uni.asset_id, uni.decimals as u32)
        .await
        .unwrap();

    let response = orderbook.market_exists(uni.asset_id).await.unwrap();
    assert_eq!(true, response.value);

    let response = orderbook.orders_by_trader(admin.address()).await.unwrap();

    assert_eq!(0, response.value.len());

    // Mint uni & USDC

    let usdv = 250000.0;
    let univ = 5.0;
    let price = 50000;
    let base_price = price * 10u64.pow(PRICE_DECIMALS as u32);
    let base_size_buy1 = uni.parse_units(univ) as i64;
    let amount_uni = base_size_buy1.abs() as u64;
    let base_size_sell1 = -base_size_buy1;
    let amount_usdc = usdc.parse_units(usdv) as u64;

    usdc.mint(admin.address().into(), amount_usdc)
        .await
        .unwrap();
    uni.mint(admin.address().into(), amount_uni).await.unwrap();

    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );
    assert_eq!(
        admin.get_asset_balance(&uni.asset_id).await.unwrap(),
        amount_uni
    );

    // Open order

    orderbook
        .open_order(uni.asset_id, base_size_buy1.clone(), base_price)
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
        .open_order(uni.asset_id, base_size_sell1.clone(), base_price)
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
        admin.get_asset_balance(&uni.asset_id).await.unwrap(),
        amount_uni
    );
}

#[tokio::test]
async fn match_test() {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];
    let user1 = &wallets[1];
    let user2 = &wallets[2];

    let token_contract = deploy_token_contract(&admin).await;
    let uni = Asset::new(admin.clone(), token_contract.contract_id().into(), "UNI");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    // Create Market
    orderbook
        ._create_market(uni.asset_id, uni.decimals as u32)
        .await
        .unwrap();

    // Mint uni & USDC

    let usdv = 250000.0;
    let univ = 5.0;
    let price = 50000;
    let base_price = price * 10u64.pow(PRICE_DECIMALS as u32);
    let base_size_buy1 = uni.parse_units(univ) as i64;
    let amount_uni = base_size_buy1.abs() as u64;
    let base_size_sell1 = -base_size_buy1;
    let amount_usdc = usdc.parse_units(usdv) as u64;

    usdc.mint(user1.address().into(), amount_usdc)
        .await
        .unwrap();
    uni.mint(user2.address().into(), amount_uni).await.unwrap();

    assert_eq!(
        user1.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );
    assert_eq!(
        user2.get_asset_balance(&uni.asset_id).await.unwrap(),
        amount_uni
    );

    // Open USDC order

    orderbook
        .with_account(user1)
        .open_order(uni.asset_id, base_size_buy1.clone(), base_price)
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

    // Open uni order

    orderbook
        .with_account(user2)
        .open_order(uni.asset_id, base_size_sell1.clone(), base_price)
        .await
        .unwrap();

    assert_eq!(user2.get_asset_balance(&uni.asset_id).await.unwrap(), 0);

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
        user1.get_asset_balance(&uni.asset_id).await.unwrap(),
        amount_uni
    );
}
