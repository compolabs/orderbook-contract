mod success {

    use crate::setup::{create_account, setup, Defaults};

    #[tokio::test]
    async fn returns_none() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        assert!(contract.account(owner.identity()).await?.value.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn returns_account_info() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let deposit_amount = 100;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let _ = contract.deposit(deposit_amount, assets.base.id).await?;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();

        assert_eq!(user_account, expected_account);

        Ok(())
    }
}
