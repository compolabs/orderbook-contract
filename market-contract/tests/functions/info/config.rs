mod success {

    use crate::setup::{setup, Defaults};

    #[tokio::test]
    async fn returns_config() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        assert_eq!(
            contract.config().await?.value,
            (
                owner.address(),
                assets.base.id,
                assets.base.decimals,
                assets.quote.id,
                assets.quote.decimals,
                defaults.price_decimals,
                assets.fuel.id,
            )
        );

        Ok(())
    }
}
