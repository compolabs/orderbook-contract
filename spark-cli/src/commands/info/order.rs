use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::types::Bits256;
use spark_market_sdk::SparkMarketContract;

#[derive(Args, Clone)]
#[command(about = "Query the market for information about a specific open order")]
pub(crate) struct OrderCommand {
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

impl OrderCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;
        let order_id = Bits256::from_hex_str(&self.order_id)?;

        if self.order_id.len() as u64 != 64 {
            anyhow::bail!("Invalid order id length");
        }

        // Connect to the deployed contract via the rpc
        let contract = SparkMarketContract::new(contract_id, wallet).await;

        let order = contract.order(order_id).await?.value;

        match order {
            Some(order) => {
                println!("{:#?}", order);
            }
            None => println!("No order found for id: {}", self.order_id),
        }

        Ok(())
    }
}
