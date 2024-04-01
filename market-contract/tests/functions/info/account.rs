mod success {

    use crate::utils::{
        interface::{core::deposit, info::account},
        setup::{create_account, setup},
    };

    #[tokio::test]
    async fn returns_none() {
        let (contract, owner, _user, _assets) = setup(9, 9, 9).await;
        assert!(account(&contract, owner).await.value.is_none());
    }

    #[tokio::test]
    async fn returns_account_info() {
        let (contract, owner, _user, assets) = setup(9, 9, 9).await;
        let deposit_amount = 100;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        deposit(&contract, deposit_amount, assets.base.id).await;

        let user_account = account(&contract, owner).await.value.unwrap();

        assert_eq!(user_account, expected_account);
    }
}
