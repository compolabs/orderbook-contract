mod success {

    use crate::setup::{setup, Defaults};
    use spark_market_sdk::ProtocolFee;

    #[tokio::test]
    async fn returns_protocol_fee_user_amount_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 10,
                taker_fee: 15,
                volume_threshold: 0,
            },
            ProtocolFee {
                maker_fee: 8,
                taker_fee: 13,
                volume_threshold: 10000,
            },
            ProtocolFee {
                maker_fee: 6,
                taker_fee: 10,
                volume_threshold: 20000,
            },
        ];

        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;
        let amount = 100_000_000u64;
        let handred_percent = 10_000u64;
        assert_eq!(
            contract
                .protocol_fee_user_amount(amount, _user.address().into())
                .await?
                .value,
            (
                amount * protocol_fee[0].maker_fee / handred_percent,
                amount * protocol_fee[0].taker_fee / handred_percent
            )
        );

        Ok(())
    }
}
