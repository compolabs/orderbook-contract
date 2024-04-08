use crate::utils::{
    interface::core::{deposit, open_order},
    setup::{setup, Defaults, OrderType},
};

mod success {

    use super::*;

    #[tokio::test]
    async fn match1() {
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
        //mint bob
        let base_mint_amount = assets.base.parse_units(sell_size) as u64;
        assets
            .base
            .mint(bob.address().into(), base_mint_amount)
            .await
            .unwrap();

        //deposit
        let _ = deposit(
            &contract.clone().with_account(bob.wallet.clone()),
            base_mint_amount,
            assets.base.asset_id,
        )
        .await;

        //create order bob
        let bob_order_id = open_order(
            &contract.clone().with_account(bob.wallet.clone()),
            assets.base.parse_units(sell_size) as u64,
            assets.base.asset_id,
            OrderType::Sell,
            (sell_price * 1e9) as u64,
        )
        .await
        .value;

        //mint alice
        let quote_mint_amount = assets.quote.parse_units(buy_price * buy_size) as u64;
        assets
            .quote
            .mint(alice.address().into(), quote_mint_amount)
            .await
            .unwrap();

        //deposit
        let _ = deposit(
            &contract.clone().with_account(alice.wallet.clone()),
            quote_mint_amount,
            assets.quote.asset_id,
        )
        .await;

        //create order alice
        //fixme InsufficientBalance((0, 92000000000000))
        let alice_order_id = open_order(
            &contract.clone().with_account(alice.wallet.clone()),
            assets.base.parse_units(buy_size) as u64,
            assets.base.asset_id,
            OrderType::Buy,
            (buy_price * 1e9) as u64,
        )
        .await
        .value;

        //todo match

        // todo cancle order
        // todo check balance
    }
}

mod revert {}
