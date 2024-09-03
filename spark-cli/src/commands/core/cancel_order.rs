use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::Bits256};
use spark_market_sdk::MarketContract;

#[derive(Args, Clone)]
#[command(about = "Cancels an open order")]
pub(crate) struct CancelCommand {
    /// The b256 id of the order
    #[clap(long)]
    pub(crate) order_id: String,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl CancelCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;
        let order_id = Bits256::from_hex_str(&self.order_id)?;

        if self.order_id.len() as u64 != 64 {
            anyhow::bail!("Invalid order id length");
        }

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let _ = contract.cancel_order(order_id).await?;

        // Balance post-call
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("\nContract call cost: {}", balance - new_balance);

        Ok(())
    }
}
