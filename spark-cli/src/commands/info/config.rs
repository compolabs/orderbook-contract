use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_market::MarketContract;

#[derive(Args, Clone)]
#[command(about = "Query the market for its configurable variables")]
pub(crate) struct ConfigCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. beta-5.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl ConfigCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet).await;

        let (
            owner,
            base_asset,
            base_asset_decimals,
            quote_asset,
            quote_asset_decimals,
            price_decimals,
        ) = contract.config().await?.value;

        // TODO: replace println with tracing
        println!("\nOwner: 0x{}", owner);
        println!("Base Asset: 0x{}", base_asset);
        println!("Base Asset Decimals: {}", base_asset_decimals);
        println!("Quote Asset: 0x{}", quote_asset);
        println!("Quote Asset Decimals: {}", quote_asset_decimals);
        println!("Price Decimals: {}", price_decimals);

        Ok(())
    }
}
