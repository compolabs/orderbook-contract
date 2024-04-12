use crate::setup::{setup, Defaults};
use spark_market_sdk::OrderType;

mod success {

    use super::*;
    use crate::setup::create_account;
    use spark_market_sdk::OpenOrderEvent;

    #[tokio::test]
    async fn sell_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 5;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let order_amount = 2;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;
        let expected_id = contract
            .order_id(
                order_amount,
                asset,
                order_type.clone(),
                owner.identity(),
                price,
            )
            .await?
            .value;

        let _ = contract.deposit(deposit_amount, asset).await?;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(contract.order(expected_id).await?.value.is_none());

        let response = contract
            .open_order(order_amount, asset, order_type.clone(), price)
            .await?;
        let id = response.value;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let expected_account = create_account(deposit_amount - order_amount, 0, order_amount, 0);
        let mut orders = contract.user_orders(owner.identity()).await?.value;
        let order = contract.order(expected_id).await?.value.unwrap();
        let stored_id = contract
            .order_id(
                order.amount,
                order.asset,
                order.order_type,
                order.owner,
                order.price,
            )
            .await?
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

        Ok(())
    }

    #[tokio::test]
    async fn sell_quote() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 1000;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        let order_amount = 500;
        let asset = assets.quote.id;
        let order_type = OrderType::Sell;
        let price = 1;
        let expected_id = contract
            .order_id(
                order_amount,
                asset,
                order_type.clone(),
                owner.identity(),
                price,
            )
            .await?
            .value;

        let _ = contract.deposit(deposit_amount, asset).await;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(contract.order(expected_id).await?.value.is_none());

        let response = contract
            .open_order(order_amount, asset, order_type.clone(), price)
            .await?;
        let id = response.value;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let expected_account = create_account(0, deposit_amount - order_amount, 0, order_amount);
        let mut orders = contract.user_orders(owner.identity()).await?.value;
        let order = contract.order(expected_id).await?.value.unwrap();
        let stored_id = contract
            .order_id(
                order.amount,
                order.asset,
                order.order_type,
                order.owner,
                order.price,
            )
            .await?
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

        Ok(())
    }

    #[tokio::test]
    async fn buy_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = assets.quote.to_contract_units(71000.0);
        let expected_account = create_account(0, deposit_amount, 0, 0);

        let order_amount = assets.base.to_contract_units(1.0);
        let asset_to_buy = assets.base.id;
        let asset_to_pay_wth = assets.quote.id;
        let order_type = OrderType::Buy;
        let price = assets.base.to_contract_units(70000.0); // TODO: this should use price formula / type instead of base
        let expected_id = contract
            .order_id(
                order_amount,
                asset_to_buy,
                order_type.clone(),
                owner.identity(),
                price,
            )
            .await?
            .value;

        let _ = contract.deposit(deposit_amount, asset_to_pay_wth).await;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(contract.order(expected_id).await?.value.is_none());

        let response = contract
            .open_order(order_amount, asset_to_buy, order_type.clone(), price)
            .await?;
        let id = response.value;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let expected_account = create_account(
            0,
            assets.quote.to_contract_units(1000.0),
            0,
            deposit_amount - assets.quote.to_contract_units(1000.0),
        );
        let mut orders = contract.user_orders(owner.identity()).await?.value;
        let order = contract.order(expected_id).await?.value.unwrap();
        let stored_id = contract
            .order_id(
                order.amount,
                order.asset,
                order.order_type,
                order.owner,
                order.price,
            )
            .await?
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

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
        assert_eq!(stored_id, expected_id);

        Ok(())
    }

    #[tokio::test]
    async fn buy_quote() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = assets.base.to_contract_units(1.1);
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let order_amount = assets.quote.to_contract_units(70000.0);
        let asset_to_buy = assets.quote.id;
        let asset_to_pay_wth = assets.base.id;
        let order_type = OrderType::Buy;
        let price = assets.base.to_contract_units(70000.0); // TODO: this should use price formula / type instead of base
        let expected_id = contract
            .order_id(
                order_amount,
                asset_to_buy,
                order_type.clone(),
                owner.identity(),
                price,
            )
            .await?
            .value;

        let _ = contract.deposit(deposit_amount, asset_to_pay_wth).await?;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(contract.order(expected_id).await?.value.is_none());

        let response = contract
            .open_order(order_amount, asset_to_buy, order_type.clone(), price)
            .await?;
        let id = response.value;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let expected_account = create_account(
            assets.base.to_contract_units(0.1),
            0,
            deposit_amount - assets.base.to_contract_units(0.1),
            0,
        );
        let mut orders = contract.user_orders(owner.identity()).await?.value;
        let order = contract.order(expected_id).await?.value.unwrap();
        let stored_id = contract
            .order_id(
                order.amount,
                order.asset,
                order.order_type,
                order.owner,
                order.price,
            )
            .await?
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

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
        assert_eq!(stored_id, expected_id);

        Ok(())
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
        .await
        .unwrap();

        let order_amount = 10;
        let asset = assets.random.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        // Revert
        contract
            .open_order(order_amount, asset, order_type, price)
            .await
            .unwrap();
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
        .await
        .unwrap();

        let order_amount = 10;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        // Revert
        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .open_order(order_amount, asset, order_type, price)
            .await
            .unwrap();
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
        .await
        .unwrap();

        let deposit_amount = 10;
        let order_amount = 100;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = contract.deposit(deposit_amount, asset).await.unwrap();

        // Revert
        contract
            .open_order(order_amount, asset, order_type.clone(), price)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_quote_balance_to_sell() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 10;
        let order_amount = 100;
        let asset = assets.quote.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = contract.deposit(deposit_amount, asset).await.unwrap();

        // Revert
        contract
            .open_order(order_amount, asset, order_type, price)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_base_balance_to_buy() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 10;
        let order_amount = 100;
        let deposit_asset = assets.base.id;
        let buy_asset = assets.quote.id;
        let order_type = OrderType::Buy;
        let price = 70000;

        let _ = contract
            .deposit(deposit_amount, deposit_asset)
            .await
            .unwrap();

        // Revert
        contract
            .open_order(order_amount, buy_asset, order_type, price)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_quote_balance_to_buy() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 10;
        let order_amount = 100;
        let deposit_asset = assets.quote.id;
        let buy_asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = contract
            .deposit(deposit_amount, deposit_asset)
            .await
            .unwrap();

        // Revert
        contract
            .open_order(order_amount, buy_asset, order_type, price)
            .await
            .unwrap();
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
        .await
        .unwrap();

        let deposit_amount = 100;
        let order_amount = 10;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70000;

        let _ = contract.deposit(deposit_amount, asset).await.unwrap();
        let _ = contract
            .open_order(order_amount, asset, order_type.clone(), price)
            .await
            .unwrap();

        // Revert
        contract
            .open_order(order_amount, asset, order_type, price)
            .await
            .unwrap();
    }
}
