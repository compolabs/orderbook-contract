use crate::setup::{create_account, setup, Defaults};
use spark_market_sdk::{AssetType, OrderType};

mod success {

    use fuels::types::Bits256;

    use super::*;

    struct OrderConfig {
        pub amount: u64,
        pub asset_type: AssetType,
        pub order_type: OrderType,
        pub price: u64,
    }

    #[tokio::test]
    async fn match_order_same_asset_type_same_user_equal_orders() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, user0, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 75_000_000_000_000_u64; // 70,000$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Buy,
                price: price1,
            },
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Buy,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Sell,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
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
                    .open_order(
                        config.amount,
                        config.asset_type,
                        config.order_type,
                        config.price,
                    )
                    .await?
                    .value,
            );
        }

        let expected_account0 = create_account(0, 0, base_deposit, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value.unwrap(),
            expected_account0
        );

        contract.match_order_many(order_ids).await?;

        let expected_account0 = create_account(base_deposit, quote_deposit, 0, 0);

        assert_eq!(
            contract.account(user0.identity()).await?.value.unwrap(),
            expected_account0
        );

        Ok(())
    }

    #[tokio::test]
    async fn match_order_same_asset_type_same_user_partial_match() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, user0, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 75_000_000_000_000_u64; // 70,000$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: 2 * base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Buy,
                price: price1,
            },
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Buy,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Sell,
                price: price2,
            },
            OrderConfig {
                amount: 2 * base_amount,
                asset_type: AssetType::Base,
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
                    .open_order(
                        config.amount,
                        config.asset_type,
                        config.order_type,
                        config.price,
                    )
                    .await?
                    .value,
            );
        }

        let expected_account0 = create_account(0, 0, base_deposit, quote_deposit);

        assert_eq!(
            contract.account(user0.identity()).await?.value.unwrap(),
            expected_account0
        );

        contract.match_order_many(order_ids).await?;

        let expected_account0 = create_account(base_deposit, quote_deposit, 0, 0);

        assert_eq!(
            contract.account(user0.identity()).await?.value.unwrap(),
            expected_account0
        );

        Ok(())
    }
}

mod revert {

    use fuels::types::Bits256;

    use super::*;

    struct OrderConfig {
        pub amount: u64,
        pub asset_type: AssetType,
        pub order_type: OrderType,
        pub price: u64,
    }

    #[tokio::test]
    #[should_panic(expected = "CantMatchMany")]
    async fn match_order_same_asset_type_same_direction() {
        let defaults = Defaults::default();
        let (contract, user0, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let base_amount = 1_000_u64; // 0.00001 BTC
        let price1 = 70_000_000_000_000_u64; // 70,000$ price
        let price2 = 75_000_000_000_000_u64; // 70,000$ price

        let order_configs: Vec<OrderConfig> = vec![
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Sell,
                price: price1,
            },
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Sell,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
                order_type: OrderType::Sell,
                price: price2,
            },
            OrderConfig {
                amount: base_amount,
                asset_type: AssetType::Base,
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
                    .open_order(
                        config.amount,
                        config.asset_type,
                        config.order_type,
                        config.price,
                    )
                    .await
                    .unwrap()
                    .value,
            );
        }

        let expected_account0 = create_account(0, 0, base_deposit, 0);

        assert_eq!(
            contract
                .account(user0.identity())
                .await
                .unwrap()
                .value
                .unwrap(),
            expected_account0
        );

        contract.match_order_many(order_ids).await.unwrap();
    }
}
