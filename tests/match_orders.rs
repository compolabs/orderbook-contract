use fuels::prelude::*;
use orderbook::orderbook_utils::Orderbook;
use src20_sdk::token_utils::{deploy_token_contract, Asset};

const PRICE_DECIMALS: u64 = 9;

//noinspection RsVariableNaming
#[tokio::test]
async fn open_base_token_order_cancel_test() {
    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];
    let alice = &wallets[1];
    let bob = &wallets[2];

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

    let buy_price = 46_000_f64 * 1e9; // Higher buy price
    let sell_price = 45_000_f64 * 1e9; // Lower sell price
    let buy_size = 2_f64 * 1e8; // Larger buy size
    let sell_size = 1_f64 * 1e8; // Smaller sell size

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units((46_000 * 2) as f64) as u64;
    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, buy_price as u64)
        .await
        .unwrap()
        .value;

    btc.mint(bob.address().into(), sell_size as u64)
        .await
        .unwrap();

    let bob_order_id = orderbook
        .with_account(bob)
        .open_order(btc.asset_id, -1 * sell_size as i64, sell_price as u64)
        .await
        .unwrap()
        .value;

    orderbook
        .match_orders(&bob_order_id, &alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    assert!(alice.get_asset_balance(&btc.asset_id).await.unwrap() == (1_f64 * 1e8) as u64);

    // Проверяем, что у Alice осталось 47,000 USDC после покупки 1 BTC по цене 45,000 USDC
    orderbook
        .with_account(alice)
        .cancel_order(&alice_order_id)
        .await
        .unwrap();
    assert!(alice.get_asset_balance(&usdc.asset_id).await.unwrap() == (47000_f64 * 1e6) as u64);
    
    // Проверяем, что у Bob есть 0 BTC после продажи
    assert!(bob.get_asset_balance(&btc.asset_id).await.unwrap() == 0);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    assert!(bob.get_asset_balance(&usdc.asset_id).await.unwrap() == (45000_f64 * 1e6) as u64);

}

#[tokio::test]
async fn open_base_token_order_cancel_test2() {
    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];
    let alice = &wallets[1];
    let bob = &wallets[2];

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

    let buy_price = 46_000_f64 * 1e9; // Higher buy price
    let sell_price = 45_000_f64 * 1e9; // Lower sell price
    let buy_size = 1_f64 * 1e8; // Smaller buy size
    let sell_size = 2_f64 * 1e8; // Lager sell size

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units((46_000) as f64) as u64;
    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, buy_price as u64)
        .await
        .unwrap()
        .value;

    btc.mint(bob.address().into(), sell_size as u64)
        .await
        .unwrap();

    let bob_order_id = orderbook
        .with_account(bob)
        .open_order(btc.asset_id, -1 * sell_size as i64, sell_price as u64)
        .await
        .unwrap()
        .value;

    orderbook
        .match_orders(&bob_order_id, &alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    assert!(alice.get_asset_balance(&btc.asset_id).await.unwrap() == (1_f64 * 1e8) as u64);

    // Проверяем, что у Alice осталось 47,000 USDC после покупки 1 BTC по цене 45,000 USDC
    orderbook
        .with_account(alice)
        .cancel_order(&alice_order_id)
        .await
        .unwrap();
    assert!(alice.get_asset_balance(&usdc.asset_id).await.unwrap() == (46000_f64 * 1e6) as u64);
    
    // Проверяем, что у Bob есть 0 BTC после продажи
    assert!(bob.get_asset_balance(&btc.asset_id).await.unwrap() == 0);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    assert!(bob.get_asset_balance(&usdc.asset_id).await.unwrap() == (45000_f64 * 1e6) as u64);

}