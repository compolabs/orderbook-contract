use crate::utils::{setup, validate_contract_id, AccountType, OrderType};
use clap::Args;
use fuels::types::{Address, ContractId, Identity};
use spark_market_sdk::{AssetType, MarketContract, OrderType as ContractOrderType};
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Create a sha256 hash (order id) of the provided information")]
pub(crate) struct OrderIdCommand {
    /// The id of the asset
    #[clap(long)]
    pub(crate) asset_type: String,

    /// The type of order
    #[clap(long)]
    pub(crate) order_type: OrderType,

    /// The b256 id of the account
    #[clap(long)]
    pub(crate) owner: String,

    /// The type of account
    #[clap(long)]
    pub(crate) account_type: AccountType,

    /// The price of the order
    #[clap(long)]
    pub(crate) price: u64,

    /// The price of the order
    #[clap(long)]
    pub(crate) block_height: u32,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl OrderIdCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        let asset_type = match self.asset_type.as_str() {
            "base" => AssetType::Base,
            "quote" => AssetType::Quote,
            _ => anyhow::bail!("Invalid asset type [base|quote]"),
        };

        // TODO: cli parsing
        let order_type = match self.order_type {
            OrderType::Buy => ContractOrderType::Buy,
            OrderType::Sell => ContractOrderType::Sell,
        };

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet).await;

        let account = match &self.account_type {
            AccountType::Address => {
                let address = Address::from_str(&self.owner).expect("Invalid address");
                Identity::Address(address)
            }
            AccountType::Contract => {
                let address = ContractId::from_str(&self.owner).expect("Invalid contract id");
                Identity::ContractId(address)
            }
        };

        let hash = contract
            .order_id(
                asset_type,
                order_type,
                account,
                self.price,
                self.block_height,
            )
            .await?
            .value;

        // TODO: replace println with tracing
        // TODO: hack to display, turn into hex manually?
        println!("\nOrder ID: {}", ContractId::from(hash.0));

        Ok(())
    }
}
