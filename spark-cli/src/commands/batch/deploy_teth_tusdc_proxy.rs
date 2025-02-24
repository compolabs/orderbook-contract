use crate::utils::setup;
use clap::Args;
use fuels::{
    accounts::ViewOnlyAccount,
    types::{AssetId, ContractId},
};
use spark_market_sdk::SparkMarketContract;
use spark_proxy_sdk::{SparkProxyContract, State};
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Deploys the teth/tusdc market proxy to a network")]
pub(crate) struct DeployTethTusdcProxyCommand {}

impl DeployTethTusdcProxyCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup("mainnet.fuel.network").await?;

        let (teth, tusdc) = (
            "0xf169e13e98ae8908199148380684894458b7916f074b85ebad2aaad489ce0d54",
            "0x22dfb618b9fc621a7d53f0f599dd427fb5688e280062a8de8883a27819d3f276",
        );

        let base_asset = AssetId::from_str(&teth).unwrap();
        let quote_asset = AssetId::from_str(&tusdc).unwrap();
        let base_decimals = 9;
        let quote_decimals = 6;
        let price_decimals = 9;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        let version = SparkMarketContract::sdk_version();

        // Deploy the contract
        let contract = SparkMarketContract::deploy(
            base_asset,
            base_decimals,
            quote_asset,
            quote_decimals,
            wallet.clone(),
            price_decimals,
            version,
        )
        .await?;

        let _ = contract.pause().await?;

        let target: ContractId = contract.contract_id().into();
        let proxy = SparkProxyContract::deploy(target, wallet.clone()).await?;

        assert!(proxy.proxy_owner().await?.value == State::Initialized(wallet.address().into()));

        let market = SparkMarketContract::new(proxy.contract_id().into(), wallet.clone()).await;
        let _ = market.initialize_ownership(wallet.address().into()).await?;

        let epoch = 4611686020165700000; // 31/02/2025
        let epoch_duration = 2600000; // 30 days
        let min_price = 500_000_000_000; // 500 USDC
        let min_size = 400_000; // 0.0004 ETH

        let _ = market.set_epoch(epoch, epoch_duration).await?;
        let _ = market.set_min_order_size(min_size).await?;
        let _ = market.set_min_order_price(min_price).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nMarket version {} ({}) deployed to: 0x{}
               Proxy deployed to: 0x{}",
            contract.contract_str_version().await?,
            version,
            contract.id(),
            proxy.id(),
        );
        println!("Deployment cost: {}", balance - new_balance);
        println!("Owner address: {}", wallet.address());
        println!("               0x{}", wallet.address().hash());

        Ok(())
    }
}
