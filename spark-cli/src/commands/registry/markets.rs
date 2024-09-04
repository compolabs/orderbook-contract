use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::types::AssetId;
use spark_registry_sdk::SparkRegistryContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Gets market contract ids")]
pub(crate) struct MarketsCommand {
    /// The base asset id
    #[clap(long)]
    pub(crate) base: String,

    /// The quote asset id
    #[clap(long)]
    pub(crate) quote: String,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl MarketsCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        let mut asset_ids: Vec<(AssetId, AssetId)> = Vec::new();
        asset_ids.push((
            AssetId::from_str(&self.base).expect("Invalid asset"),
            AssetId::from_str(&self.quote).expect("Invalid asset"),
        ));

        // Connect to the deployed contract via the rpc
        let contract = SparkRegistryContract::new(contract_id, wallet).await;

        let markets = contract.markets(asset_ids).await?.value;

        println!("\nMarkets: {:?}", markets);

        Ok(())
    }
}
