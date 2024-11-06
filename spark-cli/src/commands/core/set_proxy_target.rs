use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_proxy_sdk::SparkProxyContract;

#[derive(Args, Clone)]
#[command(about = "Change the proxy target")]
pub(crate) struct SetProxyTargetCommand {
    /// The target to set
    #[clap(long)]
    pub(crate) target: String,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl SetProxyTargetCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;
        let target = validate_contract_id(&self.target)?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = SparkProxyContract::new(contract_id, wallet.clone()).await;

        let _ = contract.set_proxy_target(target).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("\nThe proxy target has been set to: {:?}", self.target);
        println!("Contract call cost: {}", balance - new_balance);

        Ok(())
    }
}
