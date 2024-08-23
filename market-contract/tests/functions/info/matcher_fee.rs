mod success {

    use crate::setup::{setup, Defaults};

    #[tokio::test]
    async fn returns_matcher_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        assert_eq!(contract.matcher_fee().await?.value, 0_u64);

        // Change fee to be non-zero for testing purposes
        let matcher_fee = 1000_u64;

        let _ = contract.set_matcher_fee(matcher_fee).await?;

        assert_eq!(contract.matcher_fee().await?.value, matcher_fee);

        Ok(())
    }
}
