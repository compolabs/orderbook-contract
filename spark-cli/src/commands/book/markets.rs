use crate::utils::{setup, validate_contract_id, AccountType, OrderType};
use clap::Args;
use fuels::types::{Address, AssetId, ContractId, Identity};
use spark_market_sdk::{MarketContract, OrderType as ContractOrderType};
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Gets market contract ids")]
pub(crate) struct MarketsCommand {
    /// The id of the asset
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

        if self.assets.len() as u64 != 0 {
            anyhow::bail!("Invalid asset array length");
        }

        //let asset = AssetId::from_str(&self.asset).expect("Invalid asset");

        // TODO: cli parsing

        Ok(())
    }
}
