use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::types::AssetId;
use spark_orderbook_sdk::OrderbookContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Gets market contract ids")]
pub(crate) struct MarketsCommand {
    /// The ids of the assets
    #[clap(long)]
    pub(crate) assets: Vec<String>,

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

        if self.assets.len() == 0 {
            anyhow::bail!("Invalid asset array length");
        }

        let mut asset_ids: Vec<AssetId> = Vec::new();
        for asset in self.assets.clone() {
            asset_ids.push(AssetId::from_str(&asset).expect("Invalid asset"));
        }

        // Connect to the deployed contract via the rpc
        let contract = OrderbookContract::new(contract_id, wallet).await;

        let markets = contract.registered_markets(asset_ids).await?.value;

        // TODO: replace println with tracing
        println!("\nMarkets: {:?}", markets);

        Ok(())
    }
}
