use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_market_sdk::MarketContract;

#[derive(Args, Clone)]
#[command(about = "Query the protocol fee")]
pub(crate) struct ProtocolFeeCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl ProtocolFeeCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet).await;

        let protocol_fee = contract.protocol_fee().await?.value;

        println!("Protocol Fee: {:?}", protocol_fee);

        Ok(())
    }
}
