use fuels::{
    prelude::{launch_custom_provider_and_get_wallets, Address, WalletUnlocked, WalletsConfig},
    types::{ContractId, Identity},
};

use spark_proxy_sdk::SparkProxyContract;

pub(crate) struct User {
    pub(crate) wallet: WalletUnlocked,
}

impl User {
    pub(crate) fn address(&self) -> Address {
        Address::from(self.wallet.address())
    }

    pub(crate) fn identity(&self) -> Identity {
        Identity::Address(self.address())
    }
}

pub(crate) async fn setup_proxy() -> anyhow::Result<(SparkProxyContract, User, User)> {
    let number_of_wallets = 4;
    let num_coins = 1;
    let coin_amount = 1_000_000_000_000;

    let config = WalletsConfig::new(Some(number_of_wallets), Some(num_coins), Some(coin_amount));

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await?;
    let owner = wallets.pop().unwrap();
    let user = wallets.pop().unwrap();

    let contract = SparkProxyContract::deploy(ContractId::zeroed(), owner.clone()).await?;

    let owner = User { wallet: owner };
    let user = User { wallet: user };

    Ok((contract, owner, user))
}
