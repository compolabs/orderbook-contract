mod success {

    use crate::setup::{setup, Defaults};
    //use spark_market_sdk::AssetType;

    #[tokio::test]
    async fn returns_protocol_fee_amount_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let amount = 100000000; // 1 BTC
        /*let protocol_fee_amount_in_btc =
            amount * contract.protocol_fee().await?.value as u64 / 10_000;
        let protocol_fee_amount_in_eth = protocol_fee_amount_in_btc as f64 * 18.92 * 10. /* ETH decimals - BTC decimals */;
        assert_eq!(
            contract
                .protocol_fee_amount(amount /*, AssetType::Base*/)
                .await?
                .value,
            protocol_fee_amount_in_eth as u64
        );*/
        assert!(false);

        Ok(())
    }

}
