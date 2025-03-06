use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::SparkMarketContract;
use spark_proxy_sdk::SparkProxyContract;

#[derive(Args, Clone)]
#[command(about = "Upgrades the usdc/usdt market proxy to a network")]
pub(crate) struct UpgradeUsdcUsdtProxyCommand {}

impl UpgradeUsdcUsdtProxyCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup("mainnet.fuel.network").await?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        let contract_id_str = "0xdafe498b31f24ea5577055e86bf77e96bcba2c39a7ae47abaa819c303a45a352";
        let contract_id = validate_contract_id(contract_id_str)?;

        // Connect to the deployed contract via the rpc
        let proxy = SparkProxyContract::new(contract_id, wallet.clone()).await;
        let proxy_target = proxy.proxy_target().await?.value;

        let proxy = SparkMarketContract::new(contract_id, wallet.clone()).await;

        let (
            base_asset,
            base_asset_decimals,
            quote_asset,
            quote_asset_decimals,
            owner,
            price_decimals,
            version,
        ) = proxy.config().await?.value;

        println!("\nProxy target: {:?}", proxy_target);
        println!("Base Asset: 0x{}", base_asset);
        println!("Base Asset Decimals: {}", base_asset_decimals);
        println!("Quote Asset: 0x{}", quote_asset);
        println!("Quote Asset Decimals: {}", quote_asset_decimals);
        println!("Owner: {:?}", owner);
        println!("Price Decimals: {}", price_decimals);
        println!(
            "Version: {}.{}.{}",
            (version & 0xFF0000) >> 16,
            (version & 0xFF00) >> 8,
            version & 0xFF
        );

        let version = SparkMarketContract::sdk_version();

        // Deploy the contract
        let contract = SparkMarketContract::deploy(
            base_asset,
            base_asset_decimals,
            quote_asset,
            quote_asset_decimals,
            wallet.clone(),
            price_decimals,
            version,
        )
        .await?;

        let _ = contract.pause().await?;

        println!("\nNew target deployed: {:?}", contract.contract_id());

        let proxy = SparkProxyContract::new(contract_id, wallet.clone()).await;
        let _ = proxy
            .set_proxy_target(contract.contract_id().into())
            .await?;
        let proxy_target = proxy.proxy_target().await?.value;

        let proxy = SparkMarketContract::new(contract_id, wallet.clone()).await;

        let (
            base_asset,
            base_asset_decimals,
            quote_asset,
            quote_asset_decimals,
            owner,
            price_decimals,
            version,
        ) = proxy.config().await?.value;

        println!("\nNew proxy target: {:?}", proxy_target);
        println!("Base Asset: 0x{}", base_asset);
        println!("Base Asset Decimals: {}", base_asset_decimals);
        println!("Quote Asset: 0x{}", quote_asset);
        println!("Quote Asset Decimals: {}", quote_asset_decimals);
        println!("Owner: {:?}", owner);
        println!("Price Decimals: {}", price_decimals);
        println!(
            "Version: {}.{}.{}",
            (version & 0xFF0000) >> 16,
            (version & 0xFF00) >> 8,
            version & 0xFF
        );

        println!(
            "\nMarket {} upgraded to version {} ({}) with target {:?}",
            contract_id_str,
            contract.contract_str_version().await?,
            version,
            proxy_target,
        );

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("\nDeployment cost: {}", balance - new_balance);
        println!("Owner address: {}", wallet.address());
        println!("               0x{}", wallet.address().hash());

        Ok(())
    }
}
