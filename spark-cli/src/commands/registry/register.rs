use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_registry_sdk::SparkRegistryContract;

#[derive(Args, Clone)]
#[command(about = "Registers a market in the market registry")]
pub(crate) struct RegisterCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) market: String,

    /// The contract id of the market registry
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl RegisterCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;
        let market = validate_contract_id(&self.market)?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = SparkRegistryContract::new(contract_id, wallet.clone()).await;

        let _ = contract.register_market(market).await?;

        // Balance post-call
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("\nContract call cost: {}", balance - new_balance);

        Ok(())
    }
}
