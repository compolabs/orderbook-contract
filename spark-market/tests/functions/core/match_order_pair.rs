use crate::setup::{create_account, now_tai64, setup, Defaults};
use spark_market_sdk::{OrderType, ProtocolFee};

mod success_same_asset_type {

    use super::*;

    #[tokio::test]
    async fn match_same_base_asset_type_orders_same_price() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000 * 10_u64.pow(defaults.price_decimals);
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount;
        contract
            .with_account(&user0.wallet)
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .deposit(quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .open_order(base_amount, OrderType::Buy, price)
            .await?
            .value;

        let expected_account0 = create_account(0, 0, base_amount, 0);
        let expected_account1 = create_account(0, 0, 0, quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );

        let expected_account0 = create_account(0, quote_amount, 0, 0);
        let expected_account1 = create_account(base_amount, 0, 0, 0);

        contract.match_order_pair(id0, id1).await?;

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_same_base_asset_type_orders_same_price_same_user() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000 * 10_u64.pow(defaults.price_decimals);
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount;
        contract
            .with_account(&user0.wallet)
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user0.wallet)
            .deposit(quote_amount, assets.quote.id)
            .await?;

        let expected_account0 = create_account(base_amount, quote_amount, 0, 0);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Buy, price)
            .await?
            .value;

        let expected_account0 = create_account(0, 0, base_amount, quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract.match_order_pair(id0, id1).await?;

        let expected_account0 = create_account(base_amount, quote_amount, 0, 0);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_same_base_asset_type_orders_size_equal_price_different() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let sell_price = 70_000 * 10_u64.pow(defaults.price_decimals);
        let buy_price = 77_000 * 10_u64.pow(defaults.price_decimals); // 77,000$ price
        let base_amount = 100_000_u64; // 0.001 BTC
        let sell_quote_amount = sell_price / to_quote_scale * base_amount;
        let buy_quote_amount = buy_price / to_quote_scale * base_amount;
        contract
            .with_account(&user0.wallet)
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .deposit(buy_quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Sell, sell_price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .open_order(base_amount, OrderType::Buy, buy_price)
            .await?
            .value;

        let expected_account0 = create_account(0, 0, base_amount, 0);
        let expected_account1 = create_account(0, 0, 0, buy_quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );

        let expected_account0 = create_account(0, sell_quote_amount, 0, 0);
        let expected_account1 =
            create_account(base_amount, buy_quote_amount - sell_quote_amount, 0, 0);

        contract.match_order_pair(id0, id1).await?;

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_same_base_asset_type_orders_size_not_equal() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000 * 10_u64.pow(defaults.price_decimals);
        let sell_base_amount = 100_000_u64; // 0.001 BTC
        let buy_base_amount = 90_000_u64; // 0.0009 BTC
        let sell_quote_amount = price / to_quote_scale * sell_base_amount;
        let buy_quote_amount = price / to_quote_scale * buy_base_amount;
        contract
            .with_account(&user0.wallet)
            .deposit(sell_base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .deposit(sell_quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(sell_base_amount, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .open_order(buy_base_amount, OrderType::Buy, price)
            .await?
            .value;

        let expected_account0 = create_account(0, 0, sell_base_amount, 0);
        let expected_account1 =
            create_account(0, sell_quote_amount - buy_quote_amount, 0, buy_quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );

        let expected_account0 =
            create_account(0, buy_quote_amount, sell_base_amount - buy_base_amount, 0);
        let expected_account1 =
            create_account(buy_base_amount, sell_quote_amount - buy_quote_amount, 0, 0);

        contract.match_order_pair(id0, id1).await?;

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );

        let order = contract.order(id0).await?.value;
        assert!(order.is_some());
        let order = contract.order(id1).await?.value;
        assert!(order.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn match_same_base_asset_type_orders_same_price_with_matcher_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, matcher, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_000_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000 * 10_u64.pow(defaults.price_decimals);
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount + matcher_fee;
        contract
            .with_account(&user0.wallet)
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .deposit(quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .open_order(base_amount, OrderType::Buy, price)
            .await?
            .value;

        let expected_account0 = create_account(0, 0, base_amount, 0);
        let expected_account1 = create_account(0, 0, 0, quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );

        let expected_account0 = create_account(0, quote_amount - 2 * matcher_fee, 0, 0);
        let expected_account1 = create_account(base_amount, 0, 0, 0);

        let expected_account = create_account(0, matcher_fee * 2, 0, 0);

        contract
            .with_account(&matcher.wallet)
            .match_order_pair(id0, id1)
            .await?;

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );
        assert_eq!(
            contract.account(matcher.identity()).await?.value,
            expected_account
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_same_base_asset_type_orders_same_price_with_matcher_fee_same_user_matcher(
    ) -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_000_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000 * 10_u64.pow(defaults.price_decimals);
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount + matcher_fee;
        contract
            .with_account(&user0.wallet)
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user0.wallet)
            .deposit(quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Buy, price)
            .await?
            .value;

        let expected_account0 = create_account(0, 0, base_amount, quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        let expected_account0 = create_account(base_amount, quote_amount, 0, 0);

        contract
            .with_account(&user0.wallet)
            .match_order_pair(id0, id1)
            .await?;

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_same_base_asset_type_orders_same_price_with_protocol_fee() -> anyhow::Result<()>
    {
        let defaults = Defaults::default();
        let (contract, owner, user0, user1, matcher, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let tai64_epoch = now_tai64();
        // Define the new epoch duration (e.g., 1 day)
        let epoch_duration = 60 * 60 * 24 * 30;
        // Increase the epoch and duration
        let _ = contract.set_epoch(tai64_epoch, epoch_duration).await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000 * 10_u64.pow(defaults.price_decimals);
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount;
        let maker_protocol_fee = quote_amount * 10 / 10_000;
        let taker_protocol_fee = quote_amount * 15 / 10_000;
        let quote_amount = quote_amount + taker_protocol_fee;

        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 10,
                taker_fee: 15,
                volume_threshold: 0,
            },
            ProtocolFee {
                maker_fee: 8,
                taker_fee: 12,
                volume_threshold: quote_amount - taker_protocol_fee,
            },
        ];

        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;

        assert_eq!(
            contract
                .protocol_fee_user(user0.address().into())
                .await?
                .value,
            (protocol_fee[0].maker_fee, protocol_fee[0].taker_fee)
        );

        assert_eq!(
            contract
                .protocol_fee_user(user1.address().into())
                .await?
                .value,
            (protocol_fee[0].maker_fee, protocol_fee[0].taker_fee)
        );

        contract
            .with_account(&user0.wallet)
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .deposit(quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .open_order(base_amount, OrderType::Buy, price)
            .await?
            .value;

        let expected_account0 = create_account(0, 0, base_amount, 0);
        let expected_account1 = create_account(0, 0, 0, quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );

        let expected_account0 = create_account(
            0,
            quote_amount - maker_protocol_fee - taker_protocol_fee,
            0,
            0,
        );
        let expected_account1 = create_account(base_amount, 0, 0, 0);

        let expected_account = create_account(0, maker_protocol_fee + taker_protocol_fee, 0, 0);

        contract
            .with_account(&matcher.wallet)
            .match_order_pair(id0, id1)
            .await?;

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await?.value,
            expected_account1
        );
        assert_eq!(
            contract.account(owner.identity()).await?.value,
            expected_account
        );

        assert_eq!(
            contract
                .protocol_fee_user(user0.address().into())
                .await?
                .value,
            (protocol_fee[1].maker_fee, protocol_fee[1].taker_fee)
        );
        assert_eq!(
            contract
                .protocol_fee_user(user1.address().into())
                .await?
                .value,
            (protocol_fee[1].maker_fee, protocol_fee[1].taker_fee)
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_same_base_asset_type_orders_same_price_with_protocol_fee_same_user(
    ) -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, user0, _, matcher, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let protocol_fee = vec![ProtocolFee {
            maker_fee: 10,
            taker_fee: 15,
            volume_threshold: 0,
        }];
        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000 * 10_u64.pow(defaults.price_decimals);
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount;
        let maker_protocol_fee = quote_amount * 10 / 10_000;
        let taker_protocol_fee = quote_amount * 15 / 10_000;
        let quote_amount = quote_amount + taker_protocol_fee;
        contract
            .with_account(&user0.wallet)
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user0.wallet)
            .deposit(quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Buy, price)
            .await?
            .value;

        let expected_account0 = create_account(0, 0, base_amount, quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        let expected_account0 = create_account(
            base_amount,
            quote_amount - maker_protocol_fee - taker_protocol_fee,
            0,
            0,
        );

        let expected_account = create_account(0, maker_protocol_fee + taker_protocol_fee, 0, 0);

        contract
            .with_account(&matcher.wallet)
            .match_order_pair(id0, id1)
            .await?;

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );
        assert_eq!(
            contract.account(owner.identity()).await?.value,
            expected_account
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_same_base_asset_type_orders_same_price_with_protocol_fee_same_user_owner_matcher(
    ) -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 10,
                taker_fee: 15,
                volume_threshold: 0,
            },
            ProtocolFee {
                maker_fee: 8,
                taker_fee: 13,
                volume_threshold: 10000000000,
            },
            ProtocolFee {
                maker_fee: 6,
                taker_fee: 11,
                volume_threshold: 50000000000,
            },
            ProtocolFee {
                maker_fee: 4,
                taker_fee: 9,
                volume_threshold: 100000000000,
            },
            ProtocolFee {
                maker_fee: 2,
                taker_fee: 7,
                volume_threshold: 500000000000,
            },
            ProtocolFee {
                maker_fee: 1,
                taker_fee: 5,
                volume_threshold: 1000000000000,
            },
        ];
        let matcher_fee = 1_000;
        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70000000000000;
        let base_amount = 10_u64; // 0.0000001 BTC
        let deposit_quote_amount = 10_000;
        let quote_amount = price / to_quote_scale * base_amount;
        let taker_protocol_fee = quote_amount * 15 / 10_000;
        let quote_amount = quote_amount + taker_protocol_fee + matcher_fee;
        contract.deposit(base_amount, assets.base.id).await?;
        contract
            .deposit(deposit_quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .open_order(base_amount, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .open_order(base_amount, OrderType::Buy, price)
            .await?
            .value;

        let expected_account = create_account(
            0,
            deposit_quote_amount - quote_amount,
            base_amount,
            quote_amount,
        );

        assert_eq!(
            contract.account(owner.identity()).await?.value,
            expected_account
        );

        let expected_account = create_account(base_amount, deposit_quote_amount, 0, 0);

        contract.match_order_pair(id0, id1).await?;

        assert_eq!(
            contract.account(owner.identity()).await?.value,
            expected_account
        );

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CantMatch")]
    async fn match_same_asset_type_orders_buy_price_low() {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let sell_price = 70_000 * 10_u64.pow(defaults.price_decimals); // 70,000$ price
        let buy_price = 67_000 * 10_u64.pow(defaults.price_decimals); // 67,000$ price
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = buy_price / to_quote_scale * base_amount;
        contract
            .with_account(&user0.wallet)
            .deposit(base_amount, assets.base.id)
            .await
            .unwrap();
        contract
            .with_account(&user1.wallet)
            .deposit(quote_amount, assets.quote.id)
            .await
            .unwrap();

        let id0 = contract
            .with_account(&user0.wallet)
            .open_order(base_amount, OrderType::Sell, sell_price)
            .await
            .unwrap()
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .open_order(base_amount, OrderType::Buy, buy_price)
            .await
            .unwrap()
            .value;

        let expected_account0 = create_account(0, 0, base_amount, 0);
        let expected_account1 = create_account(0, 0, 0, quote_amount);

        assert_eq!(
            contract.account(user0.identity()).await.unwrap().value,
            expected_account0
        );
        assert_eq!(
            contract.account(user1.identity()).await.unwrap().value,
            expected_account1
        );

        contract.match_order_pair(id0, id1).await.unwrap();
    }
}
