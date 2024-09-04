use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_registry_sdk::SparkRegistryContract;

#[derive(Args, Clone)]
#[command(about = "Query the MarketRegistry for its configurable variables")]
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
        let contract = SparkRegistryContract::new(contract_id, wallet).await;

        let (owner, version) = contract.config().await?.value;

        println!("\nOwner: 0x{}", owner);
        println!(
            "Version: {} ({})",
            contract.contract_str_version().await?,
            version
        );

        Ok(())
    }
}
