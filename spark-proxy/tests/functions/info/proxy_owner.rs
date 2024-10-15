mod success {

    use crate::setup::setup_proxy;
    use spark_proxy_sdk::State;

    #[tokio::test]
    async fn returns_proxy_target() -> anyhow::Result<()> {
        let (contract, owner, _) = setup_proxy().await?;

        assert_eq!(
            contract.proxy_owner().await?.value,
            State::Initialized(owner.identity())
        );

        Ok(())
    }
}
