mod success {

    use crate::setup::{random_asset_id, setup};
    use fuels::types::ContractId;
    use spark_market_sdk::MarketContract;

    #[tokio::test]
    async fn succeeds_for_admin() -> anyhow::Result<()> {
        let (contract, admin, _) = setup().await.unwrap();
        let base_asset = random_asset_id(20);
        let quote_asset = random_asset_id(21);

        let market = MarketContract::deploy(
            base_asset,
            1,
            quote_asset,
            1,
            9,
            admin.wallet.clone(),
            *admin.wallet.provider().unwrap().base_asset_id(),
        )
        .await?;

        let contract_id: ContractId = market.contract_id().into();
        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .register_market(contract_id)
            .await
            .unwrap();
        assert_eq!(
            contract
                .markets(vec![(base_asset, quote_asset)])
                .await?
                .value,
            vec![(base_asset, quote_asset, Some(contract_id))]
        );
        Ok(())
    }
}

mod revert {

    use crate::setup::{random_asset_id, random_contract_id, setup};
    use fuels::types::ContractId;
    use spark_market_sdk::MarketContract;

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn reverts_when_non_owner() {
        let (contract, _, user) = setup().await.unwrap();
        let base_asset = random_asset_id(20);
        let quote_asset = random_asset_id(21);

        let market = MarketContract::deploy(
            base_asset,
            1,
            quote_asset,
            1,
            9,
            user.wallet.clone(),
            *user.wallet.provider().unwrap().base_asset_id(),
        )
        .await
        .unwrap();

        let contract_id: ContractId = market.contract_id().into();

        // Reverts
        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .register_market(contract_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "MarketAlreadyRegistered")]
    async fn reverts_when_registered() {
        let (contract, admin, _) = setup().await.unwrap();

        let base_asset = random_asset_id(20);
        let quote_asset = random_asset_id(21);

        let market = MarketContract::deploy(
            base_asset,
            1,
            quote_asset,
            1,
            9,
            admin.wallet.clone(),
            *admin.wallet.provider().unwrap().base_asset_id(),
        )
        .await
        .unwrap();

        let contract_id: ContractId = market.contract_id().into();

        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .register_market(contract_id)
            .await
            .unwrap();

        // Reverts
        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .register_market(contract_id)
            .await
            .unwrap();
    }
}
