mod success {

    use crate::setup::{setup, Defaults};

    #[tokio::test]
    async fn returns_protocol_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        // Change fee to be non-zero for testing purposes
        let protocol_fee = 5;

        /*let _ = contract.set_protocol_fee(protocol_fee).await?;

        assert_eq!(contract.protocol_fee().await?.value, protocol_fee);*/
        assert!(false);

        Ok(())
    }
}
