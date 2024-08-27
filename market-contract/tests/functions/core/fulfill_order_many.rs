use crate::setup::{create_account, setup, Defaults};
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::{LimitType, OrderType};

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
        let (contract, user0, user1, _, _, assets) = setup(
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
        let (contract, user0, user1, _, _, assets) = setup(
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
        let (contract, user0, user1, _, _, assets) = setup(
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

    #[tokio::test]
    async fn fulfill_order_many_same_asset_type_equal_orders_with_matcher_fee() -> anyhow::Result<()>
    {
        let defaults = Defaults::default();
        let (contract, user0, user1, matcher, _, assets) = setup(
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
        let (contract, user0, user1, _, _, assets) = setup(
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
        let (contract, user0, user1, _, _, assets) = setup(
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
        let (contract, user0, user1, _, _, assets) = setup(
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
        let (contract, user0, user1, _, _, assets) = setup(
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
    #[should_panic(expected = "CantFulfillMany")]
    async fn fulfill_order_many_fok_same_asset_type_cannot_fully_match() {
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
