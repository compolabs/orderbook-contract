use crate::utils::{
    interface::core::{deposit, open_order},
    setup::{setup, Defaults, OrderType},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{account, order, order_id, user_orders},
        setup::{create_account, OpenOrderEvent},
    };

    #[tokio::test]
    async fn match1() {
        let buy_price = 46_000_f64;
        let sell_price = 45_000_f64;
        let buy_size = 2_f64;
        let sell_size = -1_f64;

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
        let base_mint_amount = assets.base.parse_units(sell_size).abs() as u64;
        assets
            .base
            .mint(bob.address().into(), base_mint_amount)
            .await
            .unwrap();

        //deposit 
        let _ = deposit(&contract.with_account(&bob), base_mint_amount, assets.base).await;

        //create order bob
        let bob_order_id = contract
            .with_account(&bob)
            .open_order(
                assets.base.parse_units(sell_size) as i64,
                assets.base,
                OrderType.Sell,
                (sell_price * 1e9) as u64,
            )
            .await
            .unwrap()
            .value;

        //mint alice
        let quote_mint_amount = assets.quote.parse_units(buy_price * buy_size) as u64;
        assets
            .base
            .mint(alice.address().into(), quote_mint_amount)
            .await
            .unwrap();
        
        //deposit 
        let _ = deposit(&contract.with_account(&alice), quote_mint_amount, assets.quote).await;

        //create order alice
        let alice_order_id = contract
            .with_account(&alice)
            .open_order(
                assets.base.parse_units(buy_size) as i64,
                assets.base,
                OrderType.Buy,
                (buy_price * 1e9) as u64,
            )
            .await
            .unwrap()
            .value;

        //todo match

        // todo cancle order
        // todo check balance
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_invalid_asset() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let order_amount = 10;
        let asset = assets.random.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        // Revert
        open_order(&contract, order_amount, asset, order_type, price).await;
    }
}
