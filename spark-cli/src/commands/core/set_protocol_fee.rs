use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::{MarketContract, ProtocolFee};

#[derive(Args, Clone)]
#[command(about = "Change the protocol fee")]
pub(crate) struct SetProtocolFeeCommand {
    /// The fee to set
    #[clap(long)]
    pub(crate) fee: u64, // Todo Vec<ProtocolFee>,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl SetProtocolFeeCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        // Todo
        //let _ = contract.set_protocol_fee(self.fee.clone()).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("\nThe global fee has been set to: {:?}", self.fee);
        println!("Contract call cost: {}", balance - new_balance);

        Ok(())
    }
}
