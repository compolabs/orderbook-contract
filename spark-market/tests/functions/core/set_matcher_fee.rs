use crate::setup::{setup, Defaults};

mod success {

    use super::*;
    use spark_market_sdk::SetMatcherRewardEvent;

    #[tokio::test]
    async fn sets_matcher_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let initial_fee = 0;
        let new_fee = 5;

        // Assert precondition of initial fee
        assert_eq!(contract.matcher_fee().await?.value, initial_fee);

        // Increase the fee to new_fee
        let response = contract.set_matcher_fee(new_fee).await?;

        // Log should be emitted when fee is changed
        let log = response
            .decode_logs_with_type::<SetMatcherRewardEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(*event, SetMatcherRewardEvent { amount: new_fee });

        // Check fee has changed from the initial fee
        assert_ne!(initial_fee, new_fee);
        assert_eq!(contract.matcher_fee().await?.value, new_fee);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn reverts_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, _) = setup(
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
            .set_matcher_fee(new_fee)
            .await
            .unwrap();
    }
}
