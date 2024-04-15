use crate::setup::{price_to_contract_units, setup, Defaults};
use fuels::types::Bits256;
use spark_market_sdk::OrderType;

mod success {

    use super::*;
    use crate::setup::{base_to_quote_amount, create_account, quote_to_base_amount, Asset, User};
    use spark_market_sdk::MarketContract;

    // Utility fn to reduce setup in each test when opening orders.
    // This likely has bugs because it's not symmetrical
    // Only intended for 1 order per user
    #[allow(clippy::too_many_arguments)]
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
        order_asset_base: bool,
        expected_order_count: u64,
        defaults: &Defaults,
    ) -> anyhow::Result<Bits256> {
        let deposit = deposit_asset.to_contract_units(deposit_amount);
        let _ = contract.deposit(deposit, deposit_asset.id).await?;

        let order_amount = order_asset.to_contract_units(order_amount);
        let price = price_to_contract_units(defaults.price_decimals, price);

        let order_id = contract
            .open_order(order_amount, order_asset.id, order_type, price)
            .await?
            .value;

        let order_count = contract.user_orders(user.identity()).await?.value.len() as u64;
        let account = contract.account(user.identity()).await?.value.unwrap();
        let expected_account = match deposited_base {
            true => {
                match order_asset_base {
                    true => {
                        // should be selling
                        create_account(deposit - order_amount, 0, order_amount, 0)
                    }
                    false => {
                        // should be buying
                        let order_amount_in_base = quote_to_base_amount(
                            order_amount,
                            defaults.base_decimals,
                            price,
                            defaults.price_decimals,
                            defaults.quote_decimals,
                        );
                        let remaining_amount = deposit - order_amount_in_base;
                        create_account(remaining_amount, 0, order_amount_in_base, 0)
                    }
                }
            }
            false => {
                // TODO: this may need updating like above?
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

    // Batch is a vector with a single buy order to fulfill
    mod batch_of_one {
        use super::*;

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

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();

                // Calculate what bob's account should contain after the trade
                let bob_deposit = assets.quote.to_contract_units(70000.0 * 1.5);
                let bob_order_amount_in_quote = base_to_quote_amount(
                    assets.base.to_contract_units(1.25),
                    defaults.base_decimals,
                    price_to_contract_units(defaults.price_decimals, 71000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let bob_liquid_deposit = bob_deposit - bob_order_amount_in_quote;
                let alice_amount_in_quote = base_to_quote_amount(
                    assets.base.to_contract_units(1.0),
                    defaults.base_decimals,
                    price_to_contract_units(defaults.price_decimals, 70000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let bob_locked_amount = bob_order_amount_in_quote - alice_amount_in_quote;

                let bob_expected_account = create_account(
                    assets.base.to_contract_units(1.0),
                    bob_liquid_deposit,
                    0,
                    bob_locked_amount,
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 1);
                Ok(())
            }

            #[tokio::test]
            async fn bob_buy_greater_price_same_amount() -> anyhow::Result<()> {
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
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    70000.0 * 2.0,
                    OrderType::Buy,
                    1.0,
                    &assets.base,
                    71000.0,
                    &user,
                    false,
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

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(1.0),
                    assets.quote.to_contract_units(70000.0),
                    0,
                    0,
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
                Ok(())
            }

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
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    71000.0,
                    OrderType::Buy,
                    0.5,
                    &assets.base,
                    71000.0,
                    &user,
                    false,
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bought_amount_in_base = quote_to_base_amount(
                    bob_account.locked.quote,
                    defaults.base_decimals,
                    price_to_contract_units(defaults.price_decimals, 71000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let bought_amount_in_quote = base_to_quote_amount(
                    bought_amount_in_base,
                    defaults.base_decimals,
                    price_to_contract_units(defaults.price_decimals, 71000.0),
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
                assert_eq!(alice_orders, 1);

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account =
                    create_account(bought_amount_in_base, bought_amount_in_quote, 0, 0);
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
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

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account =
                    create_account(assets.base.to_contract_units(1.0), 0, 0, 0);
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
                Ok(())
            }

            #[tokio::test]
            async fn bob_buy_same_price_greater_amount() -> anyhow::Result<()> {
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
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    70000.0 * 2.0,
                    OrderType::Buy,
                    1.5,
                    &assets.base,
                    70000.0,
                    &user,
                    false,
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

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(1.0),
                    assets.quote.to_contract_units(35000.0),
                    0,
                    assets.quote.to_contract_units(35000.0),
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 1);

                Ok(())
            }

            #[tokio::test]
            async fn bob_buy_same_price_smaller_amount() -> anyhow::Result<()> {
                // Bob should get the exact amount that he wants to buy
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
                    0.5,
                    &assets.base,
                    70000.0,
                    &user,
                    false,
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
                let alice_expected_account = create_account(
                    0,
                    assets.quote.to_contract_units(35000.0),
                    assets.base.to_contract_units(0.5),
                    0,
                );
                assert_eq!(alice_account, alice_expected_account);
                assert_eq!(alice_orders, 1);

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(0.5),
                    assets.quote.to_contract_units(35000.0),
                    0,
                    0,
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
                Ok(())
            }
        }

        // These tests needs to be reworked. They are copy and pasted from above and need to be
        // corrected to do what the fn name suggests
        mod alice_sell_quote {
            use super::*;

            #[ignore]
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
                    &assets.quote,
                    70000.0,
                    OrderType::Sell,
                    70000.0,
                    &assets.quote,
                    1.0,
                    &owner,
                    false,
                    false,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.base,
                    2.0,
                    OrderType::Buy,
                    70000.0,
                    &assets.quote,
                    70000.0,
                    &user,
                    true,
                    false,
                    1,
                    &defaults,
                )
                .await?;

                // TODO: assert log events
                let _response = alice_contract
                    .batch_fulfill(alice_order_id, vec![bob_order_id])
                    .await?;

                // let alice_orders = alice_contract
                //     .user_orders(owner.identity())
                //     .await?
                //     .value
                //     .len() as u64;
                // let alice_account = alice_contract
                //     .account(owner.identity())
                //     .await?
                //     .value
                //     .unwrap();
                // let alice_expected_account =
                //     create_account(assets.base.to_contract_units(1.0), 0, 0, 0);
                // assert_eq!(alice_account, alice_expected_account);
                // assert_eq!(alice_orders, 0);

                // let bob_orders =
                //     bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                // let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();

                // // Calculate what bob's account should contain after the trade
                // let bob_deposit = assets.quote.to_contract_units(70000.0 * 1.5);
                // let bob_order_amount_in_quote = base_to_quote_amount(
                //     assets.base.to_contract_units(1.25),
                //     defaults.base_decimals,
                //     price_to_contract_units(defaults.price_decimals, 71000.0),
                //     defaults.price_decimals,
                //     defaults.quote_decimals,
                // );
                // let bob_liquid_deposit = bob_deposit - bob_order_amount_in_quote;
                // let alice_amount_in_quote = base_to_quote_amount(
                //     assets.base.to_contract_units(1.0),
                //     defaults.base_decimals,
                //     price_to_contract_units(defaults.price_decimals, 70000.0),
                //     defaults.price_decimals,
                //     defaults.quote_decimals,
                // );
                // let bob_locked_amount = bob_order_amount_in_quote - alice_amount_in_quote;

                // let bob_expected_account = create_account(
                //     assets.base.to_contract_units(1.0),
                //     bob_liquid_deposit,
                //     0,
                //     bob_locked_amount,
                // );
                // assert_eq!(bob_account, bob_expected_account);
                // assert_eq!(bob_orders, 1);
                Ok(())
            }

            #[ignore]
            #[tokio::test]
            async fn bob_buy_greater_price_same_amount() -> anyhow::Result<()> {
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
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    70000.0 * 2.0,
                    OrderType::Buy,
                    1.0,
                    &assets.base,
                    71000.0,
                    &user,
                    false,
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

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(1.0),
                    assets.quote.to_contract_units(70000.0),
                    0,
                    0,
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
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
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    71000.0,
                    OrderType::Buy,
                    0.5,
                    &assets.base,
                    71000.0,
                    &user,
                    false,
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bought_amount_in_base = quote_to_base_amount(
                    bob_account.locked.quote,
                    defaults.base_decimals,
                    price_to_contract_units(defaults.price_decimals, 71000.0),
                    defaults.price_decimals,
                    defaults.quote_decimals,
                );
                let bought_amount_in_quote = base_to_quote_amount(
                    bought_amount_in_base,
                    defaults.base_decimals,
                    price_to_contract_units(defaults.price_decimals, 71000.0),
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
                assert_eq!(alice_orders, 1);

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account =
                    create_account(bought_amount_in_base, bought_amount_in_quote, 0, 0);
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
                Ok(())
            }

            #[ignore]
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

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
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
                    true,
                    1,
                    &defaults,
                )
                .await?;

                let bob_order_id = open_order(
                    &bob_contract,
                    &assets.quote,
                    70000.0 * 2.0,
                    OrderType::Buy,
                    1.5,
                    &assets.base,
                    70000.0,
                    &user,
                    false,
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

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(1.0),
                    assets.quote.to_contract_units(35000.0),
                    0,
                    assets.quote.to_contract_units(35000.0),
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 1);

                Ok(())
            }

            #[ignore]
            #[tokio::test]
            async fn bob_buy_same_price_smaller_amount() -> anyhow::Result<()> {
                // Bob should get the exact amount that he wants to buy
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
                    0.5,
                    &assets.base,
                    70000.0,
                    &user,
                    false,
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
                let alice_expected_account = create_account(
                    0,
                    assets.quote.to_contract_units(35000.0),
                    assets.base.to_contract_units(0.5),
                    0,
                );
                assert_eq!(alice_account, alice_expected_account);
                assert_eq!(alice_orders, 1);

                let bob_orders =
                    bob_contract.user_orders(user.identity()).await?.value.len() as u64;
                let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
                let bob_expected_account = create_account(
                    assets.base.to_contract_units(0.5),
                    assets.quote.to_contract_units(35000.0),
                    0,
                    0,
                );
                assert_eq!(bob_account, bob_expected_account);
                assert_eq!(bob_orders, 0);
                Ok(())
            }
        }
    }

    // Batch is a vector with 2 buy orders to fulfill
    mod batch_of_two {
        use super::*;

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
                true,
                1,
                &defaults,
            )
            .await?;

            let bob_order_id1 = open_order(
                &bob_contract,
                &assets.quote,
                70000.0,
                OrderType::Buy,
                0.2,
                &assets.base,
                70000.0,
                &user,
                false,
                true,
                1,
                &defaults,
            )
            .await?;

            // Manually open another order for bob
            let order_amount = assets.base.to_contract_units(0.8);
            let price = price_to_contract_units(defaults.price_decimals, 70000.0);

            let bob_order_id2 = bob_contract
                .open_order(order_amount, assets.base.id, OrderType::Buy, price)
                .await?
                .value;

            let order_count = bob_contract.user_orders(user.identity()).await?.value.len() as u64;
            let account = bob_contract.account(user.identity()).await?.value.unwrap();
            let expected_account = create_account(0, 0, 0, assets.quote.to_contract_units(70000.0));

            assert_eq!(account, expected_account);
            assert_eq!(order_count, 2);

            let _response = alice_contract
                .batch_fulfill(alice_order_id, vec![bob_order_id1, bob_order_id2])
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

            let bob_orders = bob_contract.user_orders(user.identity()).await?.value.len() as u64;
            let bob_account = bob_contract.account(user.identity()).await?.value.unwrap();
            let bob_expected_account = create_account(assets.base.to_contract_units(1.0), 0, 0, 0);
            assert_eq!(bob_account, bob_expected_account);
            assert_eq!(bob_orders, 0);

            Ok(())
        }
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

    #[tokio::test]
    #[should_panic(expected = "LeftShouldBeSellOrder")]
    async fn when_left_is_a_buy_order() {
        let defaults = Defaults::default();
        let (alice_contract, _owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();
        let bob_contract = alice_contract.with_account(&user.wallet).await.unwrap();

        let _ = alice_contract
            .deposit(assets.quote.to_contract_units(70000.0), assets.quote.id)
            .await
            .unwrap();

        let alice_order_id = alice_contract
            .open_order(
                assets.base.to_contract_units(1.0),
                assets.base.id,
                OrderType::Buy,
                price_to_contract_units(defaults.price_decimals, 70000.0),
            )
            .await
            .unwrap()
            .value;

        let _ = bob_contract
            .deposit(assets.quote.to_contract_units(70000.0), assets.quote.id)
            .await
            .unwrap();

        let bob_order_id = bob_contract
            .open_order(
                assets.base.to_contract_units(1.0),
                assets.base.id,
                OrderType::Buy,
                price_to_contract_units(defaults.price_decimals, 70000.0),
            )
            .await
            .unwrap()
            .value;

        alice_contract
            .batch_fulfill(alice_order_id, vec![bob_order_id])
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "RightShouldBeBuyOrder")]
    async fn when_right_is_a_sell_order() {
        let defaults = Defaults::default();
        let (alice_contract, _owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();
        let bob_contract = alice_contract.with_account(&user.wallet).await.unwrap();

        let _ = alice_contract
            .deposit(assets.base.to_contract_units(1.0), assets.base.id)
            .await
            .unwrap();

        let alice_order_id = alice_contract
            .open_order(
                assets.base.to_contract_units(1.0),
                assets.base.id,
                OrderType::Sell,
                price_to_contract_units(defaults.price_decimals, 70000.0),
            )
            .await
            .unwrap()
            .value;

        let _ = bob_contract
            .deposit(assets.base.to_contract_units(1.0), assets.base.id)
            .await
            .unwrap();

        let bob_order_id = bob_contract
            .open_order(
                assets.base.to_contract_units(1.0),
                assets.base.id,
                OrderType::Sell,
                price_to_contract_units(defaults.price_decimals, 70000.0),
            )
            .await
            .unwrap()
            .value;

        alice_contract
            .batch_fulfill(alice_order_id, vec![bob_order_id])
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "AssetMismatch")]
    async fn when_asset_mismatch() {
        let defaults = Defaults::default();
        let (alice_contract, _owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();
        let bob_contract = alice_contract.with_account(&user.wallet).await.unwrap();

        let _ = alice_contract
            .deposit(assets.base.to_contract_units(1.0), assets.base.id)
            .await
            .unwrap();

        let alice_order_id = alice_contract
            .open_order(
                assets.base.to_contract_units(1.0),
                assets.base.id,
                OrderType::Sell,
                price_to_contract_units(defaults.price_decimals, 70000.0),
            )
            .await
            .unwrap()
            .value;

        let _ = bob_contract
            .deposit(assets.base.to_contract_units(1.0), assets.base.id)
            .await
            .unwrap();
        let _ = bob_contract
            .deposit(assets.quote.to_contract_units(70000.0), assets.quote.id)
            .await
            .unwrap();

        let bob_order_id = bob_contract
            .open_order(
                assets.quote.to_contract_units(70000.0),
                assets.quote.id,
                OrderType::Buy,
                price_to_contract_units(defaults.price_decimals, 70000.0),
            )
            .await
            .unwrap()
            .value;

        alice_contract
            .batch_fulfill(alice_order_id, vec![bob_order_id])
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBuyPrice")]
    async fn when_insufficient_buy_price() {
        let defaults = Defaults::default();
        let (alice_contract, _owner, user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();
        let bob_contract = alice_contract.with_account(&user.wallet).await.unwrap();

        let _ = alice_contract
            .deposit(assets.base.to_contract_units(1.0), assets.base.id)
            .await
            .unwrap();

        let alice_order_id = alice_contract
            .open_order(
                assets.base.to_contract_units(1.0),
                assets.base.id,
                OrderType::Sell,
                price_to_contract_units(defaults.price_decimals, 70000.0),
            )
            .await
            .unwrap()
            .value;

        let _ = bob_contract
            .deposit(assets.quote.to_contract_units(70000.0), assets.quote.id)
            .await
            .unwrap();

        let bob_order_id = bob_contract
            .open_order(
                assets.base.to_contract_units(1.0),
                assets.base.id,
                OrderType::Buy,
                price_to_contract_units(defaults.price_decimals, 69000.0),
            )
            .await
            .unwrap()
            .value;

        alice_contract
            .batch_fulfill(alice_order_id, vec![bob_order_id])
            .await
            .unwrap();
    }
}
