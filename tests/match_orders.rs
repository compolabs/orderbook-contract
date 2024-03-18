use fuels::types::Bits256;
use fuels::{accounts::wallet::Wallet, prelude::*};
use orderbook::orderbook_utils::Orderbook;
use src20_sdk::token_utils::{deploy_token_contract, Asset};
use std::result::Result;
const PRICE_DECIMALS: u64 = 9;
const TOLERANCE: f64 = 0.0005;

fn tolerance_eq(expected: u64, actual: u64) -> bool {
    let difference = (expected as f64 - actual as f64).abs();
    let relative_difference = difference / expected as f64;

    if (expected == actual)
        || (expected == 0 && actual == 0)
        || ((expected > actual) && (relative_difference < TOLERANCE))
    {
        println!(
            "✅ Баланс в пределах допуска или точное соответствие. Expected: {}, Actual: {}",
            expected, actual
        );
        true
    } else {
        if expected < actual {
            println!(
                "❌ Баланс больше ожидаемого. Expected: {}, Actual: {}",
                expected, actual
            );
        } else {
            println!(
                "❌ Баланс за пределами допуска. Expected: {}, Actual: {}",
                expected, actual
            );
        }
        false
    }
}

async fn init() -> (WalletUnlocked, WalletUnlocked, Asset, Asset, Orderbook) {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .expect("Failed to initialize wallets");
    let admin = wallets[0].clone();
    let alice = wallets[1].clone();
    let bob = wallets[2].clone();

    let btc_token_contract = deploy_token_contract(&admin).await;
    let btc = Asset::new(
        admin.clone(),
        btc_token_contract.contract_id().into(),
        "BTC",
    );

    let usdc_token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(
        admin.clone(),
        usdc_token_contract.contract_id().into(),
        "USDC",
    );

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    // Create Market
    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .expect("Failed to create market");

    (alice, bob, btc, usdc, orderbook)
}

async fn mint_tokens(
    usdc: &Asset,
    btc: &Asset,
    alice: &Wallet,
    bob: &Wallet,
    usdc_mint_amount: u64,
    btc_mint_amount: u64,
) {
    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();
    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();
}

async fn open_orders_match(
    orderbook: &Orderbook,
    alice: &WalletUnlocked,
    bob: &WalletUnlocked,
    btc: &Asset,
    buy_size: f64,
    buy_price: f64,
    sell_size: f64,
    sell_price: f64,
) -> Result<(Bits256, Bits256), fuels::types::errors::Error> {
    let alice_order_id = orderbook
        .with_account(&alice)
        .open_order(
            btc.asset_id,
            (buy_size * 1e8) as i64,
            (buy_price * 1e9) as u64,
        )
        .await
        .unwrap()
        .value;

    let bob_order_id = orderbook
        .with_account(&bob)
        .open_order(
            btc.asset_id,
            (sell_size * 1e8) as i64,
            (sell_price * 1e9) as u64,
        )
        .await
        .unwrap()
        .value;

    let res = orderbook.match_orders(&bob_order_id, &alice_order_id).await;
    if res.is_ok() {
        Ok((alice_order_id, bob_order_id))
    } else {
        Err(res.err().unwrap())
    }
}

// ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
#[tokio::test]
async fn match1() {
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 46_000_f64; // Higher buy price
    let sell_price = 45_000_f64; // Lower sell price
    let buy_size = 2_f64; // Larger buy size
    let sell_size = -1_f64; // Smaller sell size

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(92_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let (alice_order_id, _bob_order_id) = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await
    .expect("Failed to open and match orders");

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    let expected_balance = (1_f64 * 1e8) as u64;
    let actual_balance = alice.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    orderbook
        .with_account(&alice)
        .cancel_order(&alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice осталось 47,000 USDC после покупки 1 BTC по цене 45,000 USDC
    let expected_balance = (47_000_f64 * 1e6) as u64;
    let actual_balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob есть 0 BTC после продажи
    let expected_balance = 0;
    let actual_balance = bob.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    let expected_balance = (45_000_f64 * 1e6) as u64;
    let actual_balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);
}

// ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
#[tokio::test]
async fn match2() {
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 46_000_f64; // Higher buy price
    let sell_price = 45_000_f64; // Lower sell price
    let buy_size = 1_f64; // Smaller buy size
    let sell_size = -2_f64; // Lager sell size

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(46_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(2_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let (_alice_order_id, bob_order_id) = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await
    .expect("Failed to open and match orders");

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    let expected_balance = (1_f64 * 1e8) as u64;
    let actual_balance = alice.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Alice осталось 1000 USDC сдачи после покупки 1 BTC по цене 45,000 USDC
    let expected_balance = (1_f64 * 1e6) as u64;
    let actual_balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    orderbook
        .with_account(&bob)
        .cancel_order(&bob_order_id)
        .await
        .unwrap();

    // Проверяем, что у Bob остался 1 BTC после продажи 1 BTC из 2
    let expected_balance = (1_f64 * 1e8) as u64;
    let actual_balance = bob.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    let expected_balance = (45_000_f64 * 1e6) as u64;
    let actual_balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);
}

// ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
#[tokio::test]
async fn match3() {
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 46_000_f64;
    let sell_price = 45_000_f64;
    let buy_size = 1_f64;
    let sell_size = -1_f64;

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(46_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let (alice_order_id, _bob_order_id) = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await
    .expect("Failed to open and match orders");
    orderbook
        .with_account(&alice)
        .cancel_order(&alice_order_id)
        .await
        .unwrap();
    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    let expected_balance = (1_f64 * 1e8) as u64;
    let actual_balance = alice.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // у Alice должно остаться 1000 USDC после покупки 1 BTC
    let expected_balance = (1000_f64 * 1e6) as u64;
    let actual_balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob остался 0 BTC после продажи 1 BTC
    let expected_balance = 0;
    let actual_balance = bob.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    let expected_balance = (45_000_f64 * 1e6) as u64;
    let actual_balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);
}

// ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
#[tokio::test]
async fn match4() {
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 44_000_f64;
    let sell_price = 45_000_f64;
    let buy_size = 2_f64;
    let sell_size = -1_f64;

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(88_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let res = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await;
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .contains("OrdersCantBeMatched"));
}

// ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
#[tokio::test]
async fn match5() {
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 44_000_f64;
    let sell_price = 45_000_f64;
    let buy_size = 1_f64;
    let sell_size = -2_f64;

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(44_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(2_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let res = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await;
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .contains("OrdersCantBeMatched"));
}

// ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
#[tokio::test]
async fn match6() {
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 44_000_f64;
    let sell_price = 45_000_f64;
    let buy_size = 1_f64;
    let sell_size = -1_f64;

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(44_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let res = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await;
    assert!(res.is_err());
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .contains("OrdersCantBeMatched"));
}

// ✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
#[tokio::test]
async fn match7() {
    //--------------- WALLETS ---------------
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 45_000_f64;
    let sell_price = 45_000_f64;
    let buy_size = 2_f64;
    let sell_size = -1_f64;

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(90_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let (alice_order_id, _bob_order_id) = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await
    .expect("Failed to open and match orders");

    orderbook
        .with_account(&alice)
        .cancel_order(&alice_order_id)
        .await
        .unwrap();

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    let expected_balance = (1_f64 * 1e8) as u64;
    let actual_balance = alice.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // у Alice должно остаться 45,000 USDC после покупки 1 BTC
    let expected_balance = (45_000_f64 * 1e6) as u64;
    let actual_balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob остался 0 BTC после продажи 1 BTC
    let expected_balance = 0;
    let actual_balance = bob.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    let expected_balance = (45_000_f64 * 1e6) as u64;
    let actual_balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);
}

// ✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
#[tokio::test]
async fn match8() {
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 45_000_f64;
    let sell_price = 45_000_f64;
    let buy_size = 1_f64;
    let sell_size = -2_f64;

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(45_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(2_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let (_alice_order_id, bob_order_id) = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await
    .expect("Failed to open and match orders");

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    let expected_balance = (1_f64 * 1e8) as u64;
    let actual_balance = alice.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // у Alice должно остаться 0,000 USDC после покупки 1 BTC
    let expected_balance = 0;
    let actual_balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    orderbook
        .with_account(&bob)
        .cancel_order(&bob_order_id)
        .await
        .unwrap();

    // Проверяем, что у Bob остался 1 BTC после продажи 1 BTC из 2
    let expected_balance = (1_f64 * 1e8) as u64;
    let actual_balance = bob.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    let expected_balance = (45_000_f64 * 1e6) as u64;
    let actual_balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);
}

//✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
#[tokio::test]
async fn match9() {
    let (alice, bob, btc, usdc, orderbook) = init().await;

    let buy_price = 45_000_f64;
    let sell_price = 45_000_f64;
    let buy_size = 1_f64;
    let sell_size = -1_f64;

    // Mint BTC & USDC
    let usdc_mint_amount = usdc.parse_units(45_000_f64) as u64;
    let btc_mint_amount = btc.parse_units(1_f64) as u64;
    mint_tokens(&usdc, &btc, &alice, &bob, usdc_mint_amount, btc_mint_amount).await;

    // Open and match orders
    let (_alice_order_id, _bob_order_id) = open_orders_match(
        &orderbook, &alice, &bob, &btc, buy_size, buy_price, sell_size, sell_price,
    )
    .await
    .expect("Failed to open and match orders");

    // Проверяем, что у Alice есть 1 BTC после совершения сделки
    let expected_balance = (1_f64 * 1e8) as u64;
    let actual_balance = alice.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // у Alice должно остаться 0,000 USDC после покупки 1 BTC
    let expected_balance = 0;
    let actual_balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob остался 0 BTC после продажи 1 BTC
    let expected_balance = 0;
    let actual_balance = bob.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    let expected_balance = (45_000_f64 * 1e6) as u64;
    let actual_balance = bob.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);
}
