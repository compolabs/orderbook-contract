use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::ContractId,
};
use hex::ToHex;
use spark_market_sdk::OrderbookContract;
use src20_sdk::token_utils::{Asset, TokenContract};
use std::str::FromStr;
use utils::{
    constants::{ORDERBOOK_CONTRACT_BINARY_PATH, RPC, TOKEN_CONTRACT_ID},
    title::print_title,
};
const MARKET_SYMBOL: &str = "UNI";
const BASE_SIZE: u64 = 100; //units
const BASE_PRICE: u64 = 10; //units

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_title("Match Orders");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));
    println!("admin address = {:?}", wallet.address().to_string());

    let token_contract = TokenContract::new(
        ContractId::from_str(TOKEN_CONTRACT_ID).unwrap(),
        wallet.clone(),
    );

    // deploy
    let usdc = Asset::new(wallet.clone(), token_contract.contract_id().into(), "USDC");
    let contract = OrderbookContract::deploy(
        &wallet,
        usdc.asset_id,
        usdc.decimals as u32,
        9,
        ORDERBOOK_CONTRACT_BINARY_PATH,
    )
    .await?;

    let contract_id_str = contract.id().encode_hex::<String>();
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

    let orderbook = OrderbookContract::new(
        ContractId::from_str(&contract_id_str).unwrap(),
        wallet.clone(),
    )
    .await?;

    orderbook
        .create_market(asset.asset_id, asset.decimals as u32)
        .await?;

    println!("Market created on contract id: {}\n", contract_id_str);

    let token_contract_id = token_contract.contract_id().into();
    let base_asset = Asset::new(wallet.clone(), token_contract_id, MARKET_SYMBOL);
    let quote_asset = Asset::new(wallet.clone(), token_contract_id, "USDC");

    let orderbook = OrderbookContract::new(
        ContractId::from_str(&contract_id_str).unwrap(),
        wallet.clone(),
    )
    .await?;
    let price = BASE_PRICE * 10u64.pow(orderbook.price_decimals);

    //mint base asset to sell
    let base_size = base_asset.parse_units(BASE_SIZE as f64) as u64;
    base_asset.mint(wallet.address().into(), base_size).await?;

    // sell
    let sell_order_id = orderbook
        .open_order(base_asset.asset_id, -(base_size as i64), price - 1)
        .await?
        .value;

    //mint quote asset to buy
    let quote_size = quote_asset.parse_units(BASE_SIZE as f64 * BASE_PRICE as f64);
    quote_asset
        .mint(wallet.address().into(), quote_size as u64)
        .await?;

    //buy
    let buy_order_id = orderbook
        .open_order(base_asset.asset_id, base_size as i64, price)
        .await?
        .value;

    println!(
        "buy_order = {:?}",
        orderbook.order_by_id(&buy_order_id).await?.value.unwrap()
    );

    println!(
        "sell_order = {:?}",
        orderbook.order_by_id(&sell_order_id).await?.value.unwrap()
    );

    orderbook
        .match_orders(&sell_order_id, &buy_order_id)
        .await?;

    Ok(())
}
