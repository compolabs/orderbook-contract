use crate::utils::{setup, validate_contract_id};
use clap::Args;
use fuels::{
    accounts::ViewOnlyAccount,
    types::{AssetId, Bits256},
};
use spark_market_sdk::MarketContract;

#[derive(Args, Clone)]
#[command(about = "Matches a pair of orders")]
pub(crate) struct MatchPairCommand {
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

impl MatchPairCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        if self.orders.len() != 2 {
            anyhow::bail!("Invalid order array length <> 2");
        }

        let mut order_ids: Vec<Bits256> = Vec::new();
        for order in self.orders.clone() {
            order_ids.push(Bits256::from_hex_str(&order).expect("Invalid order_id"));
        }

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let _ = contract
            .match_order_pair(order_ids[0], order_ids[1])
            .await?;

        // Balance post-call
        let new_balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        println!(
            "Order pair matched: {} : {}",
            self.orders[0], self.orders[1]
        );

        // TODO: replace println with tracing
        println!("Contract call cost: {}", balance - new_balance);

        Ok(())
    }
}
