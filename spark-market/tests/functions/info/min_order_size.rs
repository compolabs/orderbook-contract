mod success {

    use crate::setup::{setup, Defaults};

    #[tokio::test]
    async fn returns_min_sell_order_size() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, _, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        assert_eq!(contract.min_order_size().await?.value, 0_u64);

        // Change fee to be non-zero for testing purposes
        let min_sell_order_size = 10000_u64;

        let _ = contract.set_min_order_size(min_sell_order_size).await?;

        assert_eq!(contract.min_order_size().await?.value, min_sell_order_size);

        Ok(())
    }
}
