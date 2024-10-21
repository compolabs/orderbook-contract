use crate::setup::{setup, Defaults};
use fuels::types::Identity;

mod success {

    use super::*;
    use spark_market_sdk::{OwnershipTransferred, State};

    #[tokio::test]
    async fn transfer_ownership() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

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

        let _ = contract
            .with_account(&user.wallet)
            .set_matcher_fee(1)
            .await?;

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

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
        let defaults = Defaults::default();
        let (contract, owner, user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let new_owner: Identity = user.wallet.address().into();
        contract.transfer_ownership(new_owner).await.unwrap();

        contract
            .with_account(&owner.wallet)
            .transfer_ownership(new_owner)
            .await
            .unwrap();
    }
}
