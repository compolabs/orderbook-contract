mod success {

    use crate::setup::{random_asset_id, setup};
    use fuels::types::ContractId;
    use spark_market_sdk::SparkMarketContract;

    #[tokio::test]
    async fn succeeds_for_admin() -> anyhow::Result<()> {
        let (contract, admin, _) = setup().await.unwrap();
        let base_asset = random_asset_id(20);
        let quote_asset = random_asset_id(21);

        let market = SparkMarketContract::deploy(
            base_asset,
            1,
            quote_asset,
            1,
            admin.wallet.clone(),
            9,
            0xFAFBFC,
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
        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .unregister_market(contract_id)
            .await
            .unwrap();
        assert_eq!(
            contract
                .markets(vec![(base_asset, quote_asset)])
                .await?
                .value,
            vec![(base_asset, quote_asset, None)]
        );
        Ok(())
    }
}

mod revert {

    use crate::setup::{random_asset_id, setup};
    use fuels::types::ContractId;
    use spark_market_sdk::SparkMarketContract;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_when_non_owner() {
        let (contract, admin, user) = setup().await.unwrap();
        let base_asset = random_asset_id(20);
        let quote_asset = random_asset_id(21);

        let market = SparkMarketContract::deploy(
            base_asset,
            1,
            quote_asset,
            1,
            admin.wallet.clone(),
            9,
            0xFAFBFC,
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
            .with_account(&user.wallet)
            .await
            .unwrap()
            .unregister_market(contract_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "MarketNotRegistered")]
    async fn reverts_when_not_registered() {
        let (contract, admin, _) = setup().await.unwrap();
        let base_asset = random_asset_id(20);
        let quote_asset = random_asset_id(21);

        let market = SparkMarketContract::deploy(
            base_asset,
            1,
            quote_asset,
            1,
            admin.wallet.clone(),
            9,
            0xFAFBFC,
        )
        .await
        .unwrap();

        let contract_id: ContractId = market.contract_id().into();
        // Reverts
        contract
            .with_account(&admin.wallet)
            .await
            .unwrap()
            .unregister_market(contract_id)
            .await
            .unwrap();
    }
}
