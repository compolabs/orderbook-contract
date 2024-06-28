use crate::utils::{setup, validate_contract_id, AssetType, OrderType};
use clap::Args;
use fuels::{
    accounts::ViewOnlyAccount,
    types::{AssetId, Bits256, ContractId},
};
use spark_market_sdk::{
    AssetType as ContractAssetType, MarketContract, OrderType as ContractOrderType,
};

#[derive(Args, Clone)]
#[command(about = "Fulfill a new order")]
pub(crate) struct FulfillManyCommand {
    /// The amount of asset
    #[clap(long)]
    pub(crate) amount: u64,

    /// The asset type of the market
    #[clap(long)]
    pub(crate) asset_type: AssetType,

    /// The type of order
    #[clap(long)]
    pub(crate) order_type: OrderType,

    /// The price of the order
    #[clap(long)]
    pub(crate) price: u64,

    /// The slippage of the order price
    #[clap(long)]
    pub(crate) slippage: u64,

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

impl FulfillManyCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        if self.orders.len() == 0 {
            anyhow::bail!("Invalid order array length == 0");
        }

        let mut order_ids: Vec<Bits256> = Vec::new();
        for order in self.orders.clone() {
            order_ids.push(Bits256::from_hex_str(&order).expect("Invalid order_id"));
        }

        // TODO: cli parsing
        let order_type = match self.order_type {
            OrderType::Buy => ContractOrderType::Buy,
            OrderType::Sell => ContractOrderType::Sell,
        };
        let asset_type = match self.asset_type {
            AssetType::Base => ContractAssetType::Base,
            AssetType::Quote => ContractAssetType::Quote,
        };

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let order_id = contract
            .fulfill_many(
                self.amount,
                asset_type.clone(),
                order_type.clone(),
                self.price,
                self.slippage,
                order_ids,
            )
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
