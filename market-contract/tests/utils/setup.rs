use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Address, AssetConfig, AssetId, Contract,
        LoadConfiguration, StorageConfiguration, TxPolicies, WalletUnlocked, WalletsConfig,
    },
    types::Identity,
};

// PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("market-contract/out/debug/market-contract.bin");
const MARKET_CONTRACT_BINARY_PATH: &str = "./out/debug/market-contract.bin";
const MARKET_CONTRACT_STORAGE_PATH: &str =
    "./out/debug/market-contract-storage_slots.json";
abigen!(Contract(
    name = "Market",
    abi = "./market-contract/out/debug/market-contract-abi.json"
));

pub(crate) struct Assets {
    pub(crate) base: Asset,
    pub(crate) quote: Asset,
    pub(crate) random: Asset,
}

impl Asset  {
    pub fn parse_units(&self, value: f64) -> f64 {
        value * 10_f64.powf(self.decimals as f64)
    }
    pub fn format_units(&self, value: f64) -> f64 {
        value / 10_f64.powf(self.decimals as f64)
    }
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
            base_decimals: 9,
            quote_decimals: 9,
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
) -> (Market<WalletUnlocked>, User, User, Assets) {
    let number_of_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 100_000_000_000_000;

    let base_asset_id = AssetId::new([0; 32]);
    let quote_asset_id = AssetId::new([1; 32]);
    let random_asset_id = AssetId::new([2; 32]);

    let base_asset = AssetConfig {
        id: base_asset_id,
        num_coins: coins_per_wallet,
        coin_amount: amount_per_coin,
    };
    let quote_asset = AssetConfig {
        id: quote_asset_id,
        num_coins: coins_per_wallet,
        coin_amount: amount_per_coin,
    };
    let random_asset = AssetConfig {
        id: random_asset_id,
        num_coins: coins_per_wallet,
        coin_amount: amount_per_coin,
    };
    let assets = vec![base_asset, quote_asset, random_asset];

    let config = WalletsConfig::new_multiple_assets(number_of_wallets, assets);

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None)
        .await
        .unwrap();

    let owner = wallets.pop().unwrap();
    let user = wallets.pop().unwrap();
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
    };

    let storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(MARKET_CONTRACT_STORAGE_PATH);

    let configurables = MarketConfigurables::default()
        .with_BASE_ASSET(assets.base.id.clone())
        .unwrap()
        .with_BASE_ASSET_DECIMALS(assets.base.decimals)
        .unwrap()
        .with_QUOTE_ASSET(assets.quote.id.clone())
        .unwrap()
        .with_QUOTE_ASSET_DECIMALS(assets.quote.decimals)
        .unwrap()
        .with_PRICE_DECIMALS(price_decimals)
        .unwrap()
        .with_OWNER(owner.address().into())
        .unwrap();

    let contract_configuration = LoadConfiguration::default()
        .with_storage_configuration(storage_configuration.unwrap())
        .with_configurables(configurables);

    let contract_id = Contract::load_from(MARKET_CONTRACT_BINARY_PATH, contract_configuration)
        .unwrap()
        .deploy(&owner, TxPolicies::default())
        .await
        .unwrap();

    let market = Market::new(contract_id.clone(), owner.clone());
    let owner = User { wallet: owner };
    let non_owner = User { wallet: user };

    (market, owner, non_owner, assets)
}
