use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::ContractId,
};
use orderbook::{
    constants::{ORDERBOOK_CONTRACT_ID, RPC, TOKEN_CONTRACT_ID},
    orderbook_utils::Orderbook,
    print_title,
};
use src20_sdk::token_utils::{Asset, TokenContract};
use std::str::FromStr;

const MARKET_SYMBOL: &str = "BTC";
const BASE_SIZE: f64 = 15.; //units
const START_PRICE: f64 = 65500.; //units
const STEP: f64 = 100.;

#[tokio::main]
async fn main() {
    print_title("Fill orderbook Orders");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let token_contract = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );

    let token_contract_id = token_contract.contract_id().into();
    let base_asset = Asset::new(wallet.clone(), token_contract_id, MARKET_SYMBOL);
    let quote_asset = Asset::new(wallet.clone(), token_contract_id, "USDC");

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    let base_size = base_asset.parse_units(BASE_SIZE as f64) as u64;
    for i in 1..41 {
        let diff = STEP * i as f64;
        //sell
        let sell_price =
            ((START_PRICE + diff) * 10f64.powf(orderbook.price_decimals as f64)) as u64;

        let mint_tx = base_asset.mint(wallet.address().into(), base_size).await;
        if mint_tx.is_ok() {
            let order_tx = orderbook
                .open_order(base_asset.asset_id, -1 * base_size as i64, sell_price - 1)
                .await;

            if order_tx.is_err() {
                println!("Cannot crete a sell order {:?}", order_tx.err().unwrap());
            } else {
                println!("Sell order created = {:?}", order_tx.unwrap().value);
            }
        }

        //buy
        let buy_price = ((START_PRICE - diff) * 10f64.powf(orderbook.price_decimals as f64)) as u64;
        let quote_size = quote_asset.parse_units(BASE_SIZE as f64 * (START_PRICE - diff) as f64);

        let mint_tx = quote_asset
            .mint(wallet.address().into(), quote_size as u64)
            .await;
        if mint_tx.is_ok() {
            let order_tx = orderbook
                .open_order(base_asset.asset_id, base_size as i64, buy_price)
                .await;

            if order_tx.is_err() {
                println!("Cannot crete a buy order {:?}", order_tx.err().unwrap());
            } else {
                println!("BUY order created = {:?}", order_tx.unwrap().value);
            }
        }
    }
}
