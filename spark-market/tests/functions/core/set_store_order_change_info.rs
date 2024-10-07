use crate::setup::{setup, Defaults};

mod success {

    use super::*;
    use spark_market_sdk::SetStoreOrderChangeInfoEvent;

    #[tokio::test]
    async fn set_store_order_change_info() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        // Assert precondition of initial fee
        assert_eq!(contract.store_order_change_info().await?.value, true);

        // Increase the fee to new_fee
        let response = contract.set_store_order_change_info(false).await?;

        // Log should be emitted when fee is changed
        let log = response
            .decode_logs_with_type::<SetStoreOrderChangeInfoEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(*event, SetStoreOrderChangeInfoEvent { store: false });

        // Check fee has changed from the initial fee
        assert_eq!(contract.store_order_change_info().await?.value, false);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        // Reverts
        contract
            .with_account(&user.wallet)
            .set_store_order_change_info(true)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidValueSame")]
    async fn reverts_when_same_value() {
        let defaults = Defaults::default();
        let (contract, _owner, _, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        // Reverts
        contract.set_store_order_change_info(true).await.unwrap();
    }
}
