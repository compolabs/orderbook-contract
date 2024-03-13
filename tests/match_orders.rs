use fuels::prelude::*;
use orderbook::orderbook_utils::Orderbook;
use src20_sdk::token_utils::{deploy_token_contract, Asset};

const PRICE_DECIMALS: u64 = 9;

//noinspection RsVariableNaming
#[tokio::test]
async fn match1() {
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
    let usdc_mint_amount = usdc.parse_units(92_000_f64) as u64;
    let btc_mint_amount = usdc.parse_units(1_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();
    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, buy_price as u64)
        .await
        .unwrap()
        .value;

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
    assert_eq!(
        alice.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (1_f64 * 1e8) as u64
    );

    orderbook
        .with_account(alice)
        .cancel_order(&alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice осталось 47,000 USDC после покупки 1 BTC по цене 45,000 USDC
    assert_eq!(
        alice.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (47_000_f64 * 1e6) as u64
    );

    // Проверяем, что у Bob есть 0 BTC после продажи
    assert_eq!(bob.get_asset_balance(&btc.asset_id).await.unwrap(), 0);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    assert_eq!(
        bob.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (45_000_f64 * 1e6) as u64
    );
}

#[tokio::test]
async fn match2() {
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

    let usdc_mint_amount = usdc.parse_units(46_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(2_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, buy_price as u64)
        .await
        .unwrap()
        .value;

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
    // assert_eq!(
    //     alice.get_asset_balance(&btc.asset_id).await.unwrap(),
    //     (1_f64 * 1e8) as u64
    // );

    // Проверяем, что у Alice осталось 1000 USDC сдачи после покупки 1 BTC по цене 45,000 USDC
    assert_eq!(
        alice.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (1_000_f64 * 1e6) as u64
    );

    // Проверяем, что у Bob остался 1 BTC после продажи 1 BTC из 2
    orderbook
        .with_account(bob)
        .cancel_order(&bob_order_id)
        .await
        .unwrap();

    assert_eq!(
        bob.get_asset_balance(&btc.asset_id).await.unwrap(),
        (1_f64 * 1e8) as u64
    );

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    assert_eq!(
        bob.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (45_000_f64 * 1e6) as u64
    );
}

#[tokio::test]
async fn match3() {
    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
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

    let buy_price = 46_000_f64 * 1e9;
    let sell_price = 45_000_f64 * 1e9;
    let size = 1_f64 * 1e8;

    let usdc_mint_amount = usdc.parse_units(46_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, size as i64, buy_price as u64)
        .await
        .unwrap()
        .value;

    let bob_order_id = orderbook
        .with_account(bob)
        .open_order(btc.asset_id, -1 * size as i64, sell_price as u64)
        .await
        .unwrap()
        .value;

    orderbook
        .match_orders(&bob_order_id, &alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    assert_eq!(
        alice.get_asset_balance(&btc.asset_id).await.unwrap(),
        (1_f64 * 1e8) as u64
    );

    // у Alice должно остаться 1,000 USDC после покупки 1 BTC
    assert_eq!(
        alice.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (1_000_f64 * 1e6) as u64
    );

    // Проверяем, что у Bob остался 0 BTC после продажи 1 BTC
    assert_eq!(bob.get_asset_balance(&btc.asset_id).await.unwrap(), 0);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    assert_eq!(
        bob.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (45_000_f64 * 1e6) as u64
    );
}

#[tokio::test]
async fn match4() {
    // ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
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

    let buy_price = 44_000_f64 * 1e9;
    let sell_price = 45_000_f64 * 1e9;
    let buy_size = 2_f64 * 1e8;
    let sell_size = 1_f64 * 1e8;

    let usdc_mint_amount = usdc.parse_units(88_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, buy_price as u64)
        .await
        .unwrap()
        .value;

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
}

#[tokio::test]
async fn match5() {
    // ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
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

    let buy_price = 44_000_f64 * 1e9;
    let sell_price = 45_000_f64 * 1e9;
    let buy_size = 1_f64 * 1e8;
    let sell_size = 2_f64 * 1e8;

    let usdc_mint_amount = usdc.parse_units(44_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(2_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, buy_price as u64)
        .await
        .unwrap()
        .value;

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
}

#[tokio::test]
async fn match6() {
    // ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
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

    let buy_price = 44_000_f64 * 1e9;
    let sell_price = 45_000_f64 * 1e9;
    let buy_size = 1_f64 * 1e8;
    let sell_size = 1_f64 * 1e8;

    let usdc_mint_amount = usdc.parse_units(44_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, buy_price as u64)
        .await
        .unwrap()
        .value;

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
}

#[tokio::test]
async fn match7() {
    // ✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
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

    let price = 45_000_f64 * 1e9;
    let buy_size = 2_f64 * 1e8;
    let sell_size = 1_f64 * 1e8;

    let usdc_mint_amount = usdc.parse_units(90_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, price as u64)
        .await
        .unwrap()
        .value;

    let bob_order_id = orderbook
        .with_account(bob)
        .open_order(btc.asset_id, -1 * sell_size as i64, price as u64)
        .await
        .unwrap()
        .value;

    orderbook
        .match_orders(&bob_order_id, &alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    assert_eq!(
        alice.get_asset_balance(&btc.asset_id).await.unwrap(),
        (1_f64 * 1e8) as u64
    );

    // у Alice должно остаться 45,000 USDC после покупки 1 BTC
    assert_eq!(
        alice.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (45_000_f64 * 1e6) as u64
    );

    // Проверяем, что у Bob остался 0 BTC после продажи 1 BTC
    assert_eq!(bob.get_asset_balance(&btc.asset_id).await.unwrap(), 0);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    assert_eq!(
        bob.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (45_000_f64 * 1e6) as u64
    );
}

#[tokio::test]
async fn match8() {
    // ✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
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

    let price = 45_000_f64 * 1e9;
    let buy_size = 1_f64 * 1e8;
    let sell_size = 2_f64 * 1e8;

    let usdc_mint_amount = usdc.parse_units(45_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(2_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, buy_size as i64, price as u64)
        .await
        .unwrap()
        .value;

    let bob_order_id = orderbook
        .with_account(bob)
        .open_order(btc.asset_id, -1 * sell_size as i64, price as u64)
        .await
        .unwrap()
        .value;

    orderbook
        .match_orders(&bob_order_id, &alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    assert_eq!(
        alice.get_asset_balance(&btc.asset_id).await.unwrap(),
        (1_f64 * 1e8) as u64
    );

    // у Alice должно остаться 0,000 USDC после покупки 1 BTC
    assert_eq!(alice.get_asset_balance(&usdc.asset_id).await.unwrap(), 0);

    orderbook
        .with_account(bob)
        .cancel_order(&bob_order_id)
        .await
        .unwrap();

    // Проверяем, что у Bob остался 1 BTC после продажи 1 BTC из 2
    assert_eq!(
        bob.get_asset_balance(&btc.asset_id).await.unwrap(),
        (1_f64 * 1e8) as u64
    );

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    assert_eq!(
        bob.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (45_000_f64 * 1e6) as u64
    );
}

#[tokio::test]
async fn match9() {
    //✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
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

    let price = 45_000_f64 * 1e9;
    let size = 1_f64 * 1e8;

    let usdc_mint_amount = usdc.parse_units(45_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;

    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();

    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();

    let alice_order_id = orderbook
        .with_account(alice)
        .open_order(btc.asset_id, size as i64, price as u64)
        .await
        .unwrap()
        .value;

    let bob_order_id = orderbook
        .with_account(bob)
        .open_order(btc.asset_id, -1 * size as i64, price as u64)
        .await
        .unwrap()
        .value;

    orderbook
        .match_orders(&bob_order_id, &alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    assert_eq!(
        alice.get_asset_balance(&btc.asset_id).await.unwrap(),
        (1_f64 * 1e8) as u64
    );

    // у Alice должно остаться 0,000 USDC после покупки 1 BTC
    assert_eq!(alice.get_asset_balance(&usdc.asset_id).await.unwrap(), 0);

    // Проверяем, что у Bob остался 0 BTC после продажи 1 BTC
    assert_eq!(bob.get_asset_balance(&btc.asset_id).await.unwrap(), 0);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    assert_eq!(
        bob.get_asset_balance(&usdc.asset_id).await.unwrap(),
        (45_000_f64 * 1e6) as u64
    );
}
