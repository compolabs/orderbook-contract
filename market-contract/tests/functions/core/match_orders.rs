use crate::utils::{
    interface::core::{deposit, open_order},
    setup::{setup, Defaults, OrderType},
};
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

mod success {

    use crate::utils::interface::info::account;

    use super::*;

    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
    #[tokio::test]
    async fn greater_buy_price_and_greater_buy_amount() {
        let buy_price = 46_000_f64;
        let sell_price = 45_000_f64;
        let buy_size = 2_f64;
        let sell_size = 1_f64;

        let alice_token_expected_balance = (1_f64 * 1e8) as u64;
        let alice_usdc_expected_balance = (47_000_f64 * 1e6) as u64;
        let bob_token_expected_balance = 0;
        let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

        let defaults = Defaults::default();
        let (contract, alice, bob, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let bob_instance = contract.clone().with_account(bob.wallet.clone());
        let alice_instance = contract.clone().with_account(alice.wallet.clone());

        //deposit
        let bob_deposit_base_amount = assets.base.parse_units(sell_size) as u64;
        let _ = deposit(&bob_instance, bob_deposit_base_amount, assets.base.id).await;
        let bob_account = account(&contract, bob.identity()).await.value.unwrap();
        assert_eq!(bob_account.liquid.base, bob_deposit_base_amount);

        //create order bob
        let bob_order_id = open_order(
            &bob_instance,
            assets.base.parse_units(sell_size) as u64,
            assets.base.id,
            OrderType::Sell,
            (sell_price * 1e9) as u64,
        )
        .await
        .value;

        //deposit
        let alice_deposit_quote_amount = assets.quote.parse_units(buy_price * buy_size) as u64;
        let _ = deposit(&alice_instance, alice_deposit_quote_amount, assets.quote.id).await;
        let alice_account = account(&contract, alice.identity()).await.value.unwrap();
        assert_eq!(alice_account.liquid.quote, alice_deposit_quote_amount);

        // //create order alice
        // //fixme InsufficientBalance((0, 92000000000000))
        let alice_order_id = open_order(
            &alice_instance,
            assets.base.parse_units(buy_size) as u64,
            assets.base.id,
            OrderType::Buy,
            (buy_price * 1e9) as u64,
        )
        .await
        .value;
    }

    // // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
    // #[tokio::test]
    // async fn greater_buy_price_and_greater_sell_amount() {
    //     let buy_price = 46_000_f64;
    //     let sell_price = 45_000_f64;
    //     let buy_size = 1_f64;
    //     let sell_size = -2_f64;

    //     let alice_token_expected_balance = 102_222_222 as u64;
    //     let alice_usdc_expected_balance = 0;
    //     let bob_token_expected_balance = 97_777_778 as u64;
    //     let bob_usdc_expected_balance = 45_999_999_900 as u64;

    //     let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;
    //     check_balance(&context.alice, &context.token, alice_token_expected_balance).await;

    //     check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;

    //     context
    //         .orderbook
    //         .with_account(&context.bob)
    //         .cancel_order(&context.bob_order_id)
    //         .await
    //         .unwrap();

    //     check_balance(&context.bob, &context.token, bob_token_expected_balance).await;

    //     check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    // }

    // // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
    // #[tokio::test]
    // async fn greater_buy_price_and_equal_amounts() {
    //     let buy_price = 46_000_f64;
    //     let sell_price = 45_000_f64;
    //     let buy_size = 1_f64;
    //     let sell_size = -1_f64;

    //     let alice_token_expected_balance = (1_f64 * 1e8) as u64;
    //     let alice_usdc_expected_balance = (1_000_f64 * 1e6) as u64;
    //     let bob_token_expected_balance = 0;
    //     let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

    //     let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

    //     context
    //         .orderbook
    //         .with_account(&context.alice)
    //         .cancel_order(&context.alice_order_id)
    //         .await
    //         .unwrap();

    //     check_balance(&context.alice, &context.token, alice_token_expected_balance).await;
    //     check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;
    //     check_balance(&context.bob, &context.token, bob_token_expected_balance).await;
    //     check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    // }

    // // ✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
    // #[tokio::test]
    // async fn equal_prices_and_greater_buy_amount() {
    //     let buy_price = 45_000_f64;
    //     let sell_price = 45_000_f64;
    //     let buy_size = 2_f64;
    //     let sell_size = -1_f64;

    //     let alice_token_expected_balance = (1_f64 * 1e8) as u64;
    //     let alice_usdc_expected_balance = (45_000_f64 * 1e6) as u64;
    //     let bob_token_expected_balance = 0;
    //     let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

    //     let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

    //     context
    //         .orderbook
    //         .with_account(&context.alice)
    //         .cancel_order(&context.alice_order_id)
    //         .await
    //         .unwrap();

    //     check_balance(&context.alice, &context.token, alice_token_expected_balance).await;

    //     check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;

    //     check_balance(&context.bob, &context.token, bob_token_expected_balance).await;

    //     check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    // }

    // // ✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize < sellOrder.baseSize
    // #[tokio::test]
    // async fn equal_prices_and_greater_sell_amount() {
    //     let buy_price = 45_000_f64;
    //     let sell_price = 45_000_f64;
    //     let buy_size = 1_f64;
    //     let sell_size = -2_f64;

    //     let alice_token_expected_balance = (1_f64 * 1e8) as u64;
    //     let alice_usdc_expected_balance = 0;
    //     let bob_token_expected_balance = (1_f64 * 1e8) as u64;
    //     let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

    //     let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

    //     check_balance(&context.alice, &context.token, alice_token_expected_balance).await;
    //     check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;

    //     context
    //         .orderbook
    //         .with_account(&context.bob)
    //         .cancel_order(&context.bob_order_id)
    //         .await
    //         .unwrap();

    //     check_balance(&context.bob, &context.token, bob_token_expected_balance).await;
    //     check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    // }

    // //✅ buyOrder.orderPrice = sellOrder.orderPrice & buyOrder.baseSize = sellOrder.baseSize
    // #[tokio::test]
    // async fn equal_prices_and_equal_amounts() {
    //     let buy_price = 45_000_f64;
    //     let sell_price = 45_000_f64;
    //     let buy_size = 1_f64;
    //     let sell_size = -1_f64;

    //     let alice_token_expected_balance = (1_f64 * 1e8) as u64;
    //     let alice_usdc_expected_balance = 0;
    //     let bob_token_expected_balance = 0;
    //     let bob_usdc_expected_balance = (45_000_f64 * 1e6) as u64;

    //     let context = match_orders_setup(buy_price, sell_price, buy_size, sell_size).await;

    //     check_balance(&context.alice, &context.token, alice_token_expected_balance).await;
    //     check_balance(&context.alice, &context.usdc, alice_usdc_expected_balance).await;
    //     check_balance(&context.bob, &context.token, bob_token_expected_balance).await;
    //     check_balance(&context.bob, &context.usdc, bob_usdc_expected_balance).await;
    // }
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
