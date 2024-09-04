use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::Bits256};
use spark_market_sdk::SparkMarketContract;

#[derive(Args, Clone)]
#[command(about = "Matches many orders")]
pub(crate) struct MatchManyCommand {
    /// The b256 id of the order
    #[clap(long)]
    pub(crate) orders: Vec<String>,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl MatchManyCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        if self.orders.len() < 2 {
            anyhow::bail!("Invalid order array length < 2");
        }

        let mut order_ids: Vec<Bits256> = Vec::new();
        for order in self.orders.clone() {
            order_ids.push(Bits256::from_hex_str(&order).expect("Invalid order_id"));
        }

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = SparkMarketContract::new(contract_id, wallet.clone()).await;

        let _ = contract.match_order_many(order_ids).await?;

        // Balance post-call
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("Orders matched: {:?}", self.orders,);

        // TODO: replace println with tracing
        println!("Contract call cost: {}", balance - new_balance);

        Ok(())
    }
}
