use crate::setup::{setup, Defaults};
use spark_market_sdk::{/*AssetType,*/ OrderType};

mod success {

    use super::*;

    #[tokio::test]
    async fn orders_create_different_ids() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let id1 = contract
            .order_id(
                /*AssetType::Base,*/
                OrderType::Buy,
                owner.identity(),
                70_000_000_000_000_u64,
                2,
            )
            .await?;
        let id2 = contract
            .order_id(
                /*AssetType::Base,*/
                OrderType::Buy,
                owner.identity(),
                80_000_000_000_000_u64,
                2,
            )
            .await?;

        assert_ne!(id1.value, id2.value);

        Ok(())
    }

    #[tokio::test]
    async fn accepts_base_asset() -> anyhow::Result<()> {
        // In this test we only care about the test not reverting with the correct asset
        let defaults = Defaults::default();
        let (contract, owner, _user, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let _ = contract
            .order_id(
                /*AssetType::Base,*/
                OrderType::Buy,
                owner.identity(),
                70_000_000_000_000_u64,
                2,
            )
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn accepts_quote_asset() -> anyhow::Result<()> {
        // In this test we only care about the test not reverting with the correct asset
        let defaults = Defaults::default();
        let (contract, owner, _user, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let _ = contract
            .order_id(
                /*AssetType::Base,*/
                OrderType::Buy,
                owner.identity(),
                70_000_000_000_000_u64,
                2,
            )
            .await?;

        Ok(())
    }
}
