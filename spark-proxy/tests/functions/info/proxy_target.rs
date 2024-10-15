mod success {

    use fuels::types::ContractId;

    use crate::setup::setup_proxy;

    #[tokio::test]
    async fn returns_proxy_target() -> anyhow::Result<()> {
        let (contract, _, _) = setup_proxy().await?;

        assert_eq!(
            contract.proxy_target().await?.value,
            Some(ContractId::zeroed().into())
        );

        Ok(())
    }
}
