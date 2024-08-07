use crate::setup::{setup, Defaults};

mod success {

    use super::*;
    use spark_market_sdk::SetProtocolFeeEvent;

    #[tokio::test]
    async fn sets_protocol_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let initial_fee = 15;
        let new_fee = 5;

        // Assert precondition of initial fee
        assert_eq!(contract.protocol_fee().await?.value, initial_fee);

        // Increase the fee to new_fee
        let response = contract.set_protocol_fee(new_fee).await?;

        // Log should be emitted when fee is changed
        let log = response
            .decode_logs_with_type::<SetProtocolFeeEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(*event, SetProtocolFeeEvent { amount: new_fee });

        // Check fee has changed from the initial fee
        assert_ne!(initial_fee, new_fee);
        assert_eq!(contract.protocol_fee().await?.value, new_fee);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn reverts_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let new_fee = 5;

        // Reverts
        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .set_protocol_fee(new_fee)
            .await
            .unwrap();
    }
}
