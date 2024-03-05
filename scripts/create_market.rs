use std::str::FromStr;

use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::ContractId,
};
use orderbook::{constants::ORDERBOOK_CONTRACT_ID, orderbook_utils::Orderbook, print_title};
use src20_sdk::{
    constants::{RPC, TOKEN_CONTRACT_ID},
    token_utils::{Asset, TokenContract},
};

const MARKET_SYMBOL: &str = "UNI";

#[tokio::main]
async fn main() {
    print_title("Create market");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let token_contarct = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );

    let asset = Asset::new(
        wallet.clone(),
        token_contarct.contract_id().into(),
        MARKET_SYMBOL,
    );

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    orderbook
        ._create_market(asset.asset_id, asset.decimals as u32)
        .await
        .unwrap();
}
