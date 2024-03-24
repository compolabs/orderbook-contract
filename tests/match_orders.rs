use orderbook::test_utils::*;

struct TestContext {
    admin: WalletUnlocked,
    alice: WalletUnlocked,
    bob: WalletUnlocked,
    usdc: Asset,
    token: Asset,
    orderbook: Orderbook,
    alice_order_id: Bits256,
    bob_order_id: Bits256,
    alice_token_expected_balance: u64,
    alice_usdc_expected_balance: u64,
    bob_token_expected_balance: u64,
    bob_usdc_expected_balance: u64,
}

async fn check_balance(wallet: &WalletUnlocked, asset: &Asset, expected_balance: u64) {
    let actual_balance = wallet.get_asset_balance(&asset.asset_id).await.unwrap();
    tolerance_eq(expected_balance, actual_balance);
}

async fn setup() -> TestContext {
    let (admin, alice, bob) = init_wallets().await;
    let (usdc, token) = init_tokens(&admin, "BTC").await;
    let orderbook = init_orderbook(&admin, &usdc, &token).await;

    let usdc_mint_amount = usdc.parse_units(92_000_f64) as u64;
    let token_mint_amount = token.parse_units(1_f64) as u64;

    mint_tokens(
        &usdc,
        &token,
        &alice,
        &bob,
        usdc_mint_amount,
        token_mint_amount,
    )
    .await;

    let buy_price = 46_000_f64;
    let sell_price = 45_000_f64;
    let buy_size = 2_f64;
    let sell_size = -1_f64;

    let alice_token_expected_balance = (1_f64 * 1e8) as u64;
    let alice_usdc_expected_balance = (47_000_f64 * 1e6) as u64;
    let bob_token_expected_balance = 0;
    let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

    let (alice_order_id, bob_order_id) = open_orders_match(
        &orderbook, &alice, &bob, &token, buy_size, buy_price, sell_size, sell_price,
    )
    .await
    .expect("Failed to open and match orders");

    TestContext {
        admin,
        alice,
        bob,
        usdc,
        token,
        orderbook,
        alice_order_id,
        bob_order_id,
        alice_token_expected_balance,
        alice_usdc_expected_balance,
        bob_token_expected_balance,
        bob_usdc_expected_balance,
    }
}
mod success {
    use super::*;

    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
    #[tokio::test]
    async fn match1() {
        let context = setup().await;
        check_balance(
            &context.alice,
            &context.token,
            context.alice_token_expected_balance,
        )
        .await;

        context
            .orderbook
            .with_account(&context.alice)
            .cancel_order(&context.alice_order_id)
            .await
            .unwrap();

        check_balance(
            &context.alice,
            &context.usdc,
            context.alice_usdc_expected_balance,
        )
        .await;
        check_balance(
            &context.bob,
            &context.token,
            context.bob_token_expected_balance,
        )
        .await;
        check_balance(
            &context.bob,
            &context.usdc,
            context.bob_usdc_expected_balance,
        )
        .await;
    }

    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
    #[tokio::test]
    async fn match2() {
        let context = setup().await;
        check_balance(
            &context.alice,
            &context.token,
            context.alice_token_expected_balance,
        )
        .await;

        context
            .orderbook
            .with_account(&context.alice)
            .cancel_order(&context.alice_order_id)
            .await
            .unwrap();

        check_balance(
            &context.alice,
            &context.usdc,
            context.alice_usdc_expected_balance,
        )
        .await;
        check_balance(
            &context.bob,
            &context.token,
            context.bob_token_expected_balance,
        )
        .await;
        check_balance(
            &context.bob,
            &context.usdc,
            context.bob_usdc_expected_balance,
        )
        .await;
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
}

mod revert {
    use super::*;

    // ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
    #[tokio::test]
    #[should_panic(expected = "OrdersCantBeMatched")]

    async fn match4() {
        let context = setup().await;

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
    #[should_panic(expected = "OrdersCantBeMatched")]
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
    #[should_panic(expected = "OrdersCantBeMatched")]
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
}
