use crate::setup::setup_proxy;

mod success {

    use super::*;

    #[tokio::test]
    async fn set_proxy_target() -> anyhow::Result<()> {
        let (contract, _, _) = setup_proxy().await?;

        contract
            .set_proxy_target(contract.contract_id().into())
            .await?;

        assert_eq!(
            contract.proxy_target().await?.value,
            Some(contract.contract_id().into())
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

        // Attempt to set the target with a non-owner user
        contract
            .with_account(&user.wallet)
            .set_proxy_target(contract.contract_id().into())
            .await
            .unwrap();
    }
}
