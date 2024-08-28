use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        launch_custom_provider_and_get_wallets, Address, AssetConfig, AssetId,
        WalletUnlocked, WalletsConfig,
    },
    types::Identity,
};
use spark_registry_sdk::OrderbookContract;

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

    pub(crate) async fn balance(&self, asset: &AssetId) -> u64 {
        self.wallet.get_asset_balance(asset).await.unwrap()
    }
}

pub(crate) fn random_asset_id(random: u8) -> AssetId {
    AssetId::new([random; 32])
}

pub(crate) async fn setup() -> anyhow::Result<(OrderbookContract, User, User)> {
    let number_of_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 100_000_000;

    let base_asset_id = AssetId::new([0; 32]);
    let quote_asset_id = AssetId::new([1; 32]);
    let random_asset_id = AssetId::new([2; 32]);

    let ids = vec![base_asset_id, quote_asset_id, random_asset_id];
    let mut assets: Vec<AssetConfig> = Vec::with_capacity(3);

    for id in ids {
        assets.push(AssetConfig {
            id,
            num_coins: coins_per_wallet,
            coin_amount: amount_per_coin,
        });
    }
    let config = WalletsConfig::new_multiple_assets(number_of_wallets, assets);

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None).await?;
    let owner = wallets.pop().unwrap();
    let user = wallets.pop().unwrap();

    let contract = OrderbookContract::deploy(owner.clone(), 0xFAFBFC).await?;

    let owner = User { wallet: owner };
    let non_owner = User { wallet: user };

    Ok((contract, owner, non_owner))
}
