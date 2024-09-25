use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_market_sdk::SparkMarketContract;

#[derive(Args, Clone)]
#[command(about = "Query the store order change info")]
pub(crate) struct StoreOrderChangeInfoCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl StoreOrderChangeInfoCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = SparkMarketContract::new(contract_id, wallet).await;

        let store_order_change_info = contract.store_order_change_info().await?.value;

        println!("\nStore order change info: {}", store_order_change_info);

        Ok(())
    }
}
