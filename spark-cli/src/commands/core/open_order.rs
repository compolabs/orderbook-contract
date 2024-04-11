use crate::utils::{setup, validate_contract_id, OrderType};
use clap::Args;
use fuels::{
    accounts::ViewOnlyAccount,
    types::{AssetId, ContractId},
};
use spark_market_sdk::{MarketContract, OrderType as ContractOrderType};
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Opens a new order")]
pub(crate) struct OpenCommand {
    /// The amount of asset
    #[clap(long)]
    pub(crate) amount: u64,

    /// The id of the asset
    #[clap(long)]
    pub(crate) asset: String,

    /// The type of order
    #[clap(long)]
    pub(crate) order_type: OrderType,

    /// The price of the order
    #[clap(long)]
    pub(crate) price: u64,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. beta-5.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl OpenCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        if self.asset.len() as u64 != 66 {
            anyhow::bail!("Invalid asset length");
        }

        let asset = AssetId::from_str(&self.asset).expect("Invalid asset");

        // TODO: cli parsing
        let order_type = match self.order_type {
            OrderType::Buy => ContractOrderType::Buy,
            OrderType::Sell => ContractOrderType::Sell,
        };

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let order_id = contract
            .open_order(self.amount, asset, order_type, self.price)
            .await?
            .value;

        // Balance post-call
        let new_balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // TODO: replace println with tracing
        println!("\nContract call cost: {}", balance - new_balance);
        // TODO: hack to display, turn into hex manually?
        println!("Order ID: {}", ContractId::from(order_id.0));

        Ok(())
    }
}
