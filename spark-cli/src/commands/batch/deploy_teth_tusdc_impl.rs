use crate::utils::setup;
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::AssetId};
use spark_market_sdk::SparkMarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Deploys the teth/tusdc market impl to a network")]
pub(crate) struct DeployTethTusdcImplCommand {
    /// The URL to deploy to
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl DeployTethTusdcImplCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;

        let (eth, _, usdc) = match &*self.rpc {
            "testnet.fuel.network" => (
                "0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07",
                "0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc",
                "0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05",
            ),
            "mainnet.fuel.network" => (
                "0xf169e13e98ae8908199148380684894458b7916f074b85ebad2aaad489ce0d54",
                "0x0dc8cdbe2798cb45ebc99180afc0bc514ffb505a80f122004378955c1d23892c",
                "0x22dfb618b9fc621a7d53f0f599dd427fb5688e280062a8de8883a27819d3f276",
            ),
            _ => ("", "", ""),
        };

        let base_asset = AssetId::from_str(&eth).unwrap();
        let quote_asset = AssetId::from_str(&usdc).unwrap();
        let base_decimals = 9;
        let quote_decimals = 6;
        let price_decimals = 9;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        let version = SparkMarketContract::sdk_version();

        // Deploy the contract
        let market = SparkMarketContract::deploy(
            base_asset,
            base_decimals,
            quote_asset,
            quote_decimals,
            wallet.clone(),
            price_decimals,
            version,
        )
        .await?;

        let _ = market.pause().await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nMarket version {} ({}) deployed to: 0x{}",
            market.contract_str_version().await?,
            version,
            market.id(),
        );
        println!("Deployment cost: {}", balance - new_balance);
        println!("Owner address: {}", wallet.address());
        println!("               0x{}", wallet.address().hash());

        Ok(())
    }
}