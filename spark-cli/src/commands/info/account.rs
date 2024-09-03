use crate::utils::{setup, validate_contract_id, AccountType};
use clap::Args;
use fuels::accounts::ViewOnlyAccount;
use fuels::types::{Address, ContractId, Identity};
use spark_market_sdk::MarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Query the account info for a user")]
pub(crate) struct AccountCommand {
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

impl AccountCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet.clone()).await;

        let account = match self.account_type {
            AccountType::Address => {
                let address = Address::from_str(&self.account_id).expect("Invalid address");
                contract.account(Identity::Address(address)).await?.value
            }
            AccountType::Contract => {
                let address = ContractId::from_str(&self.account_id).expect("Invalid contract id");
                contract.account(Identity::ContractId(address)).await?.value
            }
        };

        let balance = wallet
            .get_asset_balance(&wallet.provider().unwrap().base_asset_id())
            .await?;
        println!("\nContract base asset balance: {}", balance);

        println!("\n{:#?}", account);

        Ok(())
    }
}
