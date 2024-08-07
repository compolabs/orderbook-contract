mod success {

    use crate::setup::{setup, Defaults};
    use spark_market_sdk::{/*AssetType,*/ OrderType};

    #[tokio::test]
    async fn returns_zero_orders() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(orders, vec![]);

        Ok(())
    }

    #[tokio::test]
    async fn returns_orders() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let _ = contract.deposit(1000, assets.base.id).await?;
        let id1 = contract
            .open_order(
                2,
                /*AssetType::Base,*/ OrderType::Sell,
                70_000_000_000_000_u64,
            )
            .await?;
        let id2 = contract
            .open_order(
                1,
                /*AssetType::Base,*/ OrderType::Sell,
                75_000_000_000_000_u64,
            )
            .await?;

        let mut orders = contract.user_orders(owner.identity()).await?.value;

        assert_eq!(2, orders.len());
        assert_eq!(id2.value, orders.pop().unwrap());
        assert_eq!(id1.value, orders.pop().unwrap());

        Ok(())
    }
}
