use crate::utils::setup;
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::AssetId};
use spark_market_sdk::{ProtocolFee, SparkMarketContract};
use spark_registry_sdk::SparkRegistryContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(
    about = "Deploys the markets to a network and sets them up, then deploys registry and registers markets"
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
        let matcher_fee = 1_000; // 0.001 USDC

        // multi tier protocol fee structure
        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 25,       // 0.25% maker fee
                taker_fee: 40,       // 0.40% taker fee
                volume_threshold: 0, // $0 - $10,000
            },
            ProtocolFee {
                maker_fee: 20,                    // 0.20% maker fee
                taker_fee: 35,                    // 0.35% taker fee
                volume_threshold: 10_000_000_000, // $10,001 - $50,000
            },
            ProtocolFee {
                maker_fee: 14,                    // 0.14% maker fee
                taker_fee: 24,                    // 0.24% taker fee
                volume_threshold: 50_000_000_000, // $50,001 - $100,000
            },
            ProtocolFee {
                maker_fee: 12,                     // 0.12% maker fee
                taker_fee: 22,                     // 0.22% taker fee
                volume_threshold: 100_000_000_000, // $100,001 - $250,000
            },
            ProtocolFee {
                maker_fee: 10,                     // 0.10% maker fee
                taker_fee: 20,                     // 0.20% taker fee
                volume_threshold: 250_000_000_000, // $250,001 - $500,000
            },
            ProtocolFee {
                maker_fee: 8,                      // 0.08% maker fee
                taker_fee: 18,                     // 0.18% taker fee
                volume_threshold: 500_000_000_000, // $500,001 - $1,000,000
            },
            ProtocolFee {
                maker_fee: 6,                        // 0.06% maker fee
                taker_fee: 16,                       // 0.16% taker fee
                volume_threshold: 1_000_000_000_000, // $1,000,001 - $2,500,000
            },
            ProtocolFee {
                maker_fee: 4,                        // 0.04% maker fee
                taker_fee: 14,                       // 0.14% taker fee
                volume_threshold: 2_500_000_000_000, // $2,500,001 - $5,000,000
            },
            ProtocolFee {
                maker_fee: 2,                        // 0.02% maker fee
                taker_fee: 12,                       // 0.12% taker fee
                volume_threshold: 5_000_000_000_000, // $5,000,001 - $10,000,000
            },
            ProtocolFee {
                maker_fee: 0,                         // 0.00% maker fee
                taker_fee: 10,                        // 0.10% taker fee
                volume_threshold: 10_000_000_000_000, // $10,000,001+
            },
        ];

        let epoch = 4611686020155120000; // 10/01/2024
        let epoch_duration = 2510000; // 30 days
        let mut markets = vec![];

        // 1. Deploy BTC/USDC market contract
        let base_asset = AssetId::from_str(&btc).unwrap();
        let base_decimals = 8;
        let min_size = 1_500; // 0.000015 BTC

        // Initial balance prior to contract call
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
        let _ = contract.set_min_order_size(min_size).await?;

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
        let min_size = 400_000; // 0.0004 ETH

        // Initial balance prior to contract call
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
        let _ = contract.set_min_order_size(min_size).await?;

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

        // Last: Deploy Registry contract
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
