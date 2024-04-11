use crate::setup::{setup, Defaults};

mod success {

    use super::*;
    use crate::setup::create_account;
    use spark_market_sdk::DepositEvent;

    #[tokio::test]
    async fn base_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 100;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        // Precondition enforces empty account
        assert!(contract.account(owner.identity()).await?.value.is_none());

        let response = contract.deposit(deposit_amount, assets.base.id).await?;
        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            DepositEvent {
                amount: deposit_amount,
                asset: assets.base.id,
                user: owner.identity(),
            }
        );

        let user_account = contract.account(owner.identity()).await?.value.unwrap();

        assert_eq!(user_account, expected_account);

        Ok(())
    }

    #[tokio::test]
    async fn quote_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 100;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        // Precondition enforces empty account
        assert!(contract.account(owner.identity()).await?.value.is_none());

        let response = contract.deposit(deposit_amount, assets.quote.id).await?;
        let log = response.decode_logs_with_type::<DepositEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            DepositEvent {
                amount: deposit_amount,
                asset: assets.quote.id,
                user: owner.identity(),
            }
        );

        let user_account = contract.account(owner.identity()).await?.value.unwrap();

        assert_eq!(user_account, expected_account);

        Ok(())
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
        .await
        .unwrap();

        let deposit_amount = 100;

        // Revert
        contract
            .deposit(deposit_amount, assets.random.id)
            .await
            .unwrap();
    }
}
