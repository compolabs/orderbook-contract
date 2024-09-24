use crate::setup::{setup, Defaults};
use rand::Rng;
use spark_market_sdk::{OrderType, ProtocolFee};

mod success {

    use super::*;
    use crate::setup::{convert, create_account};
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

        let expected_account = create_account(deposit_amount, 0, 0, 0);

        let response = contract.cancel_order(id).await?;
        let log = response
            .decode_logs_with_type::<CancelOrderEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            CancelOrderEvent {
                order_id: id,
                user: owner.identity(),
                balance: expected_account.clone(),
            }
        );

        let user_account = contract.account(owner.identity()).await?.value;
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 0);
        assert!(contract.order(id).await?.value.is_none());

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn fuzz_sell_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let (contract, owner, _user, _, _, assets) = setup(
                defaults.base_decimals,
                defaults.quote_decimals,
                defaults.price_decimals,
            )
            .await?;
            let provider = owner.wallet.try_provider()?;

            // Randomize deposit_amount 0.0001 to 100 BTC
            let deposit_amount = rng.gen_range(1..100_000) * 10_u64.pow(defaults.base_decimals - 3);
            let expected_account = create_account(deposit_amount, 0, 0, 0);

            let asset_to_sell = assets.base.id;
            let order_type = OrderType::Sell;

            // Randomize price 0.001 to 1m USD
            let price = rng.gen_range(1..1_000_000) * 10_u64.pow(defaults.price_decimals - 3);

            // Calculate max order amount based on deposit and price
            let max_order_amount = deposit_amount;

            let order_amount = if max_order_amount > 0 {
                rng.gen_range(1..=max_order_amount)
            } else {
                1
            };

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

            let _ = contract
                .deposit(deposit_amount, asset_to_sell)
                .await
                .is_ok();

            let user_account = contract.account(owner.identity()).await?.value;
            let orders = contract.user_orders(owner.identity()).await?.value;

            assert_eq!(user_account, expected_account);
            assert_eq!(orders, vec![]);
            assert!(contract.order(expected_id).await?.value.is_none());

            let id = contract
                .open_order(order_amount, order_type.clone(), price)
                .await?
                .value;
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

            let user_account = contract.account(owner.identity()).await?.value;

            // Calculate the remaining liquid base asset after locking the amount
            let locked_base = order_amount;
            let liquid_base = deposit_amount - locked_base;

            // Create the expected account after the order is locked
            let expected_account = create_account(liquid_base, 0, locked_base, 0);

            let mut orders = contract.user_orders(owner.identity()).await?.value;

            assert_eq!(user_account, expected_account);
            assert_eq!(orders.len(), 1);
            assert_eq!(orders.pop().unwrap(), id);
            assert_eq!(id, expected_id);

            let expected_account = create_account(deposit_amount, 0, 0, 0);

            let response = contract.cancel_order(id).await?;
            let log = response
                .decode_logs_with_type::<CancelOrderEvent>()
                .unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                CancelOrderEvent {
                    order_id: id,
                    user: owner.identity(),
                    balance: expected_account.clone(),
                }
            );

            let user_account = contract.account(owner.identity()).await?.value;
            let orders = contract.user_orders(owner.identity()).await?.value;
            assert_eq!(user_account, expected_account);
            assert_eq!(orders.len(), 0);
            assert!(contract.order(id).await?.value.is_none());
        }
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
        let deposit_amount = deposit_amount
            + deposit_amount * std::cmp::max(protocol_fee[0].maker_fee, protocol_fee[0].taker_fee)
                / 10_000
            + matcher_fee;
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

        let expected_account = create_account(0, deposit_amount, 0, 0);

        let response = contract.cancel_order(id).await?;
        let log = response
            .decode_logs_with_type::<CancelOrderEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            CancelOrderEvent {
                order_id: id,
                user: owner.identity(),
                balance: expected_account.clone(),
            }
        );

        let user_account = contract.account(owner.identity()).await?.value;
        let orders = contract.user_orders(owner.identity()).await?.value;
        assert_eq!(user_account, expected_account);
        assert_eq!(orders.len(), 0);
        assert!(contract.order(id).await?.value.is_none());

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn fuzz_buy_base() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let (contract, owner, _user, _, _, assets) = setup(
                defaults.base_decimals,
                defaults.quote_decimals,
                defaults.price_decimals,
            )
            .await?;
            let provider = owner.wallet.try_provider()?;

            // Randomize deposit_amount 1 to 1 million USDC
            let deposit_amount = rng.gen_range(1..1_000_000) * 10_u64.pow(6);
            let expected_account = create_account(0, deposit_amount, 0, 0);

            let asset_to_pay_wth = assets.quote.id;
            let order_type = OrderType::Buy;

            // Randomize price 0.001 to 1m USD
            let price = rng.gen_range(1..1_000_000) * 10_u64.pow(defaults.price_decimals - 3);

            // Scale and convert to u64
            let max_order_amount = deposit_amount * 10_u64.pow(6) / price * 10_u64.pow(5);

            let order_amount = if max_order_amount > 0 {
                rng.gen_range(1..=max_order_amount)
            } else {
                1
            };

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

            let _ = contract
                .deposit(deposit_amount, asset_to_pay_wth)
                .await
                .is_ok();

            let user_account = contract.account(owner.identity()).await?.value;
            let orders = contract.user_orders(owner.identity()).await?.value;

            assert_eq!(user_account, expected_account);
            assert_eq!(orders, vec![]);
            assert!(contract.order(expected_id).await?.value.is_none());

            let id = contract
                .open_order(order_amount, order_type.clone(), price)
                .await?
                .value;
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

            let user_account = contract.account(owner.identity()).await?.value;

            // Convert the order amount to quote using the convert function
            let locked_quote = convert(
                order_amount,
                defaults.base_decimals,
                price,
                defaults.price_decimals,
                defaults.quote_decimals,
                true,
            );

            // Calculate the remaining liquid quote after locking the amount
            let liquid_quote = (deposit_amount as u128).saturating_sub(locked_quote as u128); // Ensure no overflow

            // Cast to u64 before creating the expected account
            let expected_account = create_account(0, liquid_quote as u64, 0, locked_quote as u64);

            let mut orders = contract.user_orders(owner.identity()).await?.value;

            assert_eq!(user_account, expected_account);
            assert_eq!(orders.len(), 1);
            assert_eq!(orders.pop().unwrap(), id);
            assert_eq!(id, expected_id);

            let expected_account = create_account(0, deposit_amount, 0, 0);

            let response = contract.cancel_order(id).await?;
            let log = response
                .decode_logs_with_type::<CancelOrderEvent>()
                .unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                CancelOrderEvent {
                    order_id: id,
                    user: owner.identity(),
                    balance: expected_account.clone(),
                }
            );

            let user_account = contract.account(owner.identity()).await?.value;
            let orders = contract.user_orders(owner.identity()).await?.value;
            assert_eq!(user_account, expected_account);
            assert_eq!(orders.len(), 0);
            assert!(contract.order(id).await?.value.is_none());
        }
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
