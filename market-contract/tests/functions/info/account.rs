mod success {

    use crate::setup::{create_account, setup, Defaults};

    #[tokio::test]
    async fn returns_account_zeros() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let expected_account = create_account(0, 0, 0, 0);

        assert_eq!(
            contract.account(owner.identity()).await?.value,
            expected_account
        );

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

        let user_account = contract.account(owner.identity()).await?.value;

        assert_eq!(user_account, expected_account);

        Ok(())
    }
}
