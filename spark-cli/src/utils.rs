use clap::ValueEnum;
use fuels::prelude::{ContractId, Provider, WalletUnlocked};
use std::str::FromStr;

pub(crate) async fn setup(rpc: &str) -> anyhow::Result<WalletUnlocked> {
    let provider = Provider::connect(rpc).await?;

    // First, try to get the private key from environment
    if let Ok(secret) = std::env::var("WALLET_SECRET") {
        let wallet = WalletUnlocked::new_from_private_key(secret.parse()?, Some(provider));
        return Ok(wallet);
    }

    // If no private key is provided, try to get the mnemonic phrase from environment
    if let Ok(mnemonic) = std::env::var("MNEMONIC") {
        let wallet = WalletUnlocked::new_from_mnemonic_phrase(&mnemonic, Some(provider.clone()))?;
        return Ok(wallet);
    }

    // If neither WALLET_SECRET nor MNEMONIC are provided, return an error
    Err(anyhow::anyhow!(
        "No valid private key or mnemonic found in environment"
    ))
}

pub(crate) fn validate_contract_id(contract_id: &str) -> anyhow::Result<ContractId> {
    if contract_id.len() as u64 != 66 {
        anyhow::bail!("Invalid contract id length");
    }

    Ok(ContractId::from_str(contract_id).expect("Invalid contract id"))
}

#[derive(Clone, ValueEnum)]
pub(crate) enum AccountType {
    /// Externally Owned Account
    Address,
    /// Contract
    Contract,
}

#[derive(Clone, ValueEnum)]
pub(crate) enum AssetType {
    /// Base asset
    Base,
    /// Quote asset
    Quote,
}

#[derive(Clone, ValueEnum)]
pub(crate) enum LimitType {
    /// Immediatelly or Cancel
    IOC,
    /// Fill or Kill
    FOK,
}

#[derive(Clone, ValueEnum)]
pub(crate) enum OrderType {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}
