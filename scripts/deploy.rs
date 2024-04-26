use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::ContractId,
};
use spark_market_sdk::OrderbookContract;
use src20_sdk::token_utils::{Asset, TokenContract};
use std::str::FromStr;
use utils::{
    constants::{RPC, TOKEN_CONTRACT_ID},
    title::print_title,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_title("Deploy");

    dotenv().ok();
    let provider = Provider::connect(RPC).await?;
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
    println!("admin address = {:?}", wallet.address().to_string());

    let token_contarct = TokenContract::new(
        ContractId::from_str(TOKEN_CONTRACT_ID).unwrap(),
        wallet.clone(),
    );
    let usdc = Asset::new(wallet.clone(), token_contarct.contract_id().into(), "USDC");
    let contract =
        OrderbookContract::deploy(&wallet, usdc.asset_id, usdc.decimals as u32, 9).await?;

    let block = provider.latest_block_height().await?;
    println!("🏁 Start_block: {block}");

    println!(
        "The orderbook contract has been deployed {}\n",
        contract.id()
    );

    Ok(())
}
