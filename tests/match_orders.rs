use orderbook::test_utils::*;

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
    let expected_balance = 102_222_222 as u64;
    let actual_balance = alice.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Alice осталось 1000 USDC сдачи после покупки 1 BTC по цене 45,000 USDC
    let expected_balance = 0 as u64;
    let actual_balance = alice.get_asset_balance(&usdc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    orderbook
        .with_account(&bob)
        .cancel_order(&bob_order_id)
        .await
        .unwrap();

    // Проверяем, что у Bob остался 1 BTC после продажи 1 BTC из 2
    let expected_balance = 97_777_778 as u64;
    let actual_balance = bob.get_asset_balance(&btc.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);

    // Проверяем, что у Bob есть 45,000 USDC после продажи своего BTC
    let expected_balance = 45_999_999_900 as u64;
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
    let expected_balance = (1_000_f64 * 1e6) as u64;
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
