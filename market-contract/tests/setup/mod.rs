use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        launch_custom_provider_and_get_wallets, Address, AssetConfig, AssetId, WalletUnlocked,
        WalletsConfig,
    },
    types::Identity,
};
use spark_market_sdk::{Account, Balance, MarketContract};

pub(crate) struct Assets {
    pub(crate) base: Asset,
    pub(crate) quote: Asset,
    pub(crate) random: Asset,
    pub(crate) fuel: Asset,
}

pub(crate) struct Asset {
    pub(crate) id: AssetId,
    pub(crate) decimals: u32,
}

pub(crate) struct Defaults {
    pub(crate) base_decimals: u32,
    pub(crate) quote_decimals: u32,
    pub(crate) price_decimals: u32,
}

impl Defaults {
    pub(crate) fn default() -> Self {
        Self {
            base_decimals: 8,
            quote_decimals: 6,
            price_decimals: 9,
        }
    }
}

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

pub(crate) fn create_account(
    liquid_base: u64,
    liquid_quote: u64,
    locked_base: u64,
    locked_quote: u64,
) -> Account {
    Account {
        liquid: Balance {
            base: liquid_base,
            quote: liquid_quote,
        },
        locked: Balance {
            base: locked_base,
            quote: locked_quote,
        },
    }
}

pub(crate) async fn setup(
    base_decimals: u32,
    quote_decimals: u32,
    price_decimals: u32,
) -> anyhow::Result<(MarketContract, User, User, User, User, Assets)> {
    let number_of_wallets = 4;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000_000_000;

    let fuel_asset_id = AssetId::new([0; 32]);
    let base_asset_id = AssetId::new([1; 32]);
    let quote_asset_id = AssetId::new([2; 32]);
    let random_asset_id = AssetId::new([3; 32]);

    let ids = vec![
        fuel_asset_id,
        base_asset_id,
        quote_asset_id,
        random_asset_id,
    ];
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
    let user0 = wallets.pop().unwrap();
    let user1 = wallets.pop().unwrap();
    let matcher = wallets.pop().unwrap();

    let assets = Assets {
        base: Asset {
            id: base_asset_id,
            decimals: base_decimals,
        },
        quote: Asset {
            id: quote_asset_id,
            decimals: quote_decimals,
        },
        random: Asset {
            id: random_asset_id,
            decimals: 10,
        },
        fuel: Asset {
            id: fuel_asset_id,
            decimals: 9,
        },
    };

    let contract = MarketContract::deploy(
        assets.base.id,
        assets.base.decimals,
        assets.quote.id,
        assets.quote.decimals,
        owner.clone(),
        price_decimals,
        0xFAFBFC,
    )
    .await?;

    let owner = User { wallet: owner };
    let user0 = User { wallet: user0 };
    let user1 = User { wallet: user1 };
    let matcher = User { wallet: matcher };

    Ok((contract, owner, user0, user1, matcher, assets))
}
