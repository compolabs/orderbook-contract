use crate::setup::{setup, Defaults};

mod success {

    use super::*;
    use spark_market_sdk::SetMinOrderSizeEvent;

    #[tokio::test]
    async fn sets_min_sell_order_size() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, _, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let initial_size = 0;
        let new_size = 5;

        // Assert precondition of initial fee
        assert_eq!(contract.min_order_size().await?.value, initial_size);

        // Increase the fee to new_fee
        let response = contract.set_min_order_size(new_size).await?;

        // Log should be emitted when fee is changed
        let log = response
            .decode_logs_with_type::<SetMinOrderSizeEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(*event, SetMinOrderSizeEvent { size: new_size });

        // Check fee has changed from the initial fee
        assert_ne!(initial_size, new_size);
        assert_eq!(contract.min_order_size().await?.value, new_size);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
    async fn reverts_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _, user, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let new_size = 5;

        // Reverts
        contract
            .with_account(&user.wallet)
            .set_min_order_size(new_size)
            .await
            .unwrap();
    }
}
