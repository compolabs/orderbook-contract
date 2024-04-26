use dotenv::dotenv;
use fuels::{
    accounts::Account,
    core::constants::BASE_ASSET_ID,
    crypto::SecretKey,
    prelude::{Provider, WalletUnlocked},
    types::{transaction::TxPolicies, ContractId},
};
use rand::{rngs::StdRng, SeedableRng};
use src20_sdk::token_utils::{Asset, TokenContract};
use std::str::FromStr;
use utils::constants::{RPC, TOKEN_CONTRACT_ID};

const AMOUNT_OF_WALLETS: u64 = 5;
// const AMOUNT_OF_WALLETS: u64 = 1;
const ETH_AMOUNT: f64 = 0.005;
const BASE_SYMBOL: &str = "BTC";
const BASE_SIZE: f64 = 1.;
const QUOTE_SIZE: f64 = 70000.;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let provider = Provider::connect(RPC).await?;
    let secret = std::env::var("ADMIN")?;
    let admin = WalletUnlocked::new_from_private_key(secret.parse()?, Some(provider.clone()));

    let token_contract = TokenContract::new(
        ContractId::from_str(TOKEN_CONTRACT_ID).unwrap(),
        admin.clone(),
    );

    let token_contract_id = token_contract.contract_id().into();
    let base_asset = Asset::new(admin.clone(), token_contract_id, BASE_SYMBOL);
    let quote_asset = Asset::new(admin.clone(), token_contract_id, "USDC");
    let eth = Asset {
        asset_id: BASE_ASSET_ID,
        decimals: 9,
        symbol: "ETH".to_string(),
        token_contract_instance: None,
    };

    let base_size = base_asset.parse_units(BASE_SIZE) as u64;
    let quote_size = quote_asset.parse_units(QUOTE_SIZE) as u64;
    let eth_size = eth.parse_units(ETH_AMOUNT) as u64;

    for _ in 0..AMOUNT_OF_WALLETS {
        let mut rng = StdRng::from_entropy();
        let pk = SecretKey::random(&mut rng);
        let wallet = WalletUnlocked::new_from_private_key(pk, Some(provider.clone()));
        println!(
            "Address:     {:?}\nPrivate key: {:?}\n",
            wallet.address().to_string(),
            pk.to_string()
        );

        admin
            .transfer(
                wallet.address(),
                eth_size,
                BASE_ASSET_ID,
                TxPolicies::default(),
            )
            .await?;

        base_asset.mint(wallet.address().into(), base_size).await?;

        quote_asset
            .mint(wallet.address().into(), quote_size)
            .await?;
    }

    Ok(())
}
