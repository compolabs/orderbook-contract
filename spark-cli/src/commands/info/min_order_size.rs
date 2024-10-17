use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_market_sdk::SparkMarketContract;

#[derive(Args, Clone)]
#[command(about = "Query the minimum order size")]
pub(crate) struct MinOrderSizeCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl MinOrderSizeCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = SparkMarketContract::new(contract_id, wallet).await;

        let min_order_size = contract.min_order_size().await?.value;

        println!("\nMinimum Order Size: {}", min_order_size);

        Ok(())
    }
}
