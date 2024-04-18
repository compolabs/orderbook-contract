use std::str::FromStr;

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

#[tokio::main]
async fn main() {
    print_title("Deploy");

    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
    println!("admin address = {:?}", wallet.address().to_string());

    let token_contarct = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );
    let usdc = Asset::new(wallet.clone(), token_contarct.contract_id().into(), "USDC");
    let contract = Orderbook::deploy(&wallet, usdc.asset_id, usdc.decimals, 9).await;

    let block = provider.latest_block_height().await.unwrap();
    println!("🏁 Start_block: {block}");

    println!(
        "The orderbook contract has been deployed {}\n",
        contract.instance.contract_id().hash
    );
}
