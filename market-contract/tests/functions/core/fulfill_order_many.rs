use crate::setup::{create_account, setup, Defaults};
use rand::Rng;
use spark_market_sdk::{LimitType, OrderType, ProtocolFee};

mod success_ioc {

    use fuels::types::Bits256;

    use super::*;

    struct OrderConfig {
        pub amount: u64,
        pub order_type: OrderType,
        pub price: u64,
    }

    #[tokio::test]
    async fn fulfill_order_many_same_asset_type_equal_orders() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

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
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: 5 * base_amount,
            order_type: OrderType::Sell,
            price: price1,
        };

        let base_deposit = base_amount * 5;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 3 * price2 / to_quote_scale * base_amount;
        let quote_delta = 3 * (price2 - price1) / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
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

        let expected_account0 = create_account(0, 0, 0, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::IOC,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?
            .value;

        let expected_account0 = create_account(base_deposit, quote_delta, 0, 0);
        let expected_account1 = create_account(0, quote_deposit - quote_delta, 0, 0);

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
    async fn fulfill_order_many_same_asset_type_partial_fulfill_1() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price1,
            },
            OrderConfig {
                amount: 4 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: base_amount * 5,
            order_type: OrderType::Sell,
            price: price1,
        };

        let base_deposit = base_amount * 5;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 4 * price2 / to_quote_scale * base_amount;
        let quote_delta = 3 * (price2 - price1) / to_quote_scale * base_amount;
        let quote_locked = price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
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

        let expected_account0 = create_account(0, 0, 0, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::IOC,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?
            .value;

        let expected_account0 = create_account(base_deposit, quote_delta, 0, quote_locked);
        let expected_account1 = create_account(0, quote_deposit - quote_delta - quote_locked, 0, 0);

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
    async fn fulfill_order_many_same_asset_type_partial_fulfill_2() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

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
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: 4 * base_amount,
            order_type: OrderType::Sell,
            price: price1,
        };

        let base_deposit = base_amount * 4;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 3 * price2 / to_quote_scale * base_amount;
        let quote_delta = 2 * (price2 - price1) / to_quote_scale * base_amount;
        let quote_locked = price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
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

        let expected_account0 = create_account(0, 0, 0, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::IOC,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?
            .value;

        let expected_account0 = create_account(base_deposit, quote_delta, 0, quote_locked);
        let expected_account1 = create_account(0, quote_deposit - quote_delta - quote_locked, 0, 0);

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

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn fuzz_fulfill_order_many_same_asset_type_partial_fulfill() -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let defaults = Defaults::default();
            let (contract, user0, user1, _, _, assets) = setup(
                defaults.base_decimals,
                defaults.quote_decimals,
                defaults.price_decimals,
            )
            .await?;

            // Randomize the base amount and prices within certain ranges
            let base_amount: u64 = rng.gen_range(1000..10_000_000); // 0.00001 to 0.1 BTC per order
            let price1: u64 = rng.gen_range(1_000_000_000_000..100_000_000_000_000);
            let price2: u64 = rng.gen_range(price1..(price1 + 10_000_000_000_000));

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
                    amount: 2 * base_amount,
                    order_type: OrderType::Buy,
                    price: price2,
                },
            ];

            let fulfill_order_config = OrderConfig {
                amount: 10 * base_amount,
                order_type: OrderType::Sell,
                price: price1,
            };

            let base_deposit = 10 * 10_u64.pow(defaults.base_decimals); // 10 BTC
            let quote_deposit = 500_000 * 10_u64.pow(defaults.quote_decimals); // 500k USDC

            contract
                .with_account(&user0.wallet)
                .await?
                .deposit(quote_deposit, assets.quote.id)
                .await?;

            contract
                .with_account(&user1.wallet)
                .await?
                .deposit(base_deposit, assets.base.id)
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

            let user0_account_t0 = contract.account(user0.identity()).await?.value;
            let user1_account_t0 = contract.account(user1.identity()).await?.value;

            contract
                .with_account(&user1.wallet)
                .await?
                .fulfill_many(
                    fulfill_order_config.amount,
                    fulfill_order_config.order_type,
                    LimitType::IOC,
                    fulfill_order_config.price,
                    100,
                    order_ids,
                )
                .await?
                .value;

            let user0_account_t1 = contract.account(user0.identity()).await?.value;
            let user1_account_t1 = contract.account(user1.identity()).await?.value;

            // open order balance decreased
            assert!(user0_account_t1.locked.quote < user0_account_t0.locked.quote);

            // filled liquid balance increased
            assert!(user1_account_t1.liquid.quote > user1_account_t0.liquid.quote);

            // assert change in liquid amount across users is equal
            assert!(
                user1_account_t1.liquid.quote - user1_account_t0.liquid.quote
                    == user1_account_t1.liquid.quote - user1_account_t0.liquid.quote
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn fulfill_order_many_same_asset_type_equal_orders_with_matcher_fee() -> anyhow::Result<()>
    {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_000_u64;
        contract.set_matcher_fee(matcher_fee).await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64;
        let price1 = 70_000_000_000_000_u64;
        let price2 = 70_500_000_000_000_u64;

        let order_configs = vec![
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
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: 5 * base_amount,
            order_type: OrderType::Sell,
            price: price1,
        };

        // Calculate the matcher fees for both the buyer and seller
        let total_matcher_fee = matcher_fee * order_configs.len() as u64;

        let base_deposit = base_amount * 5;
        let quote_deposit = 2 * price1 / to_quote_scale * base_amount
            + 3 * price2 / to_quote_scale * base_amount
            + total_matcher_fee;

        let quote_delta = 3 * (price2 - price1) / to_quote_scale * base_amount;

        // Deposit initial amounts for users
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;

        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
            .await?;

        // Place orders and collect order IDs
        let mut total_fill_amount = 0;
        let mut order_ids = Vec::new();

        for config in &order_configs {
            total_fill_amount += config.amount;
            let order_id = contract
                .with_account(&user0.wallet)
                .await?
                .open_order(config.amount, config.order_type.clone(), config.price)
                .await?
                .value;
            order_ids.push(order_id);
        }

        // Expected balances for user0
        let expected_account0 = create_account(0, 0, 0, quote_deposit);
        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        // Fulfill orders
        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::IOC,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?;

        // Adjust expected balances
        let expected_account0 = create_account(base_deposit, quote_delta, 0, 0);
        let expected_account1 = create_account(0, quote_deposit - quote_delta, 0, 0);

        let fill_amount = total_fill_amount * (price1 / 10_u64.pow(defaults.price_decimals)) / 100;
        let calculated_matcher_fee = quote_deposit - quote_delta - fill_amount;

        // Assert matcher fee is as expected
        assert_eq!(total_matcher_fee, calculated_matcher_fee);

        // Assert final balances
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
}

mod success_fok {

    use fuels::types::Bits256;

    use super::*;

    struct OrderConfig {
        pub amount: u64,
        pub order_type: OrderType,
        pub price: u64,
    }

    #[tokio::test]
    async fn fulfill_order_many_same_asset_type_equal_orders() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

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
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: 5 * base_amount,
            order_type: OrderType::Sell,
            price: price1,
        };

        let base_deposit = base_amount * 5;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 3 * price2 / to_quote_scale * base_amount;
        let quote_delta = 3 * (price2 - price1) / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
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

        let expected_account0 = create_account(0, 0, 0, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::FOK,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?
            .value;

        let expected_account0 = create_account(base_deposit, quote_delta, 0, 0);
        let expected_account1 = create_account(0, quote_deposit - quote_delta, 0, 0);

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
    async fn fulfill_order_many_same_asset_type_partial_fulfill_1() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price1,
            },
            OrderConfig {
                amount: 4 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: base_amount * 5,
            order_type: OrderType::Sell,
            price: price1,
        };

        let base_deposit = base_amount * 5;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 4 * price2 / to_quote_scale * base_amount;
        let quote_delta = 3 * (price2 - price1) / to_quote_scale * base_amount;
        let quote_locked = price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
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

        let expected_account0 = create_account(0, 0, 0, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::FOK,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?
            .value;

        let expected_account0 = create_account(base_deposit, quote_delta, 0, quote_locked);
        let expected_account1 = create_account(0, quote_deposit - quote_delta - quote_locked, 0, 0);

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
    async fn fulfill_order_many_same_asset_type_partial_fulfill_2() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

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
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: 4 * base_amount,
            order_type: OrderType::Sell,
            price: price1,
        };

        let base_deposit = base_amount * 4;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 3 * price2 / to_quote_scale * base_amount;
        let quote_delta = 2 * (price2 - price1) / to_quote_scale * base_amount;
        let quote_locked = price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
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

        let expected_account0 = create_account(0, 0, 0, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::FOK,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?
            .value;

        let expected_account0 = create_account(base_deposit, quote_delta, 0, quote_locked);
        let expected_account1 = create_account(0, quote_deposit - quote_delta - quote_locked, 0, 0);

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
    async fn fulfill_order_many_same_asset_type_equal_orders_with_matcher_fee() -> anyhow::Result<()>
    {
        let defaults = Defaults::default();
        let (contract, _, user0, user1, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_000_u64;
        contract.set_matcher_fee(matcher_fee).await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64;
        let price1 = 70_000_000_000_000_u64;
        let price2 = 70_500_000_000_000_u64;

        let order_configs = vec![
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
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: 5 * base_amount,
            order_type: OrderType::Sell,
            price: price1,
        };

        // Calculate total matcher fee
        let total_matcher_fee = matcher_fee * order_configs.len() as u64;

        // Calculate deposits
        let base_deposit = base_amount * 5;
        let quote_deposit = 2 * price1 / to_quote_scale * base_amount
            + 3 * price2 / to_quote_scale * base_amount
            + total_matcher_fee;

        // Calculate quote delta
        let quote_delta = 3 * (price2 - price1) / to_quote_scale * base_amount;

        // Deposit initial amounts for users
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;

        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
            .await?;

        // Place orders and collect order IDs
        let mut total_fill_amount = 0;
        let mut order_ids = Vec::new();

        for config in &order_configs {
            total_fill_amount += config.amount;
            let order_id = contract
                .with_account(&user0.wallet)
                .await?
                .open_order(config.amount, config.order_type.clone(), config.price)
                .await?
                .value;
            order_ids.push(order_id);
        }

        // Expected balances for user0
        let expected_account0 = create_account(0, 0, 0, quote_deposit);
        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        // Fulfill orders
        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::FOK,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?;

        // Calculate expected balances
        let expected_account0 = create_account(base_deposit, quote_delta, 0, 0);
        let expected_account1 = create_account(0, quote_deposit - quote_delta, 0, 0);

        // Verify matcher fee
        let total_fill_amount_in_quote =
            total_fill_amount * (price1 / 10_u64.pow(defaults.price_decimals)) / 100;
        let calculated_matcher_fee = quote_deposit - quote_delta - total_fill_amount_in_quote;
        assert_eq!(total_matcher_fee, calculated_matcher_fee);

        // Assert final balances
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
    async fn fulfill_order_many_same_asset_type_equal_orders_with_protocol_fee(
    ) -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, user0, user1, _, assets) = setup(
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

        let base_amount = 1_000_u64;
        let price1 = 70_000_000_000_000_u64;
        let price2 = 70_500_000_000_000_u64;

        let order_configs = vec![
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
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: 5 * base_amount,
            order_type: OrderType::Sell,
            price: price1,
        };

        // Calculate deposits
        let base_deposit = fulfill_order_config.amount;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 3 * price2 / to_quote_scale * base_amount;
        let max_protocol_fee = quote_deposit
            * std::cmp::max(protocol_fee[0].maker_fee, protocol_fee[0].taker_fee)
            / 10_000;
        let quote_deposit = quote_deposit + max_protocol_fee;
        let trade_volume = base_deposit * price1 / to_quote_scale;
        let maker_protocol_fee = trade_volume * protocol_fee[0].maker_fee / 10_000;
        let taker_protocol_fee = trade_volume * protocol_fee[0].taker_fee / 10_000;

        // Deposit initial amounts for users
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(quote_deposit, assets.quote.id)
            .await?;

        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(base_deposit, assets.base.id)
            .await?;

        // Place orders and collect order IDs
        let mut order_ids = Vec::new();

        for config in &order_configs {
            let order_id = contract
                .with_account(&user0.wallet)
                .await?
                .open_order(config.amount, config.order_type.clone(), config.price)
                .await?
                .value;
            order_ids.push(order_id);
        }

        // Expected balances for user0
        let expected_account0 = create_account(0, 0, 0, quote_deposit);
        assert_eq!(
            contract.account(user0.identity()).await?.value,
            expected_account0
        );

        // Fulfill orders
        contract
            .with_account(&user1.wallet)
            .await?
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::FOK,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await?;

        // Calculate expected balances
        let expected_account_owner =
            create_account(0, maker_protocol_fee + taker_protocol_fee, 0, 0);
        let expected_account0 = create_account(
            base_deposit,
            quote_deposit - trade_volume - maker_protocol_fee,
            0,
            0,
        );
        let expected_account1 = create_account(0, trade_volume - taker_protocol_fee, 0, 0);

        // Assert final balances
        assert_eq!(
            contract.account(owner.identity()).await?.value,
            expected_account_owner
        );
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
    #[should_panic(expected = "CantFulfillMany")]
    async fn fulfill_order_many_ioc_same_asset_type_same_direction() {
        let defaults = Defaults::default();
        let (contract, user0, user1, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: 2 * base_amount,
                order_type: OrderType::Buy,
                price: price1,
            },
            OrderConfig {
                amount: 4 * base_amount,
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: base_amount * 5,
            order_type: OrderType::Buy,
            price: price1,
        };

        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 4 * price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await
            .unwrap()
            .deposit(quote_deposit, assets.quote.id)
            .await
            .unwrap();
        contract
            .with_account(&user1.wallet)
            .await
            .unwrap()
            .deposit(quote_deposit, assets.quote.id)
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

        let expected_account0 = create_account(0, 0, 0, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await.unwrap().value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await
            .unwrap()
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::IOC,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "CantFulfillMany")]
    async fn fulfill_order_many_ioc_same_asset_type_low_slippage() {
        let defaults = Defaults::default();
        let (contract, user0, user1, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

        let order_configs: Vec<OrderConfig> = vec![OrderConfig {
            amount: base_amount * 2,
            order_type: OrderType::Sell,
            price: price1,
        }];

        let fulfill_order_config = OrderConfig {
            amount: base_amount * 2,
            order_type: OrderType::Buy,
            price: price2,
        };

        let base_deposit = base_amount * 2;
        let quote_deposit = 2 * price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await
            .unwrap()
            .deposit(base_deposit, assets.base.id)
            .await
            .unwrap();
        contract
            .with_account(&user1.wallet)
            .await
            .unwrap()
            .deposit(quote_deposit, assets.quote.id)
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

        let expected_account0 = create_account(0, 0, base_deposit, 0);

        assert_eq!(
            contract.account(user0.identity()).await.unwrap().value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await
            .unwrap()
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::IOC,
                fulfill_order_config.price,
                0,
                order_ids,
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "CantFulfillFOK")]
    async fn fulfill_order_many_fok_same_asset_type_fok_cannot_fulfill() {
        let defaults = Defaults::default();
        let (contract, user0, user1, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 70_500_000_000_000_u64; // 70,500$ price

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
                amount: 2 * base_amount,
                /*asset_type: AssetType::Base,*/
                order_type: OrderType::Buy,
                price: price2,
            },
        ];

        let fulfill_order_config = OrderConfig {
            amount: 6 * base_amount,
            order_type: OrderType::Sell,
            price: price1,
        };

        let base_deposit = base_amount * 6;
        let quote_deposit =
            2 * price1 / to_quote_scale * base_amount + 3 * price2 / to_quote_scale * base_amount;

        contract
            .with_account(&user0.wallet)
            .await
            .unwrap()
            .deposit(quote_deposit, assets.quote.id)
            .await
            .unwrap();
        contract
            .with_account(&user1.wallet)
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

        let expected_account0 = create_account(0, 0, 0, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await.unwrap().value,
            expected_account0
        );

        contract
            .with_account(&user1.wallet)
            .await
            .unwrap()
            .fulfill_many(
                fulfill_order_config.amount,
                fulfill_order_config.order_type,
                LimitType::FOK,
                fulfill_order_config.price,
                100,
                order_ids,
            )
            .await
            .unwrap()
            .value;
    }
}
