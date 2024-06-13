use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::AssetId};
use spark_market_sdk::MarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Deposits an asset from the wallet to the market")]
pub(crate) struct WithdrawCommand {
    /// The amount to withdraw
    #[clap(long)]
    pub(crate) amount: u64,

    /// The asset id for the asset of the market
    #[clap(long)]
    pub(crate) asset: String,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl WithdrawCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        if self.asset.len() as u64 != 66 {
            anyhow::bail!("Invalid asset length");
        }

        let asset = AssetId::from_str(&self.asset).expect("Invalid asset");

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet.get_asset_balance(&AssetId::BASE).await?;
        let asset_balance = wallet.get_asset_balance(&asset).await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let r = contract.withdraw(self.amount, asset).await?;
        // TODO: sdk debugging
        dbg!(r);

        // Balance post-call
        let new_balance = wallet.get_asset_balance(&AssetId::BASE).await?;
        let new_asset_balance = wallet.get_asset_balance(&asset).await?;

        // TODO: replace println with tracing
        println!("Contract call cost: {}", balance - new_balance);
        println!(
            "Withdrawn {} amount of {} asset",
            new_asset_balance - asset_balance,
            self.asset
        );

        Ok(())
    }
}
