use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use spark_market_sdk::SparkMarketContract;

#[derive(Args, Clone)]
#[command(about = "Change the min order price for the market")]
pub(crate) struct SetMinOrderPriceCommand {
    /// The fee to set
    #[clap(long)]
    pub(crate) price: u64,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl SetMinOrderPriceCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = SparkMarketContract::new(contract_id, wallet.clone()).await;

        let _ = contract.set_min_order_price(self.price).await?;

        // Balance post-deployment
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!(
            "\nThe min order price has been set to: {}",
            contract.min_order_price().await?.value
        );
        println!("Contract call cost: {}", balance - new_balance);

        Ok(())
    }
}
