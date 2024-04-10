mod success {

    use crate::setup::{setup, Defaults};

    #[tokio::test]
    async fn returns_global_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        // Change fee to be non-zero for testing purposes
        let global_fee = 5;
        let user = None;

        let _ = contract.set_fee(global_fee, user.clone()).await?;

        assert_eq!(contract.fee(user).await?.value, global_fee);

        Ok(())
    }

    #[tokio::test]
    async fn returns_premium_user_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        // Change fee to be non-zero for testing purposes
        let global_fee = 5;
        let user_fee = 1;

        let _ = contract.set_fee(global_fee, None).await?;
        let _ = contract.set_fee(user_fee, Some(user.identity())).await?;

        assert_eq!(contract.fee(Some(user.identity())).await?.value, user_fee);

        Ok(())
    }

    #[tokio::test]
    async fn returns_global_fee_for_non_premium_user() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        // Change fee to be non-zero for testing purposes
        let global_fee = 5;

        let _ = contract.set_fee(global_fee, Some(user.identity())).await?;

        assert_eq!(contract.fee(Some(user.identity())).await?.value, global_fee);

        Ok(())
    }
}
