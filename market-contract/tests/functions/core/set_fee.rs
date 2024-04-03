use crate::utils::{
    interface::core::set_fee,
    setup::{setup, Defaults},
};

mod success {

    use super::*;
    use crate::utils::{interface::info::fee, setup::SetFeeEvent};

    #[tokio::test]
    async fn sets_global_fee() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let initial_fee = 0;
        let new_fee = 5;
        let user = None;

        // Assert precondition of initial fee
        assert_eq!(fee(&contract, user.clone()).await.value, initial_fee);

        // Increase the fee to new_fee
        let response = set_fee(&contract, new_fee, user.clone()).await;

        // Log should be emitted when fee is changed
        let log = response.decode_logs_with_type::<SetFeeEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            SetFeeEvent {
                amount: new_fee,
                user: user.clone(),
            }
        );

        // Check fee has changed from the initial fee
        assert_ne!(initial_fee, new_fee);
        assert_eq!(fee(&contract, user).await.value, initial_fee + new_fee);
    }

    #[tokio::test]
    async fn sets_premium_user_fee() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let initial_fee = 0;
        let new_fee = 5;
        let user = Some(user.identity());

        // Assert precondition of initial fee
        assert_eq!(fee(&contract, user.clone()).await.value, initial_fee);

        // Increase the fee to new_fee
        let response = set_fee(&contract, new_fee, user.clone()).await;

        // Log should be emitted when fee is changed
        let log = response.decode_logs_with_type::<SetFeeEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            SetFeeEvent {
                amount: new_fee,
                user: user.clone(),
            }
        );

        // Check fee has changed from the initial fee
        assert_ne!(initial_fee, new_fee);
        assert_eq!(fee(&contract, user).await.value, initial_fee + new_fee);
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
        .await;

        let new_fee = 5;

        // Reverts
        set_fee(
            &contract.with_account(user.wallet.clone()),
            new_fee,
            Some(user.identity()),
        )
        .await;
    }
}
