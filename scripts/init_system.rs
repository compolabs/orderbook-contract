use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Address, ContractId},
};
use hex::ToHex;
use orderbook::{
    constants::{RPC, TOKEN_CONTRACT_ID},
    orderbook_utils::Orderbook,
    print_title,
};
use src20_sdk::token_utils::{Asset, TokenContract};
use std::{str::FromStr, time::Instant};
const MARKET_SYMBOL: &str = "BTC";
const BASE_SIZE: f64 = 1.; //units
const START_PRICE: f64 = 69500.; //units

#[tokio::main]
async fn main() {
    print_title("Init system");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
    println!("admin address = {:?}\n", wallet.address().to_string());

    let token_contract = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );

    // deploy
    let usdc = Asset::new(wallet.clone(), token_contract.contract_id().into(), "USDC");
    let contract = Orderbook::deploy(&wallet, usdc.asset_id, usdc.decimals, 9).await;

    let block = provider.latest_block_height().await.unwrap();
    println!("🏁 Start_block: {block}");
    let contract_id_str = contract.instance.contract_id().hash.encode_hex::<String>();
    println!(
        "The orderbook contract has been deployed with contract id: 0x{}\n",
        contract_id_str
    );

    // create market
    let asset = Asset::new(
        wallet.clone(),
        token_contract.contract_id().into(),
        MARKET_SYMBOL,
    );

    let orderbook = Orderbook::new(&wallet, &contract_id_str).await;

    orderbook
        ._create_market(asset.asset_id, asset.decimals as u32)
        .await
        .unwrap();

    println!("Market created on contract id: 0x{}\n", contract_id_str);

    let token_contract_id = token_contract.contract_id().into();
    let base_asset = Asset::new(wallet.clone(), token_contract_id, MARKET_SYMBOL);
    let quote_asset = Asset::new(wallet.clone(), token_contract_id, "USDC");

    let base_size = base_asset.parse_units(BASE_SIZE as f64) as u64;
    for i in 1..41 {
        let diff = i as f64;
        //sell
        let sell_price =
            ((START_PRICE + diff) * 10f64.powf(orderbook.price_decimals as f64)) as u64;

        let mint_tx = base_asset.mint(wallet.address().into(), base_size).await;
        if mint_tx.is_ok() {
            let start = Instant::now();
            let order_tx = orderbook
                .open_order(base_asset.asset_id, -1 * base_size as i64, sell_price - 1)
                .await;
            let finish = start.elapsed();
            match order_tx {
                Ok(response) => {
                    let id = Address::from(response.value.0).to_string();
                    println!("Sell OrderId: 0x{}", id);
                    println!("Gas Used: {:?}", response.gas_used);
                    println!("Transaction ID: 0x{:?}", response.tx_id.unwrap());
                    println!("Duration: {:?}", finish);
                    println!("Sell Price: {}\n", sell_price);
                }
                Err(error) => {
                    println!("Failed to create a sell order: {:?}\n", error);
                }
            }
        }

        //buy
        let buy_price = ((START_PRICE + diff) * 10f64.powf(orderbook.price_decimals as f64)) as u64;
        let quote_size = quote_asset.parse_units(BASE_SIZE as f64 * (START_PRICE + diff) as f64);

        let mint_tx = quote_asset
            .mint(wallet.address().into(), quote_size as u64)
            .await;
        if mint_tx.is_ok() {
            let start = Instant::now();
            let order_tx = orderbook
                .open_order(base_asset.asset_id, base_size as i64, buy_price)
                .await;
            let finish = start.elapsed();
            match order_tx {
                Ok(response) => {
                    let id = Address::from(response.value.0).to_string();
                    println!("Buy OrderId: 0x{}", id);
                    println!("Gas Used: {:?}", response.gas_used);
                    println!("Transaction ID: 0x{:?}", response.tx_id.unwrap());
                    println!("Duration: {:?}", finish);
                    println!("Buy Price: {}\n", sell_price);
                }
                Err(error) => {
                    println!("Failed to create a buy order: {:?}\n", error);
                }
            }
        }
    }
}
