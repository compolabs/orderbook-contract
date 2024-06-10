mod success {

    use crate::setup::{random_asset_id, random_contract_id, setup};

    #[tokio::test]
    async fn succeeds_for_admin() -> anyhow::Result<()> {
        let (contract, admin, user) = setup().await.unwrap();
        let asset_id = random_asset_id();
        let contract_id = random_contract_id();
        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .register_market(asset_id, contract_id)
            .await
            .unwrap();
        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .unregister_market(asset_id)
            .await
            .unwrap();
        assert_eq!(
            contract.registered_markets(vec![asset_id]).await?.value,
            vec![(asset_id, None)]
        );
        Ok(())
    }
}

mod revert {

    use crate::setup::{random_asset_id, random_contract_id, setup};

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn reverts_when_non_owner() {
        let (contract, admin, user) = setup().await.unwrap();
        let asset_id = random_asset_id();
        let contract_id = random_contract_id();
        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .register_market(asset_id, contract_id)
            .await
            .unwrap();
        // Reverts
        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .register_market(asset_id, contract_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "MarketNotRegistered")]
    async fn reverts_when_not_registered() {
        let (contract, admin, _) = setup().await.unwrap();
        let asset_id = random_asset_id();

        // Reverts
        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .unregister_market(asset_id)
            .await
            .unwrap();
    }
}
