mod success {

    use crate::utils::{
        interface::{
            core::{deposit, open_order},
            info::{order, order_id},
        },
        setup::{setup, Defaults, OrderType},
    };
    use fuels::types::Bits256;

    #[tokio::test]
    async fn returns_none() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let order = order(&contract, Bits256([0u8; 32])).await.value;
        assert!(order.is_none());
    }

    #[tokio::test]
    async fn returns_order() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        deposit(&contract, 100, assets.base.asset_id).await;
        let id = open_order(&contract, 1, assets.base.asset_id, OrderType::Buy, 70000).await;

        let order = order(&contract, id.value).await.value.unwrap();
        let expected_id = order_id(
            &contract,
            order.amount,
            order.asset,
            order.order_type,
            order.owner,
            order.price,
        )
        .await;

        assert_eq!(id.value, expected_id.value);
    }
}
