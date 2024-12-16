use crate::utils::setup;
use clap::Args;
use fuels::{
    accounts::ViewOnlyAccount,
    types::{AssetId, ContractId},
};
use spark_market_sdk::{ProtocolFee, SparkMarketContract};
use spark_proxy_sdk::{SparkProxyContract, State};
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Deploys the fuel/usdc market proxy to a network")]
pub(crate) struct DeployFuelUsdcProxyCommand {}

impl DeployFuelUsdcProxyCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup("mainnet.fuel.network").await?;

        let (fuel, usdc) = (
            "0x1d5d97005e41cae2187a895fd8eab0506111e0e2f3331cd3912c15c24e3c1d82",
            "0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b",
        );

        let base_asset = AssetId::from_str(&fuel).unwrap();
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

        let epoch = 4611686020160400000; // 12/01/2024
        let epoch_duration = 2600000; // 30 days
        let min_price = 500_000_000; // 0.5 USDC
        let min_size = 1_000_000; // 0.001 FUEL
        let matcher_fee = 1_000; // 0.001 USDC

        let _ = market.set_epoch(epoch, epoch_duration).await?;
        let _ = market.set_min_order_size(min_size).await?;
        let _ = market.set_min_order_price(min_price).await?;
        let _ = market.set_matcher_fee(matcher_fee).await?;

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

        let _ = market.set_protocol_fee(protocol_fee.clone()).await?;

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
