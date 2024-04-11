use crate::utils::{
    interface::core::{deposit, open_order},
    setup::{setup, Defaults, OrderType},
};

mod success {

    use super::*;
    use crate::utils::{
        interface::info::{account, order, order_id, user_orders},
        setup::{create_account, OpenOrderEvent},
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

        let deposit_amount = 5;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let order_amount = 2;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;
        let expected_id = order_id(
            &contract,
            order_amount,
            asset,
            order_type.clone(),
            owner.identity(),
            price,
        )
        .await
        .value;

        let _ = deposit(&contract, deposit_amount, asset).await;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(order(&contract, expected_id).await.value.is_none());

        let response = open_order(&contract, order_amount, asset, order_type.clone(), price).await;
        let id = response.value;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(deposit_amount - order_amount, 0, order_amount, 0);
        let mut orders = user_orders(&contract, owner.identity()).await.value;
        let order = order(&contract, expected_id).await.value.unwrap();
        let stored_id = order_id(
            &contract,
            order.amount,
            order.asset,
            order.order_type,
            order.owner,
            order.price,
        )
        .await
        .value;

        let log = response.decode_logs_with_type::<OpenOrderEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            OpenOrderEvent {
                amount: order_amount,
                asset,
                asset_type: order.asset_type,
                order_type,
                order_id: expected_id,
                price,
                user: owner.identity(),
            }
        );

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
        assert_eq!(stored_id, expected_id);
    }

    #[ignore]
    #[tokio::test]
    async fn sell_quote() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 1000;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        let order_amount = 500;
        let asset = assets.quote.id;
        let order_type = OrderType::Sell;
        let price = 1;
        let expected_id = order_id(
            &contract,
            order_amount,
            asset,
            order_type.clone(),
            owner.identity(),
            price,
        )
        .await
        .value;

        let _ = deposit(&contract, deposit_amount, asset).await;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let orders = user_orders(&contract, owner.identity()).await.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(order(&contract, expected_id).await.value.is_none());

        let response = open_order(&contract, order_amount, asset, order_type.clone(), price).await;
        let id = response.value;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, deposit_amount - order_amount, 0, order_amount);
        let mut orders = user_orders(&contract, owner.identity()).await.value;
        let order = order(&contract, expected_id).await.value.unwrap();
        let stored_id = order_id(
            &contract,
            order.amount,
            order.asset,
            order.order_type,
            order.owner,
            order.price,
        )
        .await
        .value;

        let log = response.decode_logs_with_type::<OpenOrderEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            OpenOrderEvent {
                amount: order_amount,
                asset,
                asset_type: order.asset_type,
                order_type,
                order_id: expected_id,
                price,
                user: owner.identity(),
            }
        );

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
        assert_eq!(stored_id, expected_id);
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

        let response = open_order(
            &contract,
            order_amount,
            asset_to_buy,
            order_type.clone(),
            price,
        )
        .await;
        let id = response.value;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, 0, 0, deposit_amount);
        let mut orders = user_orders(&contract, owner.identity()).await.value;
        let order = order(&contract, expected_id).await.value.unwrap();
        let stored_id = order_id(
            &contract,
            order.amount,
            order.asset,
            order.order_type,
            order.owner,
            order.price,
        )
        .await
        .value;

        let log = response.decode_logs_with_type::<OpenOrderEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            OpenOrderEvent {
                amount: order_amount,
                asset: order.asset,
                asset_type: order.asset_type,
                order_type,
                order_id: expected_id,
                price,
                user: owner.identity(),
            }
        );

        dbg!(&user_account);
        dbg!(&expected_account);

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
        assert_eq!(stored_id, expected_id);
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

        let response = open_order(
            &contract,
            order_amount,
            asset_to_buy,
            order_type.clone(),
            price,
        )
        .await;
        let id = response.value;

        let user_account = account(&contract, owner.identity()).await.value.unwrap();
        let expected_account = create_account(0, 0, deposit_amount, 0);
        let mut orders = user_orders(&contract, owner.identity()).await.value;
        let order = order(&contract, expected_id).await.value.unwrap();
        let stored_id = order_id(
            &contract,
            order.amount,
            order.asset,
            order.order_type,
            order.owner,
            order.price,
        )
        .await
        .value;

        let log = response.decode_logs_with_type::<OpenOrderEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            OpenOrderEvent {
                amount: order_amount,
                asset: order.asset,
                asset_type: order.asset_type,
                order_type,
                order_id: expected_id,
                price,
                user: owner.identity(),
            }
        );

        dbg!(&user_account);
        dbg!(&expected_account);

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
        assert_eq!(stored_id, expected_id);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InvalidAsset")]
    async fn when_invalid_asset() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let order_amount = 10;
        let asset = assets.random.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        // Revert
        open_order(&contract, order_amount, asset, order_type, price).await;
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidUser")]
    async fn when_invalid_user() {
        let defaults = Defaults::default();
        let (contract, _owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let order_amount = 10;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        // Revert
        open_order(
            &contract.with_account(user.wallet).unwrap(),
            order_amount,
            asset,
            order_type,
            price,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_base_balance_to_sell() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 10;
        let order_amount = 100;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = deposit(&contract, deposit_amount, asset).await;

        // Revert
        open_order(&contract, order_amount, asset, order_type.clone(), price).await;
    }

    #[ignore]
    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_quote_balance_to_sell() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 10;
        let order_amount = 100;
        let asset = assets.quote.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = deposit(&contract, deposit_amount, asset).await;

        // Revert
        open_order(&contract, order_amount, asset, order_type.clone(), price).await;
    }

    #[ignore] // TODO: incomplete
    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_base_balance_to_buy() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 10;
        let order_amount = 100;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = deposit(&contract, deposit_amount, asset).await;

        // Revert
        open_order(&contract, order_amount, asset, order_type.clone(), price).await;
    }

    #[ignore] // TODO: incomplete
    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_quote_balance_to_buy() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 10;
        let order_amount = 100;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = deposit(&contract, deposit_amount, asset).await;

        // Revert
        open_order(&contract, order_amount, asset, order_type.clone(), price).await;
    }

    #[tokio::test]
    #[should_panic(expected = "DuplicateOrder")]
    async fn when_opening_duplicate_order() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        let deposit_amount = 100;
        let order_amount = 10;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = deposit(&contract, deposit_amount, asset).await;
        let _ = open_order(&contract, order_amount, asset, order_type.clone(), price).await;

        // Revert
        open_order(&contract, order_amount, asset, order_type, price).await;
    }
}
