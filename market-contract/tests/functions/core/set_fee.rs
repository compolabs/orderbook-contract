use crate::setup::{setup, Defaults};

mod success {

    use super::*;
    use spark_market_sdk::SetFeeEvent;

    #[tokio::test]
    async fn sets_global_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let initial_fee = 0;
        let new_fee = 5;
        let user = None;

        // Assert precondition of initial fee
        assert_eq!(contract.fee(user.clone()).await?.value, initial_fee);

        // Increase the fee to new_fee
        let response = contract.set_fee(new_fee, user.clone()).await?;

        // Log should be emitted when fee is changed
        let log = response.decode_logs_with_type::<SetFeeEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            SetFeeEvent {
                amount: new_fee,
                user: user.clone(),
            }
        );

        // Check fee has changed from the initial fee
        assert_ne!(initial_fee, new_fee);
        assert_eq!(contract.fee(user).await?.value, initial_fee + new_fee);

        Ok(())
    }

    #[tokio::test]
    async fn sets_premium_user_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let initial_fee = 0;
        let new_fee = 5;
        let user = Some(user.identity());

        // Assert precondition of initial fee
        assert_eq!(contract.fee(user.clone()).await?.value, initial_fee);

        // Increase the fee to new_fee
        let response = contract.set_fee(new_fee, user.clone()).await?;

        // Log should be emitted when fee is changed
        let log = response.decode_logs_with_type::<SetFeeEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            SetFeeEvent {
                amount: new_fee,
                user: user.clone(),
            }
        );

        // Check fee has changed from the initial fee
        assert_ne!(initial_fee, new_fee);
        assert_eq!(contract.fee(user).await?.value, initial_fee + new_fee);

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
            .set_fee(new_fee, Some(user.identity()))
            .await
            .unwrap();
    }
}
