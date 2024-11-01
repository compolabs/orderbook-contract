use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_market_sdk::SparkMarketContract;

#[derive(Args, Clone)]
#[command(about = "Query the market for its configurable variables")]
pub(crate) struct ConfigCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl ConfigCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = SparkMarketContract::new(contract_id, wallet).await;

        let (
            base_asset,
            base_asset_decimals,
            quote_asset,
            quote_asset_decimals,
            owner,
            price_decimals,
            version,
        ) = contract.config().await?.value;

        println!("\nBase Asset: 0x{}", base_asset);
        println!("Base Asset Decimals: {}", base_asset_decimals);
        println!("Quote Asset: 0x{}", quote_asset);
        println!("Quote Asset Decimals: {}", quote_asset_decimals);
        println!("Owner: {:?}", owner);
        println!("Price Decimals: {}", price_decimals);
        println!(
            "Version: {}.{}.{}",
            (version & 0xFF0000) >> 16,
            (version & 0xFF00) >> 8,
            version & 0xFF
        );

        Ok(())
    }
}
