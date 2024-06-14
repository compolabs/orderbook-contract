mod success {

    use crate::setup::{setup, Defaults};
    use fuels::types::Bits256;
    use spark_market_sdk::{AssetType, OrderType};

    #[tokio::test]
    async fn returns_none() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        assert!(contract.order(Bits256([0u8; 32])).await?.value.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn returns_order() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let _ = contract.deposit(100, assets.base.id).await?;
        let id = contract
            .open_order(1, AssetType::Base, OrderType::Buy, 70000)
            .await?;

        let order = contract.order(id.value).await?.value.unwrap();
        let expected_id = contract
            .order_id(
                order.amount,
                order.asset_type,
                order.order_type,
                order.owner,
                order.price,
            )
            .await?;

        assert_eq!(id.value, expected_id.value);

        Ok(())
    }
}
