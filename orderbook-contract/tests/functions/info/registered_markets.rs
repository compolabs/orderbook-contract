mod success {

    use crate::setup::{random_asset_id, setup};

    #[tokio::test]
    async fn returns_id_none() -> anyhow::Result<()> {
        let (contract, _, _) = setup().await.unwrap();
        let asset_id = random_asset_id();
        assert_eq!(
            contract.registered_markets(vec![asset_id]).await?.value,
            vec![(asset_id, None)]
        );
        Ok(())
    }
}
