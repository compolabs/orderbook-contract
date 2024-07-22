use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_market_sdk::MarketContract;

#[derive(Args, Clone)]
#[command(about = "Query the total protocol fee")]
pub(crate) struct TotalProtocolFeeCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl TotalProtocolFeeCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet).await;

        let total_protocol_fee = contract.total_protocol_fee().await?.value;

        println!("Total Protocol Fee: {}", total_protocol_fee);

        Ok(())
    }
}
