mod success {

    use crate::setup::setup;

    #[tokio::test]
    async fn returns_config() -> anyhow::Result<()> {
        let (contract, owner, _) = setup().await.unwrap();
        assert_eq!(contract.config().await?.value, owner.address(),);
        Ok(())
    }
}
