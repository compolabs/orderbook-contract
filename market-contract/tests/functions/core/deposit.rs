use crate::utils::{
    interface::core::deposit,
    setup::{setup, Defaults},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::account,
        setup::{create_account, DepositEvent},
    };

    #[tokio::test]
    async fn deposit_base_asset() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        // Precondition enforces empty account
        assert!(account(&contract, owner.identity()).await.value.is_none());

        let response = deposit(&contract, deposit_amount, assets.base.id).await;
        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            DepositEvent {
                amount: deposit_amount,
                asset: assets.base.id,
                user: owner.identity(),
            }
        );

        let user_account = account(&contract, owner.identity()).await.value.unwrap();

        assert_eq!(user_account, expected_account);
    }

    #[tokio::test]
    async fn deposit_quote_asset() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        // Precondition enforces empty account
        assert!(account(&contract, owner.identity()).await.value.is_none());

        let response = deposit(&contract, deposit_amount, assets.quote.id).await;
        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(
            *event,
            DepositEvent {
                amount: deposit_amount,
                asset: assets.quote.id,
                user: owner.identity(),
            }
        );

        let user_account = account(&contract, owner.identity()).await.value.unwrap();

        assert_eq!(user_account, expected_account);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_deposit_random_asset() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;

        // Revert
        deposit(&contract, deposit_amount, assets.random.id).await;
    }
}
