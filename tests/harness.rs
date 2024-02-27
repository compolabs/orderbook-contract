use std::str::FromStr;

use fuels::{prelude::*, types::ContractId};
use orderbook::{constants::TOKEN_CONTRACT_ID, orderbook_utils::Orderbook, orderbook_utils::I64};
use src20_sdk::token_utils::{Asset, TokenContract};

#[tokio::test]
async fn create_market_test() {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];

    let orderbook = Orderbook::deploy(&admin).await;

    let id = ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into();
    let token_contract = TokenContract::new(&id, admin.clone());

    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");

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
async fn open_order_test() {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];

    let orderbook = Orderbook::deploy(&admin).await;

    let id = ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into();
    let token_contract = TokenContract::new(&id, admin.clone());

    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");

    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .unwrap();

    let amount = 1000_000_000_u64; //1000 USDC
    
    let price_decimals = 9;
    let base_size: I64 = I64 { value: 5, negative: false};
    let order_price = 10 * 10u64.pow(price_decimals);

    let response = orderbook.instance
        .methods()
        .orders_by_trader(admin.address())
        .call()
        .await
        .unwrap();

    assert_eq!(0, response.value.len());  

    orderbook.instance
        .methods()
        .open_order(btc.asset_id, base_size.clone(), order_price)
        .call()
        .await
        .unwrap();

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
    assert_eq!(order_price, order.base_price);  
    assert_eq!(base_size, order.base_size);  

}
