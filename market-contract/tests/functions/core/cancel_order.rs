use crate::setup::{setup, Defaults};
use spark_market_sdk::{OrderType, ProtocolFee};

mod success {

    use super::*;
    use crate::setup::create_account;
    use fuels::accounts::ViewOnlyAccount;
    use spark_market_sdk::CancelOrderEvent;

    #[tokio::test]
    async fn sell_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let matcher_fee = 100_000_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let protocol_fee = vec![ProtocolFee {
            maker_fee: 10,
            taker_fee: 15,
            volume_threshold: 0,
        }];
        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;

        let deposit_amount = 100;
        let order_amount = 1;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70_000_000_000_000_u64;

        let _ = contract.deposit(deposit_amount, asset).await?;
        let id = contract
            .open_order(order_amount, /*AssetType::Base,*/ order_type, price)
            .await?
            .value;

        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(deposit_amount - order_amount, 0, order_amount, 0);
        let mut orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert!(contract.order(id).await?.value.is_some());

        let response = contract.cancel_order(id).await?;
        let log = response
            .decode_logs_with_type::<CancelOrderEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(*event, CancelOrderEvent { order_id: id });

        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(deposit_amount, 0, 0, 0);
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 0);
        assert!(contract.order(id).await?.value.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn buy_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let provider = owner.wallet.try_provider()?;

        let matcher_fee = 100_u64;
        let _ = contract.set_matcher_fee(matcher_fee).await?;

        let protocol_fee = vec![ProtocolFee {
            maker_fee: 10,
            taker_fee: 15,
            volume_threshold: 0,
        }];
        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;

        let deposit_amount = 70000;
        let deposit_amount = deposit_amount + deposit_amount
            * std::cmp::max(protocol_fee[0].maker_fee, protocol_fee[0].taker_fee)
            / 10_000 + matcher_fee;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        let order_amount = 100;
        let asset_to_pay_wth = assets.quote.id;
        let order_type = OrderType::Buy;
        let price = 70000 * 10_u64.pow(defaults.price_decimals);
        let expected_id = contract
            .order_id(
                order_type.clone(),
                owner.identity(),
                price,
                provider.latest_block_height().await?,
                0,
            )
            .await?
            .value;

        let _ = contract.deposit(deposit_amount, asset_to_pay_wth).await;

        let user_account = contract.account(owner.identity()).await?.value;
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders, vec![]);
        assert!(contract.order(expected_id).await?.value.is_none());

        let id = contract
            .open_order(order_amount, order_type.clone(), price)
            .await?
            .value;

        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(0, 0, 0, deposit_amount);
        let mut orders = contract.user_orders(owner.identity()).await?.value;

        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 1);
        assert_eq!(orders.pop().unwrap(), id);
        assert_eq!(id, expected_id);

        let response = contract.cancel_order(id).await?;
        let log = response
            .decode_logs_with_type::<CancelOrderEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(*event, CancelOrderEvent { order_id: id });

        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(0, deposit_amount, 0, 0);
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 0);
        assert!(contract.order(id).await?.value.is_none());

        Ok(())
    }
}

mod revert {

    use super::*;
    use fuels::types::Bits256;

    #[tokio::test]
    #[should_panic(expected = "OrderNotFound")]
    async fn when_order_does_not_exist() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        // Revert
        contract.cancel_order(Bits256([0u8; 32])).await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn when_user_is_not_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let deposit_amount = 100;
        let order_amount = 1;
        let asset = assets.base.id;
        let order_type = OrderType::Sell;
        let price = 70_000_000_000_000_u64;

        let _ = contract.deposit(deposit_amount, asset).await.unwrap();
        let id = contract
            .open_order(order_amount, /*AssetType::Base,*/ order_type, price)
            .await
            .unwrap()
            .value;

        // Revert
        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .cancel_order(id)
            .await
            .unwrap();
    }
}
