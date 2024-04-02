mod success {

    use crate::utils::{
        interface::{
            core::{deposit, open_order},
            info::user_orders,
        },
        setup::{setup, Defaults, OrderType},
    };

    #[tokio::test]
    async fn returns_zero_orders() {
        let defaults = Defaults::default();
        let (contract, owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(orders, vec![]);
    }

    #[tokio::test]
    async fn returns_orders() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        deposit(&contract, 100, assets.base.id).await;
        let id1 = open_order(&contract, 1, assets.base.id, OrderType::Buy, 75000).await;
        let id2 = open_order(&contract, 2, assets.base.id, OrderType::Buy, 70000).await;

        let mut orders = user_orders(&contract, owner.identity()).await.value;

        assert_eq!(2, orders.len());
        assert_eq!(id2.value, orders.pop().unwrap());
        assert_eq!(id1.value, orders.pop().unwrap());
    }
}
