use crate::utils::{setup, validate_contract_id, AccountType};
use clap::Args;
use fuels::types::{Address, ContractId, Identity};
use spark_market_sdk::MarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Query the global or user fee")]
pub(crate) struct FeeCommand {
    /// The b256 id of the account
    #[clap(long)]
    pub(crate) account_id: Option<String>,

    /// The type of account
    #[clap(long)]
    pub(crate) account_type: Option<AccountType>,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. beta-5.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl FeeCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet).await;

        // TODO: force account_id and account_type to be provided together
        let account = match &self.account_type {
            Some(account) => match account {
                AccountType::Address => {
                    let address = Address::from_str(&self.account_id.as_ref().unwrap())
                        .expect("Invalid address");
                    Some(Identity::Address(address))
                }
                AccountType::Contract => {
                    let address = ContractId::from_str(&self.account_id.as_ref().unwrap())
                        .expect("Invalid contract id");
                    Some(Identity::ContractId(address))
                }
            },
            None => None,
        };

        let fee = contract.fee(account).await?.value;

        // TODO: replace println with tracing
        match self.account_type {
            Some(_) => println!("\nUser Fee: {}", fee),
            None => println!("\nGlobal Fee: {}", fee),
        };

        Ok(())
    }
}
