use crate::setup::{setup, Defaults};

mod success {

    use super::*;

    #[tokio::test]
    async fn pause() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        // Assert precondition of initial fee
        assert_eq!(contract.is_paused().await?.value, false);

        // Increase the fee to new_fee
        let _ = contract.pause().await?;

        // Check fee has changed from the initial fee
        assert_eq!(contract.is_paused().await?.value, true);

        // Increase the fee to new_fee
        let _ = contract.unpause().await?;

        // Assert precondition of initial fee
        assert_eq!(contract.is_paused().await?.value, false);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_pause_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        // Reverts
        contract.with_account(&user.wallet).pause().await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_unpause_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        // Reverts
        contract.with_account(&user.wallet).unpause().await.unwrap();
    }
}
