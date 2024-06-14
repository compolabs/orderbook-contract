use crate::setup::{setup, Defaults};
use spark_market_sdk::{AssetType, OrderType};

mod success {

    use super::*;

    #[tokio::test]
    async fn match_same_asset_type_orders_same_price() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, user0, user1, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000_000_000_000_u64; // 70,000$ price
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount;
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(base_amount, assets.base.id)
            .await?;
        println!("{}", quote_amount);
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(quote_amount, assets.quote.id)
            .await?;

        let id0 = contract
            .with_account(&user0.wallet)
            .await?
            .open_order(base_amount, AssetType::Base, OrderType::Sell, price)
            .await?
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .await?
            .open_order(base_amount, AssetType::Base, OrderType::Buy, price)
            .await?
            .value;
        Ok(())
    }

    #[tokio::test]
    async fn match_same_asset_type_orders_size_equal() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        Ok(())
    }

    async fn match_same_asset_type_orders_size_not_equal() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        Ok(())
    }
}
