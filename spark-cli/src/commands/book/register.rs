use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::AssetId};
use spark_orderbook_sdk::OrderbookContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Registers a market in orderbook")]
pub(crate) struct RegisterCommand {
    /// The contract id of the orderbook
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The asset id for the asset of the market
    #[clap(long)]
    pub(crate) asset: String,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) market: String,

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

        if self.asset.len() as u64 != 66 {
            anyhow::bail!("Invalid asset length");
        }

        let asset = AssetId::from_str(&self.asset).expect("Invalid asset");

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // Connect to the deployed contract via the rpc
        let contract = OrderbookContract::new(contract_id, wallet.clone()).await;

        let _ = contract.register_market(asset, market).await?;

        // Balance post-call
        let new_balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // TODO: replace println with tracing
        println!("\nContract call cost: {}", balance - new_balance);

        Ok(())
    }
}
