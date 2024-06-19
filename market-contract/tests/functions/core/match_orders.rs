use crate::setup::{create_account, setup, Defaults};
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::{AssetType, OrderType};

mod success {

    use super::*;

    #[tokio::test]
    async fn match_orders() -> anyhow::Result<()> {
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

        /*let id0 = contract
        .with_account(&user0.wallet)
        .await?
        .open_order(base_amount, AssetType::Base, OrderType::Sell, price)
        .await?
        .value;*/

        Ok(())
    }
}
