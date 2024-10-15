mod success {

    use crate::setup::{setup, Defaults};
    use spark_market_sdk::SparkMarketContract;

    #[tokio::test]
    async fn returns_config() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        assert_eq!(
            contract.config().await?.value,
            (
                assets.base.id,
                assets.base.decimals,
                assets.quote.id,
                assets.quote.decimals,
                Some(owner.address().into()),
                defaults.price_decimals,
                SparkMarketContract::sdk_version(),
            )
        );

        Ok(())
    }
}
