use fuels::prelude::*;
use orderbook::orderbook_utils::{Orderbook, I64};
use src20_sdk::token_utils::{deploy_token_contract, Asset};

#[tokio::test]
async fn create_market_test() {
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

    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
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
}

#[tokio::test]
async fn open_cancel_base_token_order_test() {
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

    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
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

    // Mint BTC

    let price_decimals = 9;
    let amount = 5 * 10u64.pow(btc.decimals.try_into().unwrap());
    let base_size: I64 = I64 {
        value: amount,
        negative: true,
    };
    let base_price = 10 * 10u64.pow(price_decimals);

    btc.mint(admin.address().into(), amount).await.unwrap();

    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount
    );

    // Open order

    let call_params = CallParameters::default()
        .with_asset_id(btc.asset_id)
        .with_amount(amount);

    orderbook
        .instance
        .methods()
        .open_order(btc.asset_id, base_size.clone(), base_price)
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
    assert_eq!(base_size, order.base_size);

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
        amount
    );
}

#[tokio::test]
async fn open_cancel_quote_token_order_test() {
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

    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
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

    // Mint USDC

    let price_decimals = 9;
    let usd = 5;
    let price = 50000;
    let amount = usd * 10u64.pow(usdc.decimals.try_into().unwrap());
    let base_price = price * 10u64.pow(price_decimals);
    let base_size: I64 = I64 {
        value: usd * 10u64.pow(btc.decimals.try_into().unwrap()) / price,
        negative: false,
    };

    usdc.mint(admin.address().into(), amount).await.unwrap();

    assert_eq!(
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount
    );

    // Open order

    let call_params = CallParameters::default()
        .with_asset_id(usdc.asset_id)
        .with_amount(amount);

    orderbook
        .instance
        .methods()
        .open_order(btc.asset_id, base_size.clone(), base_price)
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
    assert_eq!(base_size, order.base_size);

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
        admin.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount
    );
}
