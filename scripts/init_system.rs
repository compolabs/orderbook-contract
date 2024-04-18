use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::ContractId,
};
use orderbook::{
    constants::{RPC, TOKEN_CONTRACT_ID},
    orderbook_utils::Orderbook,
    print_title,
};
use src20_sdk::token_utils::{Asset, TokenContract};
use std::str::FromStr;
use hex::ToHex;
const MARKET_SYMBOL: &str = "UNI";
const BASE_SIZE: u64 = 100; //units
const BASE_PRICE: u64 = 10; //units

#[tokio::main]
async fn main() {
    print_title("Match Orders");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
    println!("admin address = {:?}", wallet.address().to_string());

    let token_contract = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );
    
    // deploy
    let usdc = Asset::new(wallet.clone(), token_contract.contract_id().into(), "USDC");
    let contract = Orderbook::deploy(&wallet, usdc.asset_id, usdc.decimals, 9).await;
    
    let contract_id_str = contract.instance.contract_id().hash.encode_hex::<String>();
    println!(
        "The orderbook contract has been deployed with contract id: {}\n",
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

    println!(
        "Market created on contract id: {}\n",
        contract_id_str
    );
    

    let token_contract_id = token_contract.contract_id().into();
    let base_asset = Asset::new(wallet.clone(), token_contract_id, MARKET_SYMBOL);
    let quote_asset = Asset::new(wallet.clone(), token_contract_id, "USDC");

    let orderbook = Orderbook::new(&wallet, &contract_id_str).await;
    let price = BASE_PRICE * 10u64.pow(orderbook.price_decimals as u32);

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
        "buy_order = {:?}",
        orderbook
            .order_by_id(&buy_order_id)
            .await
            .unwrap()
            .value
            .unwrap()
    );
    
    println!(
        "sell_order = {:?}",
        orderbook
            .order_by_id(&sell_order_id)
            .await
            .unwrap()
            .value
            .unwrap()
    );

    orderbook
        .match_orders(&sell_order_id, &buy_order_id)
        .await
        .unwrap();
}