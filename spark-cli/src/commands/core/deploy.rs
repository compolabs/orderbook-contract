use crate::utils::setup;
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::AssetId};
use spark_market_sdk::SparkMarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Deploys the market to a network")]
pub(crate) struct DeployCommand {
    /// The asset id for the base asset of the market
    #[clap(long)]
    pub(crate) base_asset: String,

    /// The number of decimals the base asset implements
    #[clap(long)]
    pub(crate) base_decimals: u32,

    /// The asset id for the quote asset of the market
    #[clap(long)]
    pub(crate) quote_asset: String,

    /// The number of decimals the quote asset implements
    #[clap(long)]
    pub(crate) quote_decimals: u32,

    /// The number of decimals the price uses
    #[clap(long)]
    pub(crate) price_decimals: u32,

    /// The URL to deploy to
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl DeployCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;

        if self.base_asset.len() as u64 != 66 {
            anyhow::bail!("Invalid base asset length");
        }

        if self.quote_asset.len() as u64 != 66 {
            anyhow::bail!("Invalid quote asset length");
        }

        let base_asset = AssetId::from_str(&self.base_asset).expect("Invalid base asset");
        let quote_asset = AssetId::from_str(&self.quote_asset).expect("Invalid quote asset");

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        let version = SparkMarketContract::sdk_version();

        // Deploy the contract
        let contract = SparkMarketContract::deploy(
            base_asset,
            self.base_decimals,
            quote_asset,
            self.quote_decimals,
            wallet.clone(),
            self.price_decimals,
            version,
        )
        .await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nMarket version {} ({}) deployed to: 0x{}",
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
