use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::ContractId,
};
use spark_market_sdk::OrderbookContract;
use src20_sdk::token_utils::{Asset, TokenContract};
use std::str::FromStr;
use utils::{
    constants::{ORDERBOOK_CONTRACT_ID, RPC, TOKEN_CONTRACT_ID},
    title::print_title,
};

const MARKET_SYMBOL: &str = "BTC";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_title("Create market");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let token_contarct = TokenContract::new(
        ContractId::from_str(TOKEN_CONTRACT_ID).unwrap(),
        wallet.clone(),
    );

    let asset = Asset::new(
        wallet.clone(),
        token_contarct.contract_id().into(),
        MARKET_SYMBOL,
    );

    let orderbook =
        OrderbookContract::new(ContractId::from_str(ORDERBOOK_CONTRACT_ID).unwrap(), wallet)
            .await?;

    orderbook
        .create_market(asset.asset_id, asset.decimals as u32)
        .await?;

    Ok(())
}
