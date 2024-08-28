use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_market_sdk::MarketContract;

#[derive(Args, Clone)]
#[command(about = "Query the epoch")]
pub(crate) struct EpochCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl EpochCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet).await;

        let epoch = contract.get_epoch().await?.value;

        println!("\nEpoch: epoch {}, duration {}", epoch.0, epoch.1);

        Ok(())
    }
}
