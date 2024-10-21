use crate::setup::{setup, Defaults};

mod success {

    use super::*;
    use spark_market_sdk::SetMinOrderPriceEvent;

    #[tokio::test]
    async fn sets_min_order_price() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, _, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let initial_price = 0;
        let new_price = 5;

        // Assert precondition of initial fee
        assert_eq!(contract.min_order_price().await?.value, initial_price);

        // Increase the fee to new_fee
        let response = contract.set_min_order_price(new_price).await?;

        // Log should be emitted when fee is changed
        let log = response
            .decode_logs_with_type::<SetMinOrderPriceEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(*event, SetMinOrderPriceEvent { price: new_price });

        // Check fee has changed from the initial price
        assert_ne!(initial_price, new_price);
        assert_eq!(contract.min_order_price().await?.value, new_price);

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

        let new_price = 5;

        // Reverts
        contract
            .with_account(&user.wallet)
            .set_min_order_price(new_price)
            .await
            .unwrap();
    }
}
