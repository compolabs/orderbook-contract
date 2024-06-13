use crate::setup::{setup, Defaults};
use spark_market_sdk::{AssetType, OrderType};

mod success {

    use super::*;
    use crate::setup::create_account;
    use spark_market_sdk::OpenOrderEvent;

    #[tokio::test]
    async fn match_same_asset_type_orders_same_price() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

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
