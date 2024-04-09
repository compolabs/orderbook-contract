use crate::utils::check_balance::check_balance;
use crate::utils::match_orders_case_utils::match_orders_setup;

mod success {

    use super::*;

    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
    // constants
    // balances check
    // alice deposit
    // deposit check
    // create a buy order
    // deposited balance check and order check
    // same stuff for a sell order
    // match orders
    // close order in nessesary
    // deposited balances check
    #[tokio::test]
    async fn greater_buy_price_and_greater_buy_amount() {
        let buy_price = 46_000_f64;
        let sell_price = 45_000_f64;
        let buy_size = 2_f64;
        let sell_size = -1_f64;

        let alice_token_expected_balance = (1_f64 * 1e8) as u64;
        let alice_usdc_expected_balance = (47_000_f64 * 1e6) as u64;
        let bob_token_expected_balance = 0;
        let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

        let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

        check_balance(&context.alice, &context.token, alice_token_expected_balance).await;

        context
            .orderbook
            .with_account(&context.alice)
            .cancel_order(&context.alice_order_id)
            .await
            .unwrap();

        check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;
        check_balance(&context.bob, &context.token, bob_token_expected_balance).await;
        check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    }

    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
    #[tokio::test]
    async fn match2() {
        let buy_price = 46_000_f64;
        let sell_price = 45_000_f64;
        let buy_size = 1_f64;
        let sell_size = -2_f64;

        let alice_token_expected_balance = 102_222_222 as u64;
        let alice_usdc_expected_balance = 0;
        let bob_token_expected_balance = 97_777_778 as u64;
        let bob_usdc_expected_balance = 45_999_999_900 as u64;

        let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;
        check_balance(&context.alice, &context.token, alice_token_expected_balance).await;

        check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;

        context
            .orderbook
            .with_account(&context.bob)
            .cancel_order(&context.bob_order_id)
            .await
            .unwrap();

        check_balance(&context.bob, &context.token, bob_token_expected_balance).await;

        check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    }

    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
    #[tokio::test]
    async fn match3() {
        let buy_price = 46_000_f64;
        let sell_price = 45_000_f64;
        let buy_size = 1_f64;
        let sell_size = -1_f64;

        let alice_token_expected_balance = (1_f64 * 1e8) as u64;
        let alice_usdc_expected_balance = (1_000_f64 * 1e6) as u64;
        let bob_token_expected_balance = 0;
        let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

        let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

        context
            .orderbook
            .with_account(&context.alice)
            .cancel_order(&context.alice_order_id)
            .await
            .unwrap();

        check_balance(&context.alice, &context.token, alice_token_expected_balance).await;
        check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;
        check_balance(&context.bob, &context.token, bob_token_expected_balance).await;
        check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    }

    // ✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
    #[tokio::test]
    async fn match7() {
        let buy_price = 45_000_f64;
        let sell_price = 45_000_f64;
        let buy_size = 2_f64;
        let sell_size = -1_f64;

        let alice_token_expected_balance = (1_f64 * 1e8) as u64;
        let alice_usdc_expected_balance = (45_000_f64 * 1e6) as u64;
        let bob_token_expected_balance = 0;
        let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

        let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

        context
            .orderbook
            .with_account(&context.alice)
            .cancel_order(&context.alice_order_id)
            .await
            .unwrap();

        check_balance(&context.alice, &context.token, alice_token_expected_balance).await;

        check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;

        check_balance(&context.bob, &context.token, bob_token_expected_balance).await;

        check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    }

    // ✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
    #[tokio::test]
    async fn match8() {
        let buy_price = 45_000_f64;
        let sell_price = 45_000_f64;
        let buy_size = 1_f64;
        let sell_size = -2_f64;

        let alice_token_expected_balance = (1_f64 * 1e8) as u64;
        let alice_usdc_expected_balance = 0;
        let bob_token_expected_balance = (1_f64 * 1e8) as u64;
        let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

        let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

        check_balance(&context.alice, &context.token, alice_token_expected_balance).await;
        check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;

        context
            .orderbook
            .with_account(&context.bob)
            .cancel_order(&context.bob_order_id)
            .await
            .unwrap();

        check_balance(&context.bob, &context.token, bob_token_expected_balance).await;
        check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    }

    //✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
    #[tokio::test]
    async fn match9() {
        let buy_price = 45_000_f64;
        let sell_price = 45_000_f64;
        let buy_size = 1_f64;
        let sell_size = -1_f64;

        let alice_token_expected_balance = (1_f64 * 1e8) as u64;
        let alice_usdc_expected_balance = 0;
        let bob_token_expected_balance = 0;
        let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

        let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

        check_balance(&context.alice, &context.token, alice_token_expected_balance).await;
        check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;
        check_balance(&context.bob, &context.token, bob_token_expected_balance).await;
        check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    }
}

//todo
// mod revert {
//     use super::*;

//     // ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
//     #[tokio::test]
//     #[should_panic(expected = "OrdersCantBeMatched")]
//     async fn match4() {
//         let context = setup().await;
//     }

//     // ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
//     #[tokio::test]
//     #[should_panic(expected = "OrdersCantBeMatched")]
//     async fn match5() {
//         let context = setup().await;
//     }

//     // ❌ buyOrder.orderPrice < sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
//     #[tokio::test]
//     #[should_panic(expected = "OrdersCantBeMatched")]
//     async fn match6() {
//         let context = setup().await;
//     }
// }
