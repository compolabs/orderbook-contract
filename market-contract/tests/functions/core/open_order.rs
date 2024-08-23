use crate::setup::{setup, Defaults};
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::{/*AssetType,*/ OpenOrderEvent, OrderChangeType, OrderType};

mod success {

    use super::*;
    use crate::setup::create_account;

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
        let price = 70_000_000_000_000_u64;

        let _ = contract.deposit(deposit_amount, asset).await?;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);

        let response = contract
            .open_order(
                order_amount,
                order_type.clone(),
                price,
            )
            .await?;
        let id = response.value;
        let expected_id = contract
            .order_id(
                order_type.clone(),
                owner.identity(),
                price,
                owner.wallet.try_provider()?.latest_block_height().await?,
                0,
            )
            .await?
            .value;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let expected_account = create_account(deposit_amount - order_amount, 0, order_amount, 0);
        let mut orders = contract.user_orders(owner.identity()).await?.value;
        let order = contract.order(expected_id).await?.value.unwrap();
        let block_height = owner.wallet.try_provider()?.latest_block_height().await?;
        let stored_id = contract
            .order_id(
                order.order_type,
                order.owner,
                order.price,
                order.block_height,
                0,
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

        let info = contract.order_change_info(stored_id).await?.value;
        assert_eq!(info.len(), 1);

        assert_eq!(info[0].change_type, OrderChangeType::OrderOpened);
        assert_eq!(info[0].block_height, block_height);
        assert_eq!(info[0].sender, owner.identity());
        assert_eq!(info[0].amount_before, 0);
        assert_eq!(info[0].amount_after, order_amount);

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

        let deposit_amount = 70000;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        let order_amount = 100;
        let asset_to_buy = assets.base.id;
        let asset_to_pay_wth = assets.quote.id;
        let order_type = OrderType::Buy;
        let price = 70000 * 10_u64.pow(defaults.price_decimals);

        let _ = contract.deposit(deposit_amount, asset_to_pay_wth).await;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);

        let response = contract
            .open_order(
                order_amount,
                order_type.clone(),
                price,
            )
            .await?;
        let id = response.value;
        let expected_id = contract
            .order_id(
                order_type.clone(),
                owner.identity(),
                price,
                owner.wallet.try_provider()?.latest_block_height().await?,
                0,
            )
            .await?
            .value;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let expected_account = create_account(0, 0, 0, deposit_amount);
        let mut orders = contract.user_orders(owner.identity()).await?.value;
        let order = contract.order(expected_id).await?.value.unwrap();
        let stored_id = contract
            .order_id(
                order.order_type,
                order.owner,
                order.price,
                order.block_height,
                0,
            )
            .await?
            .value;

        let log = response.decode_logs_with_type::<OpenOrderEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            OpenOrderEvent {
                amount: order_amount,
                asset: asset_to_buy,
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
    async fn sell_base_with_matcher_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_000_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let deposit_amount = 5;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let order_amount = 2;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70_000_000_000_000_u64;

        let _ = contract.deposit(deposit_amount, asset).await?;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);

        let balance = owner
            .wallet
            .get_asset_balance(&owner.wallet.provider().unwrap().base_asset_id())
            .await?;
        let response = contract
            .open_order(
                order_amount,
                order_type.clone(),
                price,
            )
            .await?;
        let new_balance = owner
            .wallet
            .get_asset_balance(&owner.wallet.provider().unwrap().base_asset_id())
            .await?;
        let gas_price = owner
            .wallet
            .provider()
            .unwrap()
            .latest_gas_price()
            .await?
            .gas_price;
        assert_eq!(balance - new_balance, matcher_fee as u64 + gas_price);

        let id = response.value;
        let block_height = owner.wallet.try_provider()?.latest_block_height().await?;
        let expected_id = contract
            .order_id(
                order_type.clone(),
                owner.identity(),
                price,
                block_height,
                0,
            )
            .await?
            .value;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let expected_account = create_account(deposit_amount - order_amount, 0, order_amount, 0);
        let mut orders = contract.user_orders(owner.identity()).await?.value;
        let order = contract.order(expected_id).await?.value.unwrap();
        let stored_id = contract
            .order_id(
                order.order_type,
                order.owner,
                order.price,
                order.block_height,
                0,
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

        let info = contract.order_change_info(stored_id).await?.value;
        assert_eq!(info.len(), 1);

        assert_eq!(info[0].change_type, OrderChangeType::OrderOpened);
        assert_eq!(info[0].block_height, block_height);
        assert_eq!(info[0].sender, owner.identity());
        assert_eq!(info[0].amount_before, 0);
        assert_eq!(info[0].amount_after, order_amount);

        Ok(())
    }

    #[tokio::test]
    async fn sell_base_with_protocol_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let deposit_amount = 10;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let order_amount = 10;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70_000_000_000_000_u64;

        let _ = contract.deposit(deposit_amount, asset).await?;

        let user_account = contract.account(owner.identity()).await?.value.unwrap();
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);

        let balance = owner
            .wallet
            .provider()
            .unwrap()
            .get_contract_asset_balance(contract.contract_id(), assets.fuel.id)
            .await?;
        let _ = contract
            .open_order(
                order_amount,
                order_type.clone(),
                price,
            )
            .await?;
        let new_balance = owner
            .wallet
            .provider()
            .unwrap()
            .get_contract_asset_balance(contract.contract_id(), assets.fuel.id)
            .await?;
        /*let protocol_fee_amount = contract
            .protocol_fee_amount(order_amount)
            .await?
            .value;
        assert_eq!(new_balance - balance, protocol_fee_amount);*/
        assert!(false);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_invalid_user() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let order_amount = 10;
        let order_type = OrderType::Sell;
        let price = 70_000_000_000_000_u64;

        // Revert
        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .open_order(order_amount, order_type, price)
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
        let price = 70_000_000_000_000_u64;

        let _ = contract.deposit(deposit_amount, asset).await.unwrap();

        // Revert
        contract
            .open_order(
                order_amount,
                order_type.clone(),
                price,
            )
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

        let deposit_amount = 1;
        let order_amount = 1500;
        let deposit_asset = assets.base.id;
        let order_type = OrderType::Buy;
        let price = 70_000_000_000_000_u64;

        let _ = contract
            .deposit(deposit_amount, deposit_asset)
            .await
            .unwrap();

        // Revert
        contract
            .open_order(order_amount, order_type, price)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "PriceTooSmall")]
    async fn when_price_too_small() {
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
        let order_type = OrderType::Sell;
        let price = 999999999;

        let _ = contract
            .deposit(deposit_amount, deposit_asset)
            .await
            .unwrap();

        // Revert
        contract
            .open_order(order_amount, order_type, price)
            .await
            .unwrap();
    }
}
