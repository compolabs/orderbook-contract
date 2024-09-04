use crate::utils::setup;
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_registry_sdk::SparkRegistryContract;

#[derive(Args, Clone)]
#[command(about = "Deploys the MarketRegistry to a network")]
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

        let version = SparkRegistryContract::sdk_version();

        // Deploy the contract
        let contract = SparkRegistryContract::deploy(wallet.clone(), version).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\n MarketRegistry version {} ({}) deployed to: 0x{}",
            contract.contract_str_version().await?,
            version,
            contract.id()
        );
        println!("Deployment cost: {}", balance - new_balance);
        println!("Owner address: {}", wallet.address());
        println!("               0x{}", wallet.address().hash());

        Ok(())
    }
}
