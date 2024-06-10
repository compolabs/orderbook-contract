use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::{
    accounts::ViewOnlyAccount,
    types::{AssetId, Bits256},
};
use spark_market_sdk::MarketContract;

#[derive(Args, Clone)]
#[command(about = "Matches orders")]
pub(crate) struct BatchCommand {
    /// The b256 id of the order
    #[clap(long)]
    pub(crate) order_id: String,

    /// The b256 ids for orders to match against `order_id`
    #[clap(long)]
    pub(crate) order_ids: Vec<String>,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl BatchCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        let order_id = Bits256::from_hex_str(&self.order_id)?;

        if self.order_id.len() as u64 != 64 {
            anyhow::bail!("Invalid order id length");
        }

        if self.order_ids.is_empty() {
            anyhow::bail!("At least one order ID must be added to the list of ids");
        }

        let mut order_ids: Vec<Bits256> = Vec::with_capacity(self.order_ids.len());
        for id in self.order_ids.iter() {
            if id.len() as u64 != 64 {
                anyhow::bail!("Invalid order id length: {}", id);
            }

            order_ids.push(Bits256::from_hex_str(id)?);
        }

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let _ = contract.batch_fulfill(order_id, order_ids).await?;

        // Balance post-call
        let new_balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // TODO: replace println with tracing
        println!("\nContract call cost: {}", balance - new_balance);
        // TODO: adjust contract to inform which orders have not been fulfilled and report here?
        //       this could be via a return value of incomplete orders

        Ok(())
    }
}
