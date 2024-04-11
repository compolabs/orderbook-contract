use crate::utils::{setup, validate_contract_id, AccountType};
use clap::Args;
use fuels::{
    accounts::ViewOnlyAccount,
    types::{Address, AssetId, ContractId, Identity},
};
use spark_market_sdk::MarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Change the global or user fee for a market")]
pub(crate) struct SetFeeCommand {
    /// The fee to set
    #[clap(long)]
    pub(crate) amount: u64,

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

impl SetFeeCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Initial balance prior to contract call - used to calculate contract interaction cost
        let balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        // TODO: force account_id and account_type to be provided together
        let account = match &self.account_type {
            Some(account) => match account {
                AccountType::Address => {
                    let address = Address::from_str(self.account_id.as_ref().unwrap())
                        .expect("Invalid address");
                    Some(Identity::Address(address))
                }
                AccountType::Contract => {
                    let address = ContractId::from_str(self.account_id.as_ref().unwrap())
                        .expect("Invalid contract id");
                    Some(Identity::ContractId(address))
                }
            },
            None => None,
        };

        let _ = contract.set_fee(self.amount, account.clone()).await?;

        // Balance post-deployment
        let new_balance = wallet.get_asset_balance(&AssetId::BASE).await?;

        // TODO: replace println with tracing
        match self.account_type {
            Some(_) => println!(
                "\nThe user fee has been set to: {} for {:?}",
                self.amount, account
            ),
            None => println!("\nThe global fee has been set to: {}", self.amount),
        };
        println!("Contract call cost: {}", balance - new_balance);

        Ok(())
    }
}
