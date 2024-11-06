use crate::utils::{setup, validate_contract_id};
use clap::Args;
use spark_proxy_sdk::SparkProxyContract;

#[derive(Args, Clone)]
#[command(about = "Query the proxy owner")]
pub(crate) struct ProxyOwnerCommand {
    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl ProxyOwnerCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = SparkProxyContract::new(contract_id, wallet).await;

        let proxy_owner = contract.proxy_owner().await?.value;

        println!("Proxy owner: {:?}", proxy_owner);

        Ok(())
    }
}
