use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Address, ContractId},
};
use orderbook::{constants::ORDERBOOK_CONTRACT_ID, orderbook_utils::Orderbook, print_title};
use src20_sdk::{
    constants::{RPC, TOKEN_CONTRACT_ID},
    token_utils::{Asset, TokenContract},
};
use std::str::FromStr;

const MARKET_SYMBOL: &str = "UNI";

#[tokio::main]
async fn main() {
    print_title("Match Orders");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let token_contract = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );

    let asset = Asset::new(
        wallet.clone(),
        token_contract.contract_id().into(),
        MARKET_SYMBOL,
    );

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    let base_size = 100;
    let base_price = 10;

    let sell_order_id = orderbook
        .open_order(asset.asset_id, -1 * base_size, base_price)
        .await
        .unwrap()
        .value;

    let buy_order_id = orderbook
        .open_order(asset.asset_id, base_size, base_price)
        .await
        .unwrap()
        .value;

    //todo match orders
    orderbook
        .match_orders(&sell_order_id, &buy_order_id)
        .await
        .unwrap();


}
