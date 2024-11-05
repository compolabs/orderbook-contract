use crate::setup::{clone_market, setup, Defaults};
use rand::Rng;

mod success {

    use super::*;
    use crate::setup::create_account;
    use spark_market_sdk::{AssetType, WithdrawToMarketEvent};

    #[tokio::test]
    async fn base_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let market = clone_market(owner.wallet.clone(), &contract).await?;

        let deposit_amount = 100;

        let _ = contract.deposit(deposit_amount, assets.base.id).await?;

        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(deposit_amount, 0, 0, 0);

        // Precondition enforces deposited account
        assert_eq!(user_account, expected_account);

        let expected_account = create_account(0, 0, 0, 0);

        let response = contract
            .withdraw_to_market(deposit_amount, AssetType::Base, market.contract_id())
            .await?;

        let log = response
            .decode_logs_with_type::<WithdrawToMarketEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            WithdrawToMarketEvent {
                amount: deposit_amount,
                asset: assets.base.id,
                user: owner.identity(),
                account: expected_account,
                market: market.contract_id().into(),
            }
        );

        let balance = owner
            .contract_balance(market.contract_id(), assets.base.id)
            .await;
        let user_account = market.account(owner.identity()).await?.value;

        let expected_account = create_account(deposit_amount, 0, 0, 0);

        assert_eq!(balance, deposit_amount);
        assert_eq!(user_account, expected_account);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn fuzz_base_asset_deposit_withdraw() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let market = clone_market(owner.wallet.clone(), &contract).await?;
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let deposit_amount: u64 = rng.gen_range(1..10_u64.pow(defaults.base_decimals));

            // Perform deposit
            let _ = contract.deposit(deposit_amount, assets.base.id).await?;

            let balance = owner
                .contract_balance(market.contract_id(), assets.base.id)
                .await;
            let user_account = contract.account(owner.identity()).await?.value;
            let expected_account = create_account(deposit_amount, 0, 0, 0);

            // Assert the deposit is what is expected
            assert_eq!(user_account, expected_account);

            let response = contract
                .withdraw_to_market(deposit_amount, AssetType::Base, market.contract_id())
                .await?;

            let log = response
                .decode_logs_with_type::<WithdrawToMarketEvent>()
                .unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                WithdrawToMarketEvent {
                    amount: deposit_amount,
                    asset: assets.base.id,
                    user: owner.identity(),
                    account: expected_account,
                    market: market.contract_id().into(),
                }
            );

            let new_balance = owner
                .contract_balance(market.contract_id(), assets.base.id)
                .await;
            let user_account = market.account(owner.identity()).await?.value;

            let expected_account = create_account(deposit_amount, 0, 0, 0);

            assert_eq!(new_balance - balance, deposit_amount);
            assert_eq!(user_account, expected_account);
        }
        Ok(())
    }

    #[tokio::test]
    async fn quote_asset() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let market = clone_market(owner.wallet.clone(), &contract).await?;

        let deposit_amount = 100;

        let _ = contract.deposit(deposit_amount, assets.quote.id).await?;

        let balance = owner
            .contract_balance(market.contract_id(), assets.base.id)
            .await;
        let user_account = contract.account(owner.identity()).await?.value;
        let expected_account = create_account(0, deposit_amount, 0, 0);

        // Precondition enforces deposited account
        assert_eq!(user_account, expected_account);

        let expected_account = create_account(0, 0, 0, 0);

        let response = contract
            .withdraw_to_market(deposit_amount, AssetType::Quote, market.contract_id())
            .await?;

        let log = response
            .decode_logs_with_type::<WithdrawToMarketEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            WithdrawToMarketEvent {
                amount: deposit_amount,
                asset: assets.quote.id,
                user: owner.identity(),
                account: expected_account,
                market: market.contract_id().into(),
            }
        );

        let new_balance = owner
            .contract_balance(market.contract_id(), assets.quote.id)
            .await;
        let user_account = market.account(owner.identity()).await?.value;

        let expected_account = create_account(0, deposit_amount, 0, 0);

        assert_eq!(new_balance - balance, deposit_amount);
        assert_eq!(user_account, expected_account);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn fuzz_quote_asset_deposit_withdraw() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;
        let market = clone_market(owner.wallet.clone(), &contract).await?;

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let deposit_amount: u64 = rng.gen_range(1..10_u64.pow(defaults.quote_decimals));

            // Perform deposit
            let _ = contract.deposit(deposit_amount, assets.quote.id).await?;

            let balance = owner
                .contract_balance(market.contract_id(), assets.base.id)
                .await;
            let user_account = contract.account(owner.identity()).await?.value;
            let expected_account = create_account(0, deposit_amount, 0, 0);

            // Assert the deposit is what is expected
            assert_eq!(user_account, expected_account);

            let expected_account = create_account(0, 0, 0, 0);

            // Perform withdrawal
            let response = contract
                .withdraw_to_market(deposit_amount, AssetType::Quote, market.contract_id())
                .await?;

            let log = response
                .decode_logs_with_type::<WithdrawToMarketEvent>()
                .unwrap();
            let event = log.first().unwrap();
            assert_eq!(
                *event,
                WithdrawToMarketEvent {
                    amount: deposit_amount,
                    asset: assets.quote.id,
                    user: owner.identity(),
                    account: expected_account,
                    market: market.contract_id().into(),
                }
            );

            let new_balance = owner
                .contract_balance(market.contract_id(), assets.quote.id)
                .await;
            let user_account = market.account(owner.identity()).await?.value;

            let expected_account = create_account(0, deposit_amount, 0, 0);

            // Assert the withdrawal is correct
            assert_eq!(new_balance - balance, deposit_amount);
            assert_eq!(user_account, expected_account);
        }
        Ok(())
    }
}

mod revert {

    use super::*;
    use spark_market_sdk::{AssetType, SparkMarketContract};

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_withdrawing_without_account() {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, _) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();
        let market = clone_market(owner.wallet.clone(), &contract).await.unwrap();

        let deposit_amount = 100;

        // Revert
        contract
            .withdraw_to_market(deposit_amount, AssetType::Base, market.contract_id())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_base_amount_greater_than_available() {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();
        let market = clone_market(owner.wallet.clone(), &contract).await.unwrap();

        let deposit_amount = 100;

        let _ = contract
            .deposit(deposit_amount, assets.base.id)
            .await
            .unwrap();

        // Revert
        contract
            .withdraw_to_market(deposit_amount + 1, AssetType::Base, market.contract_id())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientBalance")]
    async fn when_quote_amount_greater_than_available() {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();
        let market = clone_market(owner.wallet.clone(), &contract).await.unwrap();

        let deposit_amount = 100;

        let _ = contract
            .deposit(deposit_amount, assets.quote.id)
            .await
            .unwrap();

        contract.pause().await.unwrap();

        // Revert
        contract
            .withdraw_to_market(deposit_amount + 1, AssetType::Quote, market.contract_id())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidMarketAsset")]
    async fn when_base_different_on_market() {
        let defaults = Defaults::default();
        let (contract, owner, _user, _, _, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let config = contract.config().await.unwrap().value;
        let market = SparkMarketContract::deploy(
            assets.random.id,
            config.1,
            config.2,
            config.3,
            owner.wallet,
            config.5,
            config.6,
        )
        .await
        .unwrap();

        let deposit_amount = 100;

        let _ = contract
            .deposit(deposit_amount, assets.base.id)
            .await
            .unwrap();

        // Revert
        contract
            .withdraw_to_market(deposit_amount, AssetType::Base, market.contract_id())
            .await
            .unwrap();
    }
}
