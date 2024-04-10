use crate::utils::{
    interface::core::{deposit, withdraw},
    setup::{setup, Defaults},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::account,
        setup::{create_account, WithdrawEvent},
    };

    #[ignore]
    #[tokio::test]
    async fn base_asset() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;

        deposit(&contract, deposit_amount, assets.base.id).await;

        let user_balance = owner.balance(&assets.base.id).await;
        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        // Precondition enforces deposited account
        assert_eq!(user_account, expected_account);

        let response = withdraw(&contract, deposit_amount, assets.base.id).await;

        let log = response.decode_logs_with_type::<WithdrawEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            WithdrawEvent {
                amount: deposit_amount,
                asset: assets.base.id,
                user: owner.identity(),
            }
        );

        let new_balance = owner.balance(&assets.base.id).await;
        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, 0, 0, 0);

        assert_eq!(new_balance, user_balance + deposit_amount);
        assert_eq!(user_account, expected_account);
    }

    #[ignore]
    #[tokio::test]
    async fn quote_asset() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;

        deposit(&contract, deposit_amount, assets.quote.id).await;

        let user_balance = owner.balance(&assets.quote.id).await;
        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, deposit_amount, 0, 0);

        // Precondition enforces deposited account
        assert_eq!(user_account, expected_account);

        let response = withdraw(&contract, deposit_amount, assets.quote.id).await;

        let log = response.decode_logs_with_type::<WithdrawEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            WithdrawEvent {
                amount: deposit_amount,
                asset: assets.quote.id,
                user: owner.identity(),
            }
        );

        let new_balance = owner.balance(&assets.quote.id).await;
        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, 0, 0, 0);

        assert_eq!(new_balance, user_balance + deposit_amount);
        assert_eq!(user_account, expected_account);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_invalid_asset() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;

        deposit(&contract, deposit_amount, assets.base.id).await;

        // Revert
        withdraw(&contract, deposit_amount, assets.random.id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidUser")]
    async fn when_withdrawing_without_account() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;

        // Revert
        withdraw(&contract, deposit_amount, assets.base.id).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_base_amount_greater_than_available() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;

        deposit(&contract, deposit_amount, assets.base.id).await;

        // Revert
        withdraw(&contract, deposit_amount + 1, assets.base.id).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_quote_amount_greater_than_available() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;

        deposit(&contract, deposit_amount, assets.quote.id).await;

        // Revert
        withdraw(&contract, deposit_amount + 1, assets.quote.id).await;
    }
}
