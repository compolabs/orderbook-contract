use std::str::FromStr;

use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Address, Bits256, ContractId},
};
use orderbook::{constants::ORDERBOOK_CONTRACT_ID, orderbook_utils::Orderbook, print_title};
use src20_sdk::{
    constants::{RPC, TOKEN_CONTRACT_ID},
    token_utils::{Asset, TokenContract},
};

const ORDER_ID: &str = "f9d1c605af0397bd459e0af6558beb9459fbd48f97b27fa92eea4c1f37b03d64"; 

#[tokio::main]
async fn main() {
    print_title("Cancel order");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;


    let id = &Bits256::from_hex_str(ORDER_ID).unwrap();
    
    let order = orderbook
        .order_by_id(id)
        .await
        .unwrap()
        .value;

    println!("{:#?}", order);
    
    orderbook
        .cancel_order(id) //fixme TransferZeroCoins
        .await
        .unwrap();
}
