use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Address, ContractId},
};
use orderbook::{
    constants::{ORDERBOOK_CONTRACT_ID, RPC, TOKEN_CONTRACT_ID},
    orderbook_utils::Orderbook,
    print_title,
};
use src20_sdk::token_utils::{Asset, TokenContract};
use std::{env, str::FromStr};

const MARKET_SYMBOL: &str = "BTC";
const BASE_SIZE: i64 = -100; //units
const BASE_PRICE: u64 = 69432; //units

#[tokio::main]
async fn main() {
    print_title("Create Order");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
    println!("wallet address = {:?}", wallet.address());
    let token_contract = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );

    let token_contract_id = token_contract.contract_id().into();
    let base_asset = Asset::new(wallet.clone(), token_contract_id, MARKET_SYMBOL);
    let quote_asset = Asset::new(wallet.clone(), token_contract_id, "USDC");

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    if BASE_SIZE > 0 {
        let quote_size = quote_asset.parse_units(BASE_SIZE as f64 * BASE_PRICE as f64);
        quote_asset
            .mint(wallet.address().into(), quote_size as u64)
            .await
            .unwrap();
    } else {
        let base_size = base_asset.parse_units(BASE_SIZE.abs() as f64) as u64;
        base_asset
            .mint(wallet.address().into(), base_size)
            .await
            .unwrap();
    }

    let price = BASE_PRICE * 10u64.pow(orderbook.price_decimals as u32);

    match orderbook
        .open_order(base_asset.asset_id, BASE_SIZE, price)
        .await
    {
        Ok(response) => {
            let id = Address::from(response.value.0).to_string();
            println!("Order opened successfully. OrderId: 0x{id}");
            println!("Gas Used: {:?}", response.gas_used);
            println!("Transaction ID: 0x{:?}", response.tx_id.unwrap());
        }
        Err(error) => {
            eprintln!("Failed to open order: {:?}", error);
        }
    }
}
