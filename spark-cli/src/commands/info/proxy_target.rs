use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_proxy_sdk::SparkProxyContract;

#[derive(Args, Clone)]
#[command(about = "Query the proxy target")]
pub(crate) struct ProxyTargetCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl ProxyTargetCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = SparkProxyContract::new(contract_id, wallet).await;

        let proxy_target = contract.proxy_target().await?.value;

        println!("Proxy target: {:?}", proxy_target);

        Ok(())
    }
}
