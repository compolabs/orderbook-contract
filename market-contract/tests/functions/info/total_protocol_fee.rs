mod success {

    use crate::setup::{setup, Defaults};
    use spark_market_sdk::{AssetType, OrderType};

    #[tokio::test]
    async fn returns_total_protocol_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 10;

        let order_amount = 10;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70_000_000_000_000_u64;

        let _ = contract.deposit(deposit_amount, asset).await?;

        let _ = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(orders, vec![]);

        let _ = contract
            .open_order(order_amount, AssetType::Base, order_type.clone(), price)
            .await?;

        assert_eq!(contract.total_protocol_fee().await?.value, 2);

        Ok(())
    }
}
