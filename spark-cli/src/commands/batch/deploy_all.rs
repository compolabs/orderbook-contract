use crate::utils::setup;
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::AssetId};
use spark_market_sdk::{ProtocolFee, SparkMarketContract};
use spark_registry_sdk::SparkRegistryContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(
    about = "Deploys the markets to a network and setup them then deploy registry and register markets"
)]
pub(crate) struct DeployAllCommand {
    /// The URL to deploy to
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl DeployAllCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;

        let eth = "0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07";
        let btc = "0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc";
        let usdc = "0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05";

        let quote_asset = AssetId::from_str(&usdc).unwrap();
        let quote_decimals = 6;
        let price_decimals = 9;
        let version = SparkMarketContract::sdk_version();
        let matcher_fee = 1_000;
        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 10,
                taker_fee: 15,
                volume_threshold: 0,
            },
            ProtocolFee {
                maker_fee: 8,
                taker_fee: 13,
                volume_threshold: 10000000000,
            },
            ProtocolFee {
                maker_fee: 6,
                taker_fee: 11,
                volume_threshold: 50000000000,
            },
            ProtocolFee {
                maker_fee: 4,
                taker_fee: 9,
                volume_threshold: 100000000000,
            },
            ProtocolFee {
                maker_fee: 2,
                taker_fee: 7,
                volume_threshold: 500000000000,
            },
            ProtocolFee {
                maker_fee: 1,
                taker_fee: 5,
                volume_threshold: 1000000000000,
            },
        ];
        let epoch = 4611686020155120000; // 10/01/2024
        let epoch_duration = 2510000; // 30 days
        let mut markets = vec![];

        // 1. Deploy BTC/USDC market contract
        let base_asset = AssetId::from_str(&btc).unwrap();
        let base_decimals = 8;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

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

        let _ = contract.set_matcher_fee(matcher_fee).await?;
        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;
        let _ = contract.set_epoch(epoch, epoch_duration).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nBTC/USDC Market version {} ({}) deployed to: 0x{}",
            contract.contract_str_version().await?,
            version,
            contract.id()
        );
        markets.push(contract.contract_id());
        println!("Deployment cost: {}", balance - new_balance);

        // 2. Deploy ETH/USDC market contract
        let base_asset = AssetId::from_str(&eth).unwrap();
        let base_decimals = 9;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

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

        let _ = contract.set_matcher_fee(matcher_fee).await?;
        let _ = contract.set_protocol_fee(protocol_fee.clone()).await?;
        let _ = contract.set_epoch(epoch, epoch_duration).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nETH/USDC Market version {} ({}) deployed to: 0x{}",
            contract.contract_str_version().await?,
            version,
            contract.id()
        );
        markets.push(contract.contract_id());
        println!("Deployment cost: {}", balance - new_balance);

        // Last. Deploy Registry contract
        let version = SparkRegistryContract::sdk_version();

        // Deploy the contract
        let contract = SparkRegistryContract::deploy(wallet.clone(), version).await?;

        for market in markets {
            let _ = contract.register_market(market.into()).await?;
        }

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nMarketRegistry version {} ({}) deployed to: 0x{}",
            contract.contract_str_version().await?,
            version,
            contract.id()
        );
        println!("Deployment cost: {}", balance - new_balance);

        Ok(())
    }
}
