use crate::utils::setup;
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_orderbook_sdk::OrderbookContract;

#[derive(Args, Clone)]
#[command(about = "Deploys the orderbook to a network")]
pub(crate) struct DeployCommand {
    /// The URL to deploy to
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl DeployCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Deploy the contract
        let contract = OrderbookContract::deploy(wallet.clone()).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // TODO: replace println with tracing
        println!("\nOrderbook deployed to: 0x{}", contract.id());
        println!("Deployment cost: {}", balance - new_balance);
        println!("Owner address: {}", wallet.address());
        println!("               0x{}", wallet.address().hash());

        Ok(())
    }
}
