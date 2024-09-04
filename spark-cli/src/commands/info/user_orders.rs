use crate::utils::{setup, validate_contract_id, AccountType};
use clap::Args;
use fuels::types::{Address, ContractId, Identity};
use spark_market_sdk::SparkMarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Query the market for the currently open orders for the user")]
pub(crate) struct UserOrdersCommand {
    /// The b256 id of the account
    #[clap(long)]
    pub(crate) account_id: String,

    /// The type of account
    #[clap(long)]
    pub(crate) account_type: AccountType,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl UserOrdersCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = SparkMarketContract::new(contract_id, wallet).await;

        let account = match self.account_type {
            AccountType::Address => {
                let address = Address::from_str(&self.account_id).expect("Invalid address");
                Identity::Address(address)
            }
            AccountType::Contract => {
                let address = ContractId::from_str(&self.account_id).expect("Invalid contract id");
                Identity::ContractId(address)
            }
        };

        let orders = contract.user_orders(account).await?.value;

        if orders.is_empty() {
            anyhow::bail!("User has no open orders");
        }

        for order in orders {
            println!("{:?}", Address::new(order.0));
        }

        Ok(())
    }
}
