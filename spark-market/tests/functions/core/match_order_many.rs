use crate::setup::{create_account, setup, Defaults};
use rand::Rng;
use spark_market_sdk::{/*AssetType,*/ OrderType};

mod success {

    use fuels::types::Bits256;

    use super::*;

    struct OrderConfig {
        pub amount: u64,
        pub order_type: OrderType,
        pub price: u64,
    }

    #[tokio::test]
    async fn match_order_many_same_asset_type_same_user_equal_orders() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 75_000_000_000_000_u64; // 75,000$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Buy,
                price: price1,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price1,
            },
        ];

        let base_deposit = base_amount * 2;
        let quote_deposit =
            price1 / to_quote_scale * base_amount + price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
            .await?;
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;

        let mut order_ids: Vec<Bits256> = Vec::new();
        for config in order_configs {
            order_ids.push(
                contract
                    .with_account(&user0.wallet)
                    .await?
                    .open_order(config.amount, config.order_type, config.price)
                    .await?
                    .value,
            );
        }

        let expected_account0 = create_account(0, 0, base_deposit, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract.match_order_many(order_ids).await?;

        let expected_account0 = create_account(base_deposit, quote_deposit, 0, 0);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_order_many_same_asset_type_same_user_partial_match() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 75_000_000_000_000_u64; // 75,000$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price1,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price2,
            },
            OrderConfig {
                amount: 2 * base_amount,
                order_type: OrderType::Sell,
                price: price1,
            },
        ];

        let base_deposit = base_amount * 3;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
            .await?;
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;

        let mut order_ids: Vec<Bits256> = Vec::new();
        for config in order_configs {
            order_ids.push(
                contract
                    .with_account(&user0.wallet)
                    .await?
                    .open_order(config.amount, config.order_type, config.price)
                    .await?
                    .value,
            );
        }

        let expected_account0 = create_account(0, 0, base_deposit, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract.match_order_many(order_ids).await?;

        let expected_account0 = create_account(base_deposit, quote_deposit, 0, 0);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_order_many_big_match() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: 287573,
                order_type: OrderType::Buy,
                price: 61348523016940,
            },
            OrderConfig {
                amount: 1124659,
                order_type: OrderType::Buy,
                price: 61348575050000,
            },
            OrderConfig {
                amount: 489073,
                order_type: OrderType::Buy,
                price: 61348523016940,
            },
            OrderConfig {
                amount: 342334,
                order_type: OrderType::Buy,
                price: 61348523016940,
            },
            OrderConfig {
                amount: 1749096,
                order_type: OrderType::Buy,
                price: 61348538733430,
            },
            OrderConfig {
                amount: 440000,
                order_type: OrderType::Sell,
                price: 61169061298050,
            },
        ];

        let base_deposit = 440_000_u64;
        let quote_deposit = 100_000_000_000_u64;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
            .await?;
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;

        let mut order_ids: Vec<Bits256> = Vec::new();
        for config in order_configs {
            order_ids.push(
                contract
                    .with_account(&user0.wallet)
                    .await?
                    .open_order(config.amount, config.order_type, config.price)
                    .await?
                    .value,
            );
        }

        contract.match_order_many(order_ids).await?;

        let orders = contract.user_orders(user0.identity()).await?.value;
        assert_eq!(orders.len(), 4);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn fuzz_match_order_many() -> anyhow::Result<()> {
        for _ in 0..100 {
            let defaults = Defaults::default();
            let (contract, _, user0, user1, _, assets) = setup(
                defaults.base_decimals,
                defaults.quote_decimals,
                defaults.price_decimals,
            )
            .await?;

            // Specify the range for order amounts and prices
            let amount_range = 100_000..100_000_000; // 0.001 BTC to 1 BTC
            let price_range = 1_64..100_000_000_000_000_i64; // 0.000000001 USDC to 100k USDC
            let price_variation_range = -500..=500; // Range for price variation

            let mut rng = rand::thread_rng();
            let mut order_configs: Vec<OrderConfig> = Vec::new();

            let base_price = rng.gen_range(price_range.clone());

            for _ in 0..3 {
                // Generate a random variation within the range of -500 to 500
                let buy_price_variation: i64 = rng.gen_range(price_variation_range.clone()) + 500;
                let sell_price_variation: i64 = rng.gen_range(price_variation_range.clone()) - 500;

                // Adjust the buy and sell order prices by their respective variations
                let buy_order_price = (base_price as i64 + buy_price_variation).max(0) as u64;
                let sell_order_price = (base_price as i64 + sell_price_variation).max(0) as u64;

                let buy_order = OrderConfig {
                    amount: rng.gen_range(amount_range.clone()),
                    order_type: OrderType::Buy,
                    price: buy_order_price,
                };
                let sell_order = OrderConfig {
                    amount: rng.gen_range(amount_range.clone()),
                    order_type: OrderType::Sell,
                    price: sell_order_price,
                };
                order_configs.push(buy_order);
                order_configs.push(sell_order);
            }

            let base_deposit = 1_000_000_000_u64; // 10 BTC
            let quote_deposit = 1_000_000_000_000_u64; // 1m USDC

            // user0 deposits and opens 6 orders
            contract
                .with_account(&user0.wallet)
                .await?
                .deposit(base_deposit, assets.base.id)
                .await?;
            contract
                .with_account(&user0.wallet)
                .await?
                .deposit(quote_deposit, assets.quote.id)
                .await?;

            let mut order_ids: Vec<Bits256> = Vec::new();
            for config in &order_configs {
                order_ids.push(
                    contract
                        .with_account(&user0.wallet)
                        .await?
                        .open_order(config.amount, config.order_type.clone(), config.price)
                        .await?
                        .value,
                );
            }

            // user1 deposits and opens 6 orders
            contract
                .with_account(&user1.wallet)
                .await?
                .deposit(base_deposit, assets.base.id)
                .await?;
            contract
                .with_account(&user1.wallet)
                .await?
                .deposit(quote_deposit, assets.quote.id)
                .await?;

            for config in &order_configs {
                order_ids.push(
                    contract
                        .with_account(&user1.wallet)
                        .await?
                        .open_order(config.amount, config.order_type.clone(), config.price)
                        .await?
                        .value,
                );
            }

            let orders_0: Vec<Bits256> = contract.user_orders(user0.identity()).await?.value;
            let orders_1: Vec<Bits256> = contract.user_orders(user1.identity()).await?.value;

            // Ensure that all orders were opened
            assert!(orders_0.len() == 6);
            assert!(orders_1.len() == 6);

            let result = contract.match_order_many(order_ids.clone()).await;

            assert!(
                result.is_ok(),
                "Expected match_order_many to succeed but it failed with: {:?}",
                result.err()
            );

            let orders_0: Vec<Bits256> = contract.user_orders(user0.identity()).await?.value;
            let orders_1: Vec<Bits256> = contract.user_orders(user1.identity()).await?.value;

            // Checking that at least 1 of the 6 orders was filled based on the random input
            assert!(orders_0.len() < 6);
            assert!(orders_1.len() < 6);
        }

        Ok(())
    }

    #[tokio::test]
    async fn match_order_many_same_asset_type_same_user_equal_orders_with_matcher_fee(
    ) -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, _, matcher, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_000_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 75_000_000_000_000_u64; // 75,000$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Buy,
                price: price1,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price1,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price2,
            },
        ];

        let base_deposit = base_amount * 2;
        let quote_deposit = price1 / to_quote_scale * base_amount
            + price2 / to_quote_scale * base_amount
            + matcher_fee * 2;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
            .await?;
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;

        let mut order_ids: Vec<Bits256> = Vec::new();
        for config in order_configs {
            order_ids.push(
                contract
                    .with_account(&user0.wallet)
                    .await?
                    .open_order(config.amount, config.order_type, config.price)
                    .await?
                    .value,
            );
        }

        let expected_account = create_account(0, 0, base_deposit, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account
        );

        contract
            .with_account(&matcher.wallet)
            .await?
            .match_order_many(order_ids.clone())
            .await?;

        let expected_account = create_account(0, matcher_fee * 4, 0, 0);
        assert_eq!(
            contract.account(matcher.identity()).await?.value,
            expected_account
        );

        let expected_account = create_account(base_deposit, quote_deposit - matcher_fee * 4, 0, 0);
        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account
        );

        Ok(())
    }
}

mod revert {

    use fuels::types::Bits256;

    use super::*;

    struct OrderConfig {
        pub amount: u64,
        pub order_type: OrderType,
        pub price: u64,
    }

    #[tokio::test]
    #[should_panic(expected = "CantMatchMany")]
    async fn match_order_many_same_asset_type_same_direction() {
        let defaults = Defaults::default();
        let (contract, _, user0, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 75_000_000_000_000_u64; // 75,000$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price1,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                order_type: OrderType::Sell,
                price: price1,
            },
        ];

        let base_deposit = base_amount * 4;

        contract
            .with_account(&user0.wallet)
            .await
            .unwrap()
            .deposit(base_deposit, assets.base.id)
            .await
            .unwrap();

        let mut order_ids: Vec<Bits256> = Vec::new();
        for config in order_configs {
            order_ids.push(
                contract
                    .with_account(&user0.wallet)
                    .await
                    .unwrap()
                    .open_order(config.amount, config.order_type, config.price)
                    .await
                    .unwrap()
                    .value,
            );
        }

        contract.match_order_many(order_ids).await.unwrap();
    }
}
