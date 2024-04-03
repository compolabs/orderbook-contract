use crate::utils::{
    interface::core::{cancel_order, deposit, open_order},
    setup::{setup, Defaults, OrderType},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{account, order, order_id, user_orders},
        setup::{create_account, CancelOrderEvent},
    };

    #[tokio::test]
    async fn sell_base() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;
        let order_amount = 1;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = deposit(&contract, deposit_amount, asset).await;
        let id = open_order(&contract, order_amount, asset, order_type, price)
            .await
            .value;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(deposit_amount - order_amount, 0, order_amount, 0);
        let mut orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert!(order(&contract, id).await.value.is_some());

        let response = cancel_order(&contract, id).await;
        let log = response
            .decode_logs_with_type::<CancelOrderEvent>()
            .unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(*event, CancelOrderEvent { order_id: id });

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(deposit_amount, 0, 0, 0);
        let orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 0);
        assert!(order(&contract, id).await.value.is_none());
    }

    #[tokio::test]
    async fn sell_quote() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 70000;
        let order_amount = 50000;
        let asset = assets.quote.id;
        let order_type = OrderType::Sell;
        let price = 50000;

        let _ = deposit(&contract, deposit_amount, asset).await;
        let id = open_order(&contract, order_amount, asset, order_type, price)
            .await
            .value;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, deposit_amount - order_amount, 0, order_amount);
        let mut orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert!(order(&contract, id).await.value.is_some());

        let response = cancel_order(&contract, id).await;
        let log = response
            .decode_logs_with_type::<CancelOrderEvent>()
            .unwrap();
        let event = log.get(0).unwrap();
        assert_eq!(*event, CancelOrderEvent { order_id: id });

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, deposit_amount, 0, 0);
        let orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 0);
        assert!(order(&contract, id).await.value.is_none());
    }

    #[ignore]
    #[tokio::test]
    async fn buy_base() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 70000;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        let order_amount = 1;
        let asset_to_buy = assets.base.id;
        let asset_to_pay_wth = assets.quote.id;
        let order_type = OrderType::Buy;
        let price = 70000;
        let expected_id = order_id(
            &contract,
            order_amount,
            asset_to_buy,
            order_type.clone(),
            owner.identity(),
            price,
        )
        .await
        .value;

        let _ = deposit(&contract, deposit_amount, asset_to_pay_wth).await;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(order(&contract, expected_id).await.value.is_none());

        let id = open_order(&contract, order_amount, asset_to_buy, order_type, price)
            .await
            .value;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, 0, 0, deposit_amount);
        let mut orders = user_orders(&contract, owner.identity()).await.value;

        dbg!(&user_account);
        dbg!(&expected_account);

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
    }

    #[ignore]
    #[tokio::test]
    async fn buy_quote() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 1;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let order_amount = 70000;
        let asset_to_buy = assets.quote.id;
        let asset_to_pay_wth = assets.base.id;
        let order_type = OrderType::Buy;
        let price = 70000;
        let expected_id = order_id(
            &contract,
            order_amount,
            asset_to_buy,
            order_type.clone(),
            owner.identity(),
            price,
        )
        .await
        .value;

        let _ = deposit(&contract, deposit_amount, asset_to_pay_wth).await;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(order(&contract, expected_id).await.value.is_none());

        let id = open_order(&contract, order_amount, asset_to_buy, order_type, price)
            .await
            .value;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, 0, deposit_amount, 0);
        let mut orders = user_orders(&contract, owner.identity()).await.value;

        dbg!(&user_account);
        dbg!(&expected_account);

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
    }
}

mod revert {

    use super::*;
    use fuels::types::Bits256;

    #[tokio::test]
    #[should_panic(expected = "NoOrdersFound")]
    async fn when_order_does_not_exist() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        // Revert
        cancel_order(&contract, Bits256([0u8; 32])).await;
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_user_is_not_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;
        let order_amount = 1;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = deposit(&contract, deposit_amount, asset).await;
        let id = open_order(&contract, order_amount, asset, order_type, price)
            .await
            .value;

        // Revert
        cancel_order(&contract.with_account(user.wallet), id).await;
    }
}
