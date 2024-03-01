use fuels::prelude::*;
use orderbook::orderbook_utils::{Orderbook, I64};
use src20_sdk::token_utils::{deploy_token_contract, Asset};

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
    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals).await;

    // Create Market
    orderbook
        .instance
        .methods()
        .create_market(btc.asset_id, btc.decimals as u32)
        .call()
        .await
        .unwrap();

    let response = orderbook
        .instance
        .methods()
        .market_exists(btc.asset_id)
        .call()
        .await
        .unwrap();
    assert_eq!(true, response.value);

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    // Mint BTC

    let price_decimals = 9;
    let price = 50000;
    let base_price = price * 10u64.pow(price_decimals);
    let btcv = 5;
    let amount_btc = btcv * 10u64.pow(btc.decimals.try_into().unwrap());
    let base_size_n1: I64 = I64 {
        value: amount_btc,
        negative: true,
    };

    btc.mint(admin.address().into(), amount_btc).await.unwrap();

    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount_btc
    );

    // Open order

    let call_params = CallParameters::default()
        .with_asset_id(btc.asset_id)
        .with_amount(amount_btc);

    orderbook
        .instance
        .methods()
        .open_order(btc.asset_id, base_size_n1.clone(), base_price)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    assert_eq!(admin.get_asset_balance(&btc.asset_id).await.unwrap(), 0);

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_n1, order.base_size);

    // Add btc value to order
    btc.mint(admin.address().into(), amount_btc).await.unwrap();

    let call_params = CallParameters::default()
        .with_asset_id(btc.asset_id)
        .with_amount(amount_btc);

    orderbook
        .instance
        .methods()
        .open_order(btc.asset_id, base_size_n1.clone(), base_price)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    assert_eq!(admin.get_asset_balance(&btc.asset_id).await.unwrap(), 0);

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

    let base_size_n2: I64 = I64 {
        value: 2 * amount_btc,
        negative: true,
    };

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_n2, order.base_size);

    // Mint USDC

    let usd = 250000;
    let amount_usdc = usd * 10u64.pow(usdc.decimals.try_into().unwrap());
    let base_size_p1: I64 = I64 {
        value: amount_btc,
        negative: false,
    };

    usdc.mint(admin.address().into(), amount_usdc)
        .await
        .unwrap();

    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );

    // Add usdc value to order
    let call_params = CallParameters::default()
        .with_asset_id(usdc.asset_id)
        .with_amount(amount_usdc);

    orderbook
        .instance
        .methods()
        .open_order(btc.asset_id, base_size_p1.clone(), base_price)
        .call_params(call_params)
        .unwrap()
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();

    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );
    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount_btc
    );

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_n1, order.base_size);

    // Mint USDC
    usdc.mint(admin.address().into(), amount_usdc)
        .await
        .unwrap();

    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc * 2
    );

    // Add more usdc value to order
    let base_size_p2: I64 = I64 {
        value: 2 * usd * 10u64.pow(btc.decimals.try_into().unwrap()) / price,
        negative: false,
    };

    let call_params = CallParameters::default()
        .with_asset_id(usdc.asset_id)
        .with_amount(amount_usdc * 2);

    orderbook
        .instance
        .methods()
        .open_order(btc.asset_id, base_size_p2.clone(), base_price)
        .call_params(call_params)
        .unwrap()
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();

    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );
    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount_btc * 2
    );

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_p1, order.base_size);

    // Cancel by not order owner
    orderbook
        .with_account(user)
        .instance
        .methods()
        .cancel_order(*order_id)
        .append_variable_outputs(1)
        .call()
        .await
        .expect_err("Order cancelled by another user");

    // Cancel order
    orderbook
        .instance
        .methods()
        .cancel_order(*order_id)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

    assert!(response.value.is_none());

    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        2 * amount_btc
    );
    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        2 * amount_usdc
    );
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
    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals).await;

    // Create Market
    orderbook
        .instance
        .methods()
        .create_market(btc.asset_id, btc.decimals as u32)
        .call()
        .await
        .unwrap();

    let response = orderbook
        .instance
        .methods()
        .market_exists(btc.asset_id)
        .call()
        .await
        .unwrap();
    assert_eq!(true, response.value);

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    // Mint BTC & USDC

    let price_decimals = 9;
    let usd = 250000;
    let btcv = 5;
    let price = 50000;
    let amount_usdc = usd * 10u64.pow(usdc.decimals.try_into().unwrap());
    let amount_btc = btcv * 10u64.pow(btc.decimals.try_into().unwrap());
    let base_price = price * 10u64.pow(price_decimals);
    let base_size_p1: I64 = I64 {
        value: amount_btc,
        negative: false,
    };
    let base_size_n1: I64 = I64 {
        value: amount_btc,
        negative: true,
    };

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

    let call_params = CallParameters::default()
        .with_asset_id(usdc.asset_id)
        .with_amount(amount_usdc);

    orderbook
        .instance
        .methods()
        .open_order(btc.asset_id, base_size_p1.clone(), base_price)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    assert_eq!(admin.get_asset_balance(&usdc.asset_id).await.unwrap(), 0);

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id = response.value.get(0).unwrap();
    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_p1, order.base_size);

    // Cancel order by submitting btc
    let call_params = CallParameters::default()
        .with_asset_id(btc.asset_id)
        .with_amount(amount_btc);

    orderbook
        .instance
        .methods()
        .open_order(btc.asset_id, base_size_n1.clone(), base_price)
        .append_variable_outputs(2)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

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
async fn match_orders_test() {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];
    let user1 = &wallets[1];
    let user2 = &wallets[2];

    let token_contract = deploy_token_contract(&admin).await;
    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals).await;

    // Create Market
    orderbook
        .instance
        .methods()
        .create_market(btc.asset_id, btc.decimals as u32)
        .call()
        .await
        .unwrap();

    // Mint BTC & USDC

    let price_decimals = 9;
    let usd = 250000;
    let btcv = 5;
    let price = 50000;
    let amount_usdc = usd * 10u64.pow(usdc.decimals.try_into().unwrap());
    let amount_btc = btcv * 10u64.pow(btc.decimals.try_into().unwrap());
    let base_price = price * 10u64.pow(price_decimals);
    let base_size_p1: I64 = I64 {
        value: amount_btc,
        negative: false,
    };
    let base_size_n1: I64 = I64 {
        value: amount_btc,
        negative: true,
    };

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

    let call_params = CallParameters::default()
        .with_asset_id(usdc.asset_id)
        .with_amount(amount_usdc);

    orderbook
        .with_account(user1)
        .instance
        .methods()
        .open_order(btc.asset_id, base_size_p1.clone(), base_price)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    assert_eq!(user1.get_asset_balance(&usdc.asset_id).await.unwrap(), 0);

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(user1.address())
        .call()
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id_1 = response.value.get(0).unwrap();
    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id_1)
        .call()
        .await
        .unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_p1, order.base_size);

    // Open BTC order

    let call_params = CallParameters::default()
        .with_asset_id(btc.asset_id)
        .with_amount(amount_btc);

    orderbook
        .with_account(user2)
        .instance
        .methods()
        .open_order(btc.asset_id, base_size_n1.clone(), base_price)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    assert_eq!(user2.get_asset_balance(&btc.asset_id).await.unwrap(), 0);

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(user2.address())
        .call()
        .await
        .unwrap();

    assert_eq!(1, response.value.len());

    let order_id_2 = response.value.get(0).unwrap();
    let response = orderbook
        .instance
        .methods()
        .order_by_id(*order_id_2)
        .call()
        .await
        .unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);
    assert_eq!(base_size_n1, order.base_size);

    // Match orders
    orderbook
        .instance
        .methods()
        .match_orders(*order_id_2, *order_id_1)
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(user1.address())
        .call()
        .await
        .unwrap();

    assert_eq!(0, response.value.len());

    let response = orderbook
        .instance
        .methods()
        .orders_by_trader(user2.address())
        .call()
        .await
        .unwrap();

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
