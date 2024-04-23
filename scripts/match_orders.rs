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
const BASE_SIZE: f64 = 0.01; //units
const BASE_PRICE: f64 = 65500.; //units

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

    let token_contract_id = token_contract.contract_id().into();
    let base_asset = Asset::new(wallet.clone(), token_contract_id, MARKET_SYMBOL);
    let quote_asset = Asset::new(wallet.clone(), token_contract_id, "USDC");

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;
    let price = (BASE_PRICE * 10f64.powf(orderbook.price_decimals as f64)) as u64;

    //mint base asset to sell
    let base_size = base_asset.parse_units(BASE_SIZE as f64) as u64;
    base_asset
        .mint(wallet.address().into(), base_size)
        .await
        .unwrap();

    // sell
    let sell_order_id = orderbook
        .open_order(base_asset.asset_id, -1 * base_size as i64, price - 1)
        .await
        .unwrap()
        .value;

    //mint quote asset to buy
    let quote_size = quote_asset.parse_units(BASE_SIZE as f64 * BASE_PRICE as f64);
    quote_asset
        .mint(wallet.address().into(), quote_size as u64)
        .await
        .unwrap();

    //buy
    let buy_order_id = orderbook
        .open_order(base_asset.asset_id, base_size as i64, price)
        .await
        .unwrap()
        .value;

    println!(
        "buy_order = {:?}\n",
        orderbook.order_by_id(&buy_order_id).await.unwrap().value.unwrap()
    );
    println!(
        "sell_order = {:?}",
        orderbook.order_by_id(&sell_order_id).await.unwrap().value.unwrap()
    );


    orderbook
        .match_orders(&sell_order_id, &buy_order_id)
        .await
        .unwrap();
}
