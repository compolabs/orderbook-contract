mod success {

    use crate::utils::{
        interface::{core::set_fee, info::fee},
        setup::{setup, Defaults},
    };

    #[tokio::test]
    async fn returns_global_fee() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        // Change fee to be non-zero for testing purposes
        let global_fee = 5;
        let user = None;

        set_fee(&contract, global_fee, user.clone()).await;

        assert_eq!(fee(&contract, user).await.value, global_fee);
    }

    #[tokio::test]
    async fn returns_premium_user_fee() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        // Change fee to be non-zero for testing purposes
        let global_fee = 5;
        let user_fee = 1;

        set_fee(&contract, global_fee, None).await;
        set_fee(&contract, user_fee, Some(user.identity())).await;

        assert_eq!(fee(&contract, Some(user.identity())).await.value, user_fee);
    }

    #[tokio::test]
    async fn returns_global_fee_for_non_premium_user() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        // Change fee to be non-zero for testing purposes
        let global_fee = 5;

        set_fee(&contract, global_fee, Some(user.identity())).await;

        assert_eq!(
            fee(&contract, Some(user.identity())).await.value,
            global_fee
        );
    }
}
