use crate::setup::{setup, Defaults};
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::{OpenOrderEvent, OrderChangeType, OrderType, ProtocolFee};

mod success {

    use spark_market_sdk::AssetType;

    use super::*;
    use crate::setup::create_account;

    #[tokio::test]
    async fn sell_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
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

        let user_account = contract.account(owner.identity()).await?.value;
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);

        let response = contract
            .open_order(order_amount, order_type.clone(), price)
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

        let user_account = contract.account(owner.identity()).await?.value;
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
                balance: expected_account.clone(),
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
    async fn buy_base_with_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let deposit_amount = 70000 + matcher_fee;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        let order_amount = 100;
        let asset_to_buy = assets.base.id;
        let asset_to_pay_wth = assets.quote.id;
        let order_type = OrderType::Buy;
        let price = 70000 * 10_u64.pow(defaults.price_decimals);

        let _ = contract.deposit(deposit_amount, asset_to_pay_wth).await;

        let user_account = contract.account(owner.identity()).await?.value;
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);

        let response = contract
            .open_order(order_amount, order_type.clone(), price)
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

        let user_account = contract.account(owner.identity()).await?.value;
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
                balance: expected_account.clone(),
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
    async fn sell_base_with_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let protocol_fee = vec![ProtocolFee {
            maker_fee: 10,
            taker_fee: 15,
            volume_threshold: 0,
        }];

        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;

        let deposit_amount = 5;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let order_amount = 2;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70_000_000_000_000_u64;

        let balance = _user.wallet.get_asset_balance(&assets.base.id).await?;
        let _ = contract
            .with_account(&_user.wallet)
            .deposit(deposit_amount, asset)
            .await?;
        let new_balance = _user.wallet.get_asset_balance(&assets.base.id).await?;
        assert_eq!(balance - new_balance, deposit_amount);

        let user_account = contract.account(_user.identity()).await?.value;
        let orders = contract.user_orders(_user.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);

        let response = contract
            .with_account(&_user.wallet)
            .open_order(order_amount, order_type.clone(), price)
            .await?;

        let id = response.value;
        let block_height = _user.wallet.try_provider()?.latest_block_height().await?;
        let expected_id = contract
            .order_id(order_type.clone(), _user.identity(), price, block_height, 0)
            .await?
            .value;

        let expected_account = create_account(deposit_amount - order_amount, 0, order_amount, 0);
        let mut orders = contract.user_orders(_user.identity()).await?.value;
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
                user: _user.identity(),
                balance: expected_account.clone(),
            }
        );

        let user_account = contract.account(_user.identity()).await?.value;

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);
        assert_eq!(stored_id, expected_id);

        let info = contract.order_change_info(stored_id).await?.value;
        assert_eq!(info.len(), 1);

        assert_eq!(info[0].change_type, OrderChangeType::OrderOpened);
        assert_eq!(info[0].block_height, block_height);
        assert_eq!(info[0].sender, _user.identity());
        assert_eq!(info[0].amount_before, 0);
        assert_eq!(info[0].amount_after, order_amount);

        let order = contract.order(id).await?.value.unwrap();
        assert_eq!(order.amount, order_amount);
        assert_eq!(order.asset_type, AssetType::Base);
        assert_eq!(order.order_type, OrderType::Sell);
        assert_eq!(order.owner, _user.identity());
        assert_eq!(order.price, price);
        assert_eq!(order.block_height, block_height);
        assert_eq!(order.order_height, 0);
        assert_eq!(order.matcher_fee, matcher_fee);
        assert_eq!(order.protocol_maker_fee, protocol_fee[0].maker_fee);
        assert_eq!(order.protocol_taker_fee, protocol_fee[0].taker_fee);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_invalid_user() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, _) = setup(
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
            .open_order(order_amount, order_type, price)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_base_balance_to_sell() {
        let defaults = Defaults::default();
        let (contract, _owner, _, _, _, assets) = setup(
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
            .open_order(order_amount, order_type.clone(), price)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_insufficient_base_balance_to_buy() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 70000;
        let order_amount = 100;
        let asset_to_pay_wth = assets.quote.id;
        let order_type = OrderType::Buy;
        let price = 70000 * 10_u64.pow(defaults.price_decimals);

        let _ = contract.deposit(deposit_amount, asset_to_pay_wth).await;

        let matcher_fee = 100_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await.unwrap();

        let protocol_fee = vec![ProtocolFee {
            maker_fee: 10,
            taker_fee: 15,
            volume_threshold: 0,
        }];

        let _ = contract
            .set_protocol_fee(protocol_fee.clone())
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
        let (contract, _owner, _user, _, _, assets) = setup(
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

        // minimum price is 0.000000001 i.e. 1 / 1e9
        let price = 0;

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
