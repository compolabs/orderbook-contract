use crate::setup::{random_asset_id, setup};
use fuels::types::ContractId;
use fuels::types::Identity;

mod success {

    use super::*;
    use spark_market_sdk::SparkMarketContract;
    use spark_registry_sdk::{OwnershipTransferred, State};

    #[tokio::test]
    async fn transfer_ownership() -> anyhow::Result<()> {
        let (contract, owner, user) = setup().await.unwrap();

        let new_owner: Identity = user.wallet.address().into();
        let response = contract.transfer_ownership(new_owner).await?;

        // Log should be emitted when fee is changed
        let log = response
            .decode_logs_with_type::<OwnershipTransferred>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            OwnershipTransferred {
                new_owner: new_owner,
                previous_owner: owner.wallet.address().into(),
            }
        );
        assert_eq!(contract.owner().await?.value, State::Initialized(new_owner));

        let base_asset = random_asset_id(20);
        let quote_asset = random_asset_id(21);

        let market = SparkMarketContract::deploy(
            base_asset,
            1,
            quote_asset,
            1,
            owner.wallet.clone(),
            9,
            0xFAFBFC,
        )
        .await?;

        let contract_id: ContractId = market.contract_id().into();
        contract
            .with_account(&user.wallet)
            .register_market(contract_id)
            .await?;

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_when_non_owner() {
        let (contract, _, user) = setup().await.unwrap();

        let new_owner: Identity = user.wallet.address().into();
        contract
            .with_account(&user.wallet)
            .transfer_ownership(new_owner)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_when_already_transfered() {
        let (contract, owner, user) = setup().await.unwrap();

        let new_owner: Identity = user.wallet.address().into();
        contract.transfer_ownership(new_owner).await.unwrap();

        contract
            .with_account(&owner.wallet)
            .transfer_ownership(new_owner)
            .await
            .unwrap();
    }
}
