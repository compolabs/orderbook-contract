use crate::setup::{setup, Defaults};
use fuels::types::Bits256;
use spark_market_sdk::OrderType;

mod success {

    // TODO: open_order does not handle math correctly so changing values will break tests. Need math help
    use super::*;
    use crate::setup::{
        base_to_quote_amount, calc_amount, create_account, quote_to_base_amount, Asset, User,
    };
    use spark_market_sdk::MarketContract;

    // Utility fn to reduce setup in each test when opening orders
    async fn open_order(
        contract: &MarketContract,
        deposit_asset: &Asset,
        deposit_amount: f64,
        order_type: OrderType,
        order_amount: f64,
        order_asset: &Asset,
        price: f64,
        user: &User,
        deposited_base: bool,
        expected_order_count: u64,
        defaults: &Defaults,
    ) -> anyhow::Result<Bits256> {
        let deposit = deposit_asset.to_contract_units(deposit_amount);
        let _ = contract.deposit(deposit, deposit_asset.id).await?;

        let order_amount = order_asset.to_contract_units(order_amount);
        let price = order_asset.to_contract_units(price); // TODO: this should use price formula / type instead of asset

        let order_id = contract
            .open_order(order_amount, order_asset.id, order_type, price)
            .await?
            .value;

        let order_count = contract.user_orders(user.identity()).await?.value.len() as u64;
        let account = contract.account(user.identity()).await?.value.unwrap();
        let expected_account = match deposited_base {
            true => create_account(0, 0, order_amount, 0),
            false => {
                let order_amount_in_quote = base_to_quote_amount(
                    order_amount,
                    defaults.base_decimals,
                    price,
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let remaining_amount = deposit - order_amount_in_quote;
                create_account(0, remaining_amount, 0, order_amount_in_quote)
            }
        };
        assert_eq!(account, expected_account);
        assert_eq!(order_count, expected_order_count);

        Ok(order_id)
    }

    // Batch is a vector with a single order to fulfill
    mod batch_of_one {
        use super::*;

        // Alice is trying to sell her base asset to the batch of a single buyer for their quote asset
        mod alice_sell_base {
            use super::*;

            #[tokio::test]
            async fn bob_buy_greater_price_greater_amount() -> anyhow::Result<()> {
                // Bob should only get the exact amount that alice is selling
                let defaults = Defaults::default();
                let (alice_contract, owner, user, assets) = setup(
                    defaults.base_decimals,
                    defaults.quote_decimals,
                    defaults.price_decimals,
                )
                .await?;
                let bob_contract = alice_contract.with_account(&user.wallet).await?;

                let alice_order_id = open_order(
                    &alice_contract,
                    &assets.base,
                    1.0,
                    OrderType::Sell,
                    1.0,
                    &assets.base,
                    70000.0,
                    &owner,
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    70000.0 * 1.5,
                    OrderType::Buy,
                    1.25,
                    &assets.base,
                    71000.0,
                    &user,
                    false,
                    1,
                    &defaults,
                )
                .await?;

                // TODO: assert log events
                let _response = alice_contract
                    .batch_fulfill(alice_order_id, vec![bob_order_id])
                    .await?;

                let alice_orders = alice_contract
                    .user_orders(owner.identity())
                    .await?
                    .value
                    .len() as u64;
                let alice_account = alice_contract
                    .account(owner.identity())
                    .await?
                    .value
                    .unwrap();
                let alice_expected_account =
                    create_account(0, assets.quote.to_contract_units(70000.0), 0, 0);
                assert_eq!(alice_account, alice_expected_account);
                assert_eq!(alice_orders, 0);

                let bob_orders = alice_contract
                    .user_orders(user.identity())
                    .await?
                    .value
                    .len() as u64;

                let deposit = assets.quote.to_contract_units(70000.0 * 1.5);
                let order_amount_in_quote = base_to_quote_amount(
                    assets.base.to_contract_units(1.25),
                    defaults.base_decimals,
                    assets.base.to_contract_units(71000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let remaining_deposit = deposit - order_amount_in_quote;
                let alice_amount_in_quote = base_to_quote_amount(
                    assets.base.to_contract_units(1.0),
                    defaults.base_decimals,
                    assets.base.to_contract_units(70000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let remaining_amount = order_amount_in_quote - alice_amount_in_quote;

                // deposit is liquid quote
                // lock some in quote for order (deposit - order)
                // fulfil order which is less than locked amount (deposit - order) - alice in quote
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(1.0),
                    remaining_deposit,
                    0,
                    remaining_amount,
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 1);
                Ok(())
            }

            #[ignore]
            #[tokio::test]
            async fn bob_buy_greater_price_same_amount() -> anyhow::Result<()> {
                // Bob should only get the exact amount that alice is selling
                Ok(())
            }

            #[ignore]
            #[tokio::test]
            async fn bob_buy_greater_price_smaller_amount() -> anyhow::Result<()> {
                // Bob should get more than his order because he is paying more than the price that alice set
                let defaults = Defaults::default();
                let (alice_contract, owner, user, assets) = setup(
                    defaults.base_decimals,
                    defaults.quote_decimals,
                    defaults.price_decimals,
                )
                .await?;
                let bob_contract = alice_contract.with_account(&user.wallet).await?;

                let alice_order_id = open_order(
                    &alice_contract,
                    &assets.base,
                    1.0,
                    OrderType::Sell,
                    1.0,
                    &assets.base,
                    70000.0,
                    &owner,
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    70000.0 * 1.5,
                    OrderType::Buy,
                    0.5,
                    &assets.base,
                    71000.0,
                    &user,
                    false,
                    1,
                    &defaults,
                )
                .await?;

                let bought_amount_in_base = assets
                    .base
                    .to_contract_units(calc_amount(0.5, 71000.0, 70000.0));
                dbg!(bought_amount_in_base);
                let bought_amount_in_quote = base_to_quote_amount(
                    bought_amount_in_base,
                    defaults.base_decimals,
                    assets.base.to_contract_units(71000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let alice_remaining_locked_base =
                    assets.base.to_contract_units(1.0) - bought_amount_in_base;

                // TODO: assert log events
                let _response = alice_contract
                    .batch_fulfill(alice_order_id, vec![bob_order_id])
                    .await?;

                let alice_orders = alice_contract
                    .user_orders(owner.identity())
                    .await?
                    .value
                    .len() as u64;
                let alice_account = alice_contract
                    .account(owner.identity())
                    .await?
                    .value
                    .unwrap();
                let alice_expected_account =
                    create_account(0, bought_amount_in_quote, alice_remaining_locked_base, 0);
                assert_eq!(alice_account, alice_expected_account);
                assert_eq!(alice_orders, 0);

                let bob_orders = alice_contract
                    .user_orders(user.identity())
                    .await?
                    .value
                    .len() as u64;

                let deposit = assets.quote.to_contract_units(70000.0 * 1.5);
                let order_amount_in_quote = base_to_quote_amount(
                    assets.base.to_contract_units(0.5),
                    defaults.base_decimals,
                    assets.base.to_contract_units(71000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let remaining_deposit = deposit - order_amount_in_quote;
                let alice_amount_in_quote = base_to_quote_amount(
                    assets.base.to_contract_units(1.0),
                    defaults.base_decimals,
                    assets.base.to_contract_units(70000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let remaining_amount = order_amount_in_quote - alice_amount_in_quote;

                // deposit is liquid quote
                // lock some in quote for order (deposit - order)
                // fulfil order which is less than locked amount (deposit - order) - alice in quote
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(1.0),
                    remaining_deposit,
                    0,
                    remaining_amount,
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 1);
                Ok(())
            }

            #[tokio::test]
            async fn bob_buy_same_price_same_amount() -> anyhow::Result<()> {
                let defaults = Defaults::default();
                let (alice_contract, owner, user, assets) = setup(
                    defaults.base_decimals,
                    defaults.quote_decimals,
                    defaults.price_decimals,
                )
                .await?;
                let bob_contract = alice_contract.with_account(&user.wallet).await?;

                let alice_order_id = open_order(
                    &alice_contract,
                    &assets.base,
                    1.0,
                    OrderType::Sell,
                    1.0,
                    &assets.base,
                    70000.0,
                    &owner,
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    70000.0,
                    OrderType::Buy,
                    1.0,
                    &assets.base,
                    70000.0,
                    &user,
                    false,
                    1,
                    &defaults,
                )
                .await?;

                // TODO: assert log events
                let _response = alice_contract
                    .batch_fulfill(alice_order_id, vec![bob_order_id])
                    .await?;

                let alice_orders = alice_contract
                    .user_orders(owner.identity())
                    .await?
                    .value
                    .len() as u64;
                let alice_account = alice_contract
                    .account(owner.identity())
                    .await?
                    .value
                    .unwrap();
                let alice_expected_account =
                    create_account(0, assets.quote.to_contract_units(70000.0), 0, 0);
                assert_eq!(alice_account, alice_expected_account);
                assert_eq!(alice_orders, 0);

                let bob_orders = alice_contract
                    .user_orders(user.identity())
                    .await?
                    .value
                    .len() as u64;

                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account =
                    create_account(assets.base.to_contract_units(1.0), 0, 0, 0);
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
                Ok(())
            }

            #[ignore]
            #[tokio::test]
            async fn bob_buy_same_price_greater_amount() -> anyhow::Result<()> {
                // Bob should only get the exact amount that alice is selling
                Ok(())
            }

            #[ignore]
            #[tokio::test]
            async fn bob_buy_same_price_smaller_amount() -> anyhow::Result<()> {
                // Bob should get the exact amount that he wants to buy
                Ok(())
            }
        }

        // Alice is trying to buy the base asset from the batch of a single seller with her quote asset
        mod alice_buy_base {

            use super::*;

            #[ignore]
            #[tokio::test]
            async fn bob_sell_smaller_price_greater_amount() -> anyhow::Result<()> {
                Ok(())
            }

            #[ignore]
            #[tokio::test]
            async fn bob_sell_smaller_price_same_amount() -> anyhow::Result<()> {
                Ok(())
            }

            #[ignore]
            #[tokio::test]
            async fn bob_sell_smaller_price_smaller_amount() -> anyhow::Result<()> {
                let defaults = Defaults::default();
                let (alice_contract, owner, user, assets) = setup(
                    defaults.base_decimals,
                    defaults.quote_decimals,
                    defaults.price_decimals,
                )
                .await?;
                let bob_contract = alice_contract.with_account(&user.wallet).await?;

                let alice_order_id = open_order(
                    &alice_contract,
                    &assets.quote,
                    70000.0 * 1.5,
                    OrderType::Buy,
                    1.25,
                    &assets.base,
                    71000.0,
                    &owner,
                    false,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.base,
                    1.0,
                    OrderType::Sell,
                    1.0,
                    &assets.base,
                    70000.0,
                    &user,
                    true,
                    1,
                    &defaults,
                )
                .await?;

                // TODO: assert log events
                let _response = alice_contract
                    .batch_fulfill(alice_order_id, vec![bob_order_id])
                    .await?;

                let alice_orders = alice_contract
                    .user_orders(owner.identity())
                    .await?
                    .value
                    .len() as u64;
                let alice_account = alice_contract
                    .account(owner.identity())
                    .await?
                    .value
                    .unwrap();
                let alice_expected_account =
                    create_account(0, assets.quote.to_contract_units(70000.0), 0, 0);
                assert_eq!(alice_account, alice_expected_account);
                assert_eq!(alice_orders, 0);

                let bob_orders = alice_contract
                    .user_orders(user.identity())
                    .await?
                    .value
                    .len() as u64;

                let deposit = assets.quote.to_contract_units(70000.0 * 1.5);
                let order_amount_in_quote = base_to_quote_amount(
                    assets.base.to_contract_units(1.25),
                    defaults.base_decimals,
                    assets.base.to_contract_units(71000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let remaining_deposit = deposit - order_amount_in_quote;
                let alice_amount_in_quote = base_to_quote_amount(
                    assets.base.to_contract_units(1.0),
                    defaults.base_decimals,
                    assets.base.to_contract_units(70000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let remaining_amount = order_amount_in_quote - alice_amount_in_quote;

                // deposit is liquid quote
                // lock some in quote for order (deposit - order)
                // fulfil order which is less than locked amount (deposit - order) - alice in quote
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(1.0),
                    remaining_deposit,
                    0,
                    remaining_amount,
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 1);
                Ok(())
            }

            #[tokio::test]
            async fn bob_sell_same_price_same_amount() -> anyhow::Result<()> {
                let defaults = Defaults::default();
                let (alice_contract, owner, user, assets) = setup(
                    defaults.base_decimals,
                    defaults.quote_decimals,
                    defaults.price_decimals,
                )
                .await?;
                let bob_contract = alice_contract.with_account(&user.wallet).await?;

                let alice_order_id = open_order(
                    &alice_contract,
                    &assets.quote,
                    70000.0,
                    OrderType::Buy,
                    1.0,
                    &assets.base,
                    70000.0,
                    &owner,
                    false,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.base,
                    1.0,
                    OrderType::Sell,
                    1.0,
                    &assets.base,
                    70000.0,
                    &user,
                    true,
                    1,
                    &defaults,
                )
                .await?;

                // TODO: assert log events
                let _response = alice_contract
                    .batch_fulfill(alice_order_id, vec![bob_order_id])
                    .await?;

                let alice_orders = alice_contract
                    .user_orders(owner.identity())
                    .await?
                    .value
                    .len() as u64;
                let alice_account = alice_contract
                    .account(owner.identity())
                    .await?
                    .value
                    .unwrap();
                let alice_expected_account =
                    create_account(assets.base.to_contract_units(1.0), 0, 0, 0);
                assert_eq!(alice_account, alice_expected_account);
                assert_eq!(alice_orders, 0);

                let bob_orders = alice_contract
                    .user_orders(user.identity())
                    .await?
                    .value
                    .len() as u64;

                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account =
                    create_account(0, assets.quote.to_contract_units(70000.0), 0, 0);
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
                Ok(())
            }

            #[tokio::test]
            async fn bob_sell_same_price_greater_amount() -> anyhow::Result<()> {
                Ok(())
            }

            #[tokio::test]
            async fn bob_sell_same_price_smaller_amount() -> anyhow::Result<()> {
                Ok(())
            }
        }
    }

    #[ignore]
    #[tokio::test]
    async fn alice_sell_exact_amount_base_asset_to_bob() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (alice_contract, owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let bob_contract = alice_contract.with_account(&user.wallet).await?;

        let alice_order_id = open_order(
            &alice_contract,
            &assets.base,
            1.0,
            OrderType::Sell,
            1.0,
            &assets.base,
            70000.0,
            &owner,
            true,
            1,
            &defaults,
        )
        .await?;

        let bob_order_id = open_order(
            &bob_contract,
            &assets.quote,
            70000.0,
            OrderType::Buy,
            1.0,
            &assets.base,
            70000.0,
            &user,
            false,
            1,
            &defaults,
        )
        .await?;

        let _response = alice_contract
            .batch_fulfill(alice_order_id, vec![bob_order_id])
            .await?;

        let alice_account = alice_contract
            .account(owner.identity())
            .await?
            .value
            .unwrap();
        let alice_expected_account =
            create_account(0, assets.quote.to_contract_units(70000.0), 0, 0);
        assert_eq!(alice_account, alice_expected_account);

        let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
        let bob_expected_account = create_account(assets.base.to_contract_units(1.0), 0, 0, 0);
        assert_eq!(bob_account, bob_expected_account);

        Ok(())
    }

    #[tokio::test]
    async fn alice_sell_exact_amount_quote_asset_to_bob() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (alice_contract, owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let bob_contract = alice_contract.with_account(&user.wallet).await?;

        let alice_deposit = assets.quote.to_contract_units(70000.0);
        let alice_deposit_asset = assets.quote.id;
        let alice_order_type = OrderType::Sell;
        let alice_price = assets.quote.to_contract_units(70000.0); // TODO: this should use price formula / type instead of quote
        let _ = alice_contract
            .deposit(alice_deposit, alice_deposit_asset)
            .await;

        let alice_order_id = alice_contract
            .open_order(
                alice_deposit,
                alice_deposit_asset,
                alice_order_type,
                alice_price,
            )
            .await?
            .value;

        let alice_account = alice_contract
            .account(owner.identity())
            .await?
            .value
            .unwrap();
        let alice_expected_account = create_account(0, 0, 0, alice_deposit);
        assert_eq!(alice_account, alice_expected_account);

        let bob_deposit = assets.base.to_contract_units(1.0);
        let bob_deposit_asset = assets.base.id;
        let bob_order_type = OrderType::Buy;
        let bob_price = assets.base.to_contract_units(70000.0); // TODO: this should use price formula / type instead of base
        let _ = bob_contract
            .with_account(&user.wallet)
            .await?
            .deposit(bob_deposit, bob_deposit_asset)
            .await;

        let bob_order_id = bob_contract
            .open_order(
                alice_deposit,
                alice_deposit_asset,
                bob_order_type,
                bob_price,
            )
            .await?
            .value;

        let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
        let bob_expected_account = create_account(0, 0, bob_deposit, 0);
        assert_eq!(bob_account, bob_expected_account);

        let _response = alice_contract
            .batch_fulfill(alice_order_id, vec![bob_order_id])
            .await?;

        let alice_account = alice_contract
            .account(owner.identity())
            .await?
            .value
            .unwrap();
        let alice_expected_account = create_account(bob_deposit, 0, 0, 0);
        assert_eq!(alice_account, alice_expected_account);

        let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
        let bob_expected_account = create_account(0, alice_deposit, 0, 0);
        assert_eq!(bob_account, bob_expected_account);
        Ok(())
    }

    #[tokio::test]
    async fn alice_sells_two_base_asset_orders_from_bob() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (alice_contract, owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let bob_contract = alice_contract.with_account(&user.wallet).await?;

        let alice_deposit = assets.quote.to_contract_units(70000.0);
        let alice_deposit_asset = assets.quote.id;
        let alice_order_type = OrderType::Sell;
        let alice_price = assets.quote.to_contract_units(70000.0); // TODO: this should use price formula / type instead of quote
        let _ = alice_contract
            .deposit(alice_deposit, alice_deposit_asset)
            .await;

        let alice_order_id = alice_contract
            .open_order(
                alice_deposit,
                alice_deposit_asset,
                alice_order_type,
                alice_price,
            )
            .await?
            .value;

        let alice_account = alice_contract
            .account(owner.identity())
            .await?
            .value
            .unwrap();
        let alice_expected_account = create_account(0, 0, 0, alice_deposit);
        assert_eq!(alice_account, alice_expected_account);

        let bob_deposit = assets.base.to_contract_units(1.0);
        let bob_deposit_asset = assets.base.id;
        let bob_order_type = OrderType::Buy;
        let bob_price = assets.base.to_contract_units(70000.0); // TODO: this should use price formula / type instead of base
        let _ = bob_contract
            .with_account(&user.wallet)
            .await?
            .deposit(bob_deposit, bob_deposit_asset)
            .await;

        let bob_order_id1 = bob_contract
            .open_order(
                alice_deposit / 5,
                alice_deposit_asset,
                bob_order_type.clone(),
                bob_price,
            )
            .await?
            .value;

        let bob_order_id2 = bob_contract
            .open_order(
                alice_deposit / 5 * 4,
                alice_deposit_asset,
                bob_order_type,
                bob_price,
            )
            .await?
            .value;

        let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
        let bob_expected_account = create_account(0, 0, bob_deposit, 0);
        assert_eq!(bob_account, bob_expected_account);

        let _response = alice_contract
            .batch_fulfill(alice_order_id, vec![bob_order_id1, bob_order_id2])
            .await?;

        let alice_account = alice_contract
            .account(owner.identity())
            .await?
            .value
            .unwrap();
        let alice_expected_account = create_account(bob_deposit, 0, 0, 0);
        assert_eq!(alice_account, alice_expected_account);

        let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
        let bob_expected_account = create_account(0, alice_deposit, 0, 0);
        assert_eq!(bob_account, bob_expected_account);
        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NoOrdersFound")]
    async fn when_alice_order_does_not_exist() {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        // Revert
        contract
            .batch_fulfill(Bits256([0u8; 32]), vec![])
            .await
            .unwrap();
    }
}
