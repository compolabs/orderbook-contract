use crate::utils::{setup, validate_contract_id, AssetType};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::{AssetType as ContractAssetType, MarketContract};

#[derive(Args, Clone)]
#[command(about = "Deposits an asset from the wallet to the market")]
pub(crate) struct DepositCommand {
    /// The amount to deposit
    #[clap(long)]
    pub(crate) amount: u64,

    /// The asset type of the market
    #[clap(long)]
    pub(crate) asset_type: AssetType,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl DepositCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        let asset_type = match self.asset_type {
            AssetType::Base => ContractAssetType::Base,
            AssetType::Quote => ContractAssetType::Quote,
        };

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let config = contract.config().await?.value;
        let asset = if asset_type == ContractAssetType::Base {
            config.0
        } else {
            config.2
        };
        let asset_balance = wallet.get_asset_balance(&asset).await?;

        let _ = contract.deposit(self.amount, asset).await?;

        // Balance post-call
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;
        let new_asset_balance = wallet.get_asset_balance(&asset).await?;

        println!("\nContract call cost: {}", balance - new_balance);
        println!(
            "Deposited {} amount of asset {}",
            self.amount,
            asset_balance - new_asset_balance
        );

        Ok(())
    }
}
