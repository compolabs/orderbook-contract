use crate::utils::{setup, validate_contract_id, AccountType};
use clap::Args;
use fuels::types::{Address, ContractId, Identity};
use spark_market_sdk::SparkMarketContract;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(about = "Query the protocol fee user")]
pub(crate) struct ProtocolFeeUserCommand {
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

impl ProtocolFeeUserCommand {
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

        let protocol_fee_user = contract.protocol_fee_user(account).await?.value;

        println!(
            "Protocol Fee: for {:?} (maker_fee, taker_fee) ({}, {})",
            account, protocol_fee_user.0, protocol_fee_user.1
        );

        Ok(())
    }
}
