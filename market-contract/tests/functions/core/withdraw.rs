use crate::setup::{setup, Defaults};

mod success {

    use super::*;
    use crate::setup::create_account;
    use spark_market_sdk::{AssetType, WithdrawEvent};

    #[tokio::test]
    async fn base_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 100;

        let _ = contract.deposit(deposit_amount, assets.base.id).await?;

        let user_balance = owner.balance(&assets.base.id).await;
        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        // Precondition enforces deposited account
        assert_eq!(user_account, expected_account);

        let response = contract.withdraw(deposit_amount, AssetType::Base).await?;

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
        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(0, 0, 0, 0);

        assert_eq!(new_balance, user_balance + deposit_amount);
        assert_eq!(user_account, expected_account);

        Ok(())
    }

    #[tokio::test]
    async fn quote_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 100;

        let _ = contract.deposit(deposit_amount, assets.quote.id).await?;

        let user_balance = owner.balance(&assets.quote.id).await;
        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        // Precondition enforces deposited account
        assert_eq!(user_account, expected_account);

        let response = contract.withdraw(deposit_amount, AssetType::Quote).await?;

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
        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(0, 0, 0, 0);

        assert_eq!(new_balance, user_balance + deposit_amount);
        assert_eq!(user_account, expected_account);

        Ok(())
    }
}

mod revert {

    use super::*;
    use spark_market_sdk::AssetType;

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_withdrawing_without_account() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 100;

        // Revert
        contract
            .withdraw(deposit_amount, AssetType::Base)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_base_amount_greater_than_available() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 100;

        let _ = contract
            .deposit(deposit_amount, assets.base.id)
            .await
            .unwrap();

        // Revert
        contract
            .withdraw(deposit_amount + 1, AssetType::Base)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_quote_amount_greater_than_available() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 100;

        let _ = contract
            .deposit(deposit_amount, assets.quote.id)
            .await
            .unwrap();

        // Revert
        contract
            .withdraw(deposit_amount + 1, AssetType::Quote)
            .await
            .unwrap();
    }
}
