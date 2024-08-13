use crate::setup::{create_account, setup, Defaults};
use fuels::prelude::*;
use spark_market_sdk::OrderType;

mod success {

    use super::*;

    #[tokio::test]
    async fn assert_protocol_fee_is_zero_before_order_match() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, user0, user1, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000_000_000_000_u64; // 70,000$ price
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount;
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(quote_amount, assets.quote.id)
            .await?;

        let accumulated_protocol_fee = contract.total_protocol_fee().await.unwrap().value;
        assert_eq!(accumulated_protocol_fee, 0);

        let id0 = contract
            .with_account(&user0.wallet)
            .await?
            .open_order(
                base_amount,
                /*AssetType::Base,*/ OrderType::Sell,
                price,
            )
            .await?
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .await?
            .open_order(base_amount, /*AssetType::Base,*/ OrderType::Buy, price)
            .await?
            .value;

        let accumulated_protocol_fee = contract.total_protocol_fee().await.unwrap().value;
        assert_eq!(accumulated_protocol_fee, 0);

        let cancel_order_tx0 = contract
            .with_account(&user0.wallet)
            .await?
            .cancel_order(id0)
            .await;
        let cancel_order_tx1 = contract
            .with_account(&user1.wallet)
            .await?
            .cancel_order(id1)
            .await;
        assert!(cancel_order_tx0.is_ok());
        assert!(cancel_order_tx1.is_ok());

        let accumulated_protocol_fee = contract.total_protocol_fee().await.unwrap().value;
        assert_eq!(accumulated_protocol_fee, 0);

        Ok(())
    }

    #[tokio::test]
    async fn assert_protocol_fee_accumulates_after_match_order() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, user0, user1, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000_000_000_000_u64; // 70,000$ price
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount;
        contract
            .with_account(&user0.wallet)
            .await?
            .deposit(base_amount, assets.base.id)
            .await?;
        contract
            .with_account(&user1.wallet)
            .await?
            .deposit(quote_amount, assets.quote.id)
            .await?;

        // protocol fees are 0 after deposit
        let accumulated_protocol_fee = contract.total_protocol_fee().await.unwrap().value;
        assert_eq!(accumulated_protocol_fee, 0);

        let id0 = contract
            .with_account(&user0.wallet)
            .await?
            .open_order(
                base_amount,
                /*AssetType::Base,*/ OrderType::Sell,
                price,
            )
            .await?
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .await?
            .open_order(base_amount, /*AssetType::Base,*/ OrderType::Buy, price)
            .await?
            .value;

        // protocol fees are 0 afer open order
        let accumulated_protocol_fee = contract.total_protocol_fee().await.unwrap().value;
        assert_eq!(accumulated_protocol_fee, 0);

        contract.match_order_pair(id0, id1).await?;

        // protocol fees increased after sucessful match order
        let accumulated_protocol_fee = contract.total_protocol_fee().await.unwrap().value;
        assert!(accumulated_protocol_fee != 0);

        let withdraw_tx = contract.withdraw_protocol_fee(user0.identity()).await;
        assert!(withdraw_tx.is_ok());

        let receipts = withdraw_tx.unwrap().receipts;

        let mut withdraw_tx_transfer_out_amount = 0;
        for receipt in receipts {
            if let Receipt::TransferOut { amount, .. } = receipt {
                withdraw_tx_transfer_out_amount += amount;
            }
        }

        assert_eq!(accumulated_protocol_fee, withdraw_tx_transfer_out_amount);

        // after withdraw protocol fees are 0
        let accumulated_protocol_fee_after_withdraw =
            contract.total_protocol_fee().await.unwrap().value;
        assert_eq!(accumulated_protocol_fee_after_withdraw, 0);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn reverts_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let new_fee = 5;

        // Reverts
        contract
            .with_account(&user.wallet)
            .await
            .unwrap()
            .set_protocol_fee(new_fee)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn reverts_when_non_owner_attemps_withdraw() {
        let defaults = Defaults::default();
        let (contract, user0, user1, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let to_quote_scale =
            10_u64.pow(defaults.price_decimals + defaults.base_decimals - defaults.quote_decimals);
        let price = 70_000_000_000_000_u64; // 70,000$ price
        let base_amount = 100_000_u64; // 0.001 BTC
        let quote_amount = price / to_quote_scale * base_amount;
        contract
            .with_account(&user0.wallet)
            .await
            .unwrap()
            .deposit(base_amount, assets.base.id)
            .await
            .unwrap();
        contract
            .with_account(&user1.wallet)
            .await
            .unwrap()
            .deposit(quote_amount, assets.quote.id)
            .await
            .unwrap();

        let id0 = contract
            .with_account(&user0.wallet)
            .await
            .unwrap()
            .open_order(
                base_amount,
                /*AssetType::Base,*/ OrderType::Sell,
                price,
            )
            .await
            .unwrap()
            .value;
        let id1 = contract
            .with_account(&user1.wallet)
            .await
            .unwrap()
            .open_order(base_amount, /*AssetType::Base,*/ OrderType::Buy, price)
            .await
            .unwrap()
            .value;

        contract.match_order_pair(id0, id1).await.unwrap();

        contract
            .with_account(&user1.wallet)
            .await
            .unwrap()
            .withdraw_protocol_fee(user1.identity())
            .await
            .unwrap();
    }
}
