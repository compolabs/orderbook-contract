use crate::setup::{setup, Defaults};
use spark_market_sdk::OrderType;

mod success {

    use super::*;
    use fuels::types::Bits256;

    #[tokio::test]
    async fn returns_order_id() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let id = contract
            .order_id(10, assets.base.id, OrderType::Buy, owner.identity(), 70000)
            .await?;
        let expected_id = Bits256([
            159, 12, 221, 182, 141, 102, 174, 95, 21, 142, 121, 195, 128, 124, 84, 242, 46, 187,
            60, 57, 72, 58, 86, 233, 236, 198, 174, 153, 212, 35, 88, 30,
        ]);
        assert_eq!(id.value, expected_id);

        Ok(())
    }

    #[tokio::test]
    async fn orders_create_different_ids() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let id1 = contract
            .order_id(10, assets.base.id, OrderType::Buy, owner.identity(), 70000)
            .await?;
        let id2 = contract
            .order_id(15, assets.base.id, OrderType::Buy, owner.identity(), 70000)
            .await?;

        let expected_id1 = Bits256([
            159, 12, 221, 182, 141, 102, 174, 95, 21, 142, 121, 195, 128, 124, 84, 242, 46, 187,
            60, 57, 72, 58, 86, 233, 236, 198, 174, 153, 212, 35, 88, 30,
        ]);
        let expected_id2 = Bits256([
            141, 48, 252, 3, 23, 245, 160, 247, 37, 135, 39, 90, 196, 71, 232, 49, 186, 168, 88,
            219, 171, 236, 105, 101, 179, 81, 223, 176, 72, 109, 125, 8,
        ]);

        assert_ne!(id1.value, id2.value);
        assert_eq!(id1.value, expected_id1);
        assert_eq!(id2.value, expected_id2);

        Ok(())
    }

    #[tokio::test]
    async fn accepts_base_asset() -> anyhow::Result<()> {
        // In this test we only care about the test not reverting with the correct asset
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let _ = contract
            .order_id(10, assets.base.id, OrderType::Buy, owner.identity(), 70000)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn accepts_quote_asset() -> anyhow::Result<()> {
        // In this test we only care about the test not reverting with the correct asset
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let _ = contract
            .order_id(10, assets.quote.id, OrderType::Buy, owner.identity(), 70000)
            .await?;

        Ok(())
    }
}

mod revert {

    use super::*;

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn reverts_upon_invalid_asset() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        contract
            .order_id(
                10,
                assets.random.id,
                OrderType::Buy,
                owner.identity(),
                70000,
            )
            .await
            .unwrap();
    }
}
