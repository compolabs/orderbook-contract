use fuels::prelude::*;
use src20_sdk::token_utils::{Asset, deploy_token_contract};
use orderbook::orderbook_utils::{Orderbook, I64};

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

    let orderbook = Orderbook::deploy(&admin).await;

    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .unwrap();

    let response = orderbook.instance
        .methods()
        .market_exists(btc.asset_id)
        .call()
        .await
        .unwrap();
    assert_eq!(true, response.value);

}

#[tokio::test]
async fn open_cancel_negative_order_test() {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];

    let orderbook = Orderbook::deploy(&admin).await;

    let token_contract = deploy_token_contract(&admin).await;
    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");

    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .unwrap();

    let response = orderbook.instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(0, response.value.len());  

    // Mint BTC
    
    let price_decimals = 9;
    let base_size: I64 = I64 { value: 5, negative: true};
    let base_price = 10 * 10u64.pow(price_decimals);
    let amount = base_size.value * 10u64.pow(btc.decimals.try_into().unwrap()); 

    btc.mint(admin.address().into(), amount)
        .await
        .unwrap();

    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount
    );

    // Open order

    let call_params = CallParameters::default()
            .with_asset_id(btc.asset_id)
            .with_amount(amount);

    orderbook.instance
        .methods()
        .open_order(btc.asset_id, base_size.clone(), base_price)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    assert_eq!(
        admin.get_asset_balance(&btc.asset_id).await.unwrap(),
        0
    );    
    
    let response = orderbook.instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(1, response.value.len());  

    let order_id = response.value.get(0).unwrap();
    let response = orderbook.instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

    let order = response.value.unwrap();
    assert_eq!(base_price, order.base_price);  
    assert_eq!(base_size, order.base_size);  

    // Cancel order

    orderbook.instance
        .methods()
        .cancel_order(*order_id)
        .call()
        .await
        .unwrap();

    let response = orderbook.instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(0, response.value.len());  

    let response = orderbook.instance
        .methods()
        .order_by_id(*order_id)
        .call()
        .await
        .unwrap();

    assert!(response.value.is_none());
}

