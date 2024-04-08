mod success {

    use crate::utils::{
        interface::{core::deposit, info::account},
        setup::{create_account, setup, Defaults},
    };

    #[tokio::test]
    async fn returns_none() {
        let defaults = Defaults::default();
        let (contract, owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;
        assert!(account(&contract, owner.identity()).await.value.is_none());
    }

    #[tokio::test]
    async fn returns_account_info() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;
        let deposit_amount = 100;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        deposit(&contract, deposit_amount, assets.base.asset_id).await;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();

        assert_eq!(user_account, expected_account);
    }
}
