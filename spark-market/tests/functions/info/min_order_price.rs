mod success {

    use crate::setup::{setup, Defaults};

    #[tokio::test]
    async fn returns_min_order_price() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, _, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        assert_eq!(contract.min_order_price().await?.value, 0_u64);

        // Change fee to be non-zero for testing purposes
        let min_order_price = 1_000_000_000_u64;

        let _ = contract.set_min_order_price(min_order_price).await?;

        assert_eq!(contract.min_order_price().await?.value, min_order_price);

        Ok(())
    }
}
