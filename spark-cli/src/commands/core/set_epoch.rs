use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::MarketContract;

#[derive(Args, Clone)]
#[command(about = "Change the epoch and epoch duration for the market")]
pub(crate) struct SetEpochCommand {
    /// The epoch to set
    #[clap(long)]
    pub(crate) epoch: u64,

    /// The epoch duration to set
    #[clap(long)]
    pub(crate) epoch_duration: u64,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl SetEpochCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let _ = contract.set_epoch(self.epoch, self.epoch_duration).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nThe epoch and duration have been set to: {}, {}",
            self.epoch, self.epoch_duration
        );
        println!("Contract call cost: {}", balance - new_balance);

        Ok(())
    }
}
