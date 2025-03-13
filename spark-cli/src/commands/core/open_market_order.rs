use crate::utils::{setup, validate_contract_id, /*AssetType,*/ OrderType};
use clap::Args;
use fuels::{accounts::ViewOnlyAccount, types::ContractId};
use spark_market_sdk::{
    OrderType as ContractOrderType, /*AssetType as ContractAssetType,*/ SparkMarketContract,
};

#[derive(Args, Clone)]
#[command(about = "Opens a new order")]
pub(crate) struct OpenMarketCommand {
    /// The amount of asset
    #[clap(long)]
    pub(crate) amount: u64,

    /// The asset type of the market
    /*#[clap(long)]
    pub(crate) asset_type: AssetType,*/

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
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl OpenMarketCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        let order_type = match self.order_type {
            OrderType::Buy => ContractOrderType::Buy,
            OrderType::Sell => ContractOrderType::Sell,
        };

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        // Connect to the deployed contract via the rpc
        let contract = SparkMarketContract::new(contract_id, wallet.clone()).await;

        let order_id = contract
            .open_market_order(self.amount, order_type.clone(), self.price)
            .await?
            .value;

        // Balance post-call
        let new_balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;

        println!("\nContract call cost: {}", balance - new_balance);
        println!("Order ID: {}", ContractId::from(order_id.0));

        Ok(())
    }
}
