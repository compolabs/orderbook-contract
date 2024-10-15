use crate::setup::setup_proxy;
use spark_proxy_sdk::State;

mod success {

    use super::*;

    #[tokio::test]
    async fn set_proxy_owner() -> anyhow::Result<()> {
        let (contract, _, user) = setup_proxy().await?;

        contract
            .set_proxy_owner(State::Initialized(user.identity()))
            .await?;

        assert_eq!(
            contract.proxy_owner().await?.value,
            State::Initialized(user.identity())
        );

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_when_non_owner() {
        let (contract, _, user) = setup_proxy().await.unwrap();

        // Attempt to set the owner with a non-owner user
        contract
            .with_account(&user.wallet)
            .set_proxy_owner(State::Initialized(user.identity()))
            .await
            .unwrap();
    }
}
