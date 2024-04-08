use fuels::{
    accounts::ViewOnlyAccount,
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Address, AssetConfig, AssetId, Contract,
        LoadConfiguration, StorageConfiguration, TxPolicies, WalletUnlocked, WalletsConfig,
    },
    types::Identity,
};
use src20_sdk::token_utils::{deploy_token_contract, get_symbol_hash, Asset};

abigen!(Contract(
    name = "Market",
    abi = "./out/debug/market-contract-abi.json"
));

const MARKET_CONTRACT_BINARY_PATH: &str = "./out/debug/market-contract.bin";
const MARKET_CONTRACT_STORAGE_PATH: &str = "./out/debug/market-contract-storage_slots.json";

pub(crate) struct Assets {
    pub(crate) base: Asset,
    pub(crate) quote: Asset,
    pub(crate) random: Asset,
}

// pub(crate) struct Asset {
//     pub(crate) id: AssetId,
//     pub(crate) decimals: u32,
// }

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
    let amount_per_coin = 1000_000_000;

    // let base_asset_id = AssetId::new([0; 32]);
    // let quote_asset_id = AssetId::new([1; 32]);
    // let random_asset_id = AssetId::new([2; 32]);

    // let base_asset = AssetConfig {
    //     id: base_asset_id,
    //     num_coins: coins_per_wallet,
    //     coin_amount: amount_per_coin,
    // };
    // let quote_asset = AssetConfig {
    //     id: quote_asset_id,
    //     num_coins: coins_per_wallet,
    //     coin_amount: amount_per_coin,
    // };
    // let random_asset = AssetConfig {
    //     id: random_asset_id,
    //     num_coins: coins_per_wallet,
    //     coin_amount: amount_per_coin,
    // };
    // let assets = vec![base_asset, quote_asset, random_asset];

    // let config = WalletsConfig::new_multiple_assets(number_of_wallets, assets);
    let config = WalletsConfig::new(Some(number_of_wallets), Some(coins_per_wallet), Some(amount_per_coin));

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None)
        .await
        .unwrap();

    let owner = wallets.pop().unwrap();
    let user = wallets.pop().unwrap();

    let token_contract = deploy_token_contract(&owner).await;

    let assets = Assets {
        base: Asset {
            asset_id: token_contract
                .clone()
                .contract_id()
                .asset_id(&get_symbol_hash("BASE")),
            decimals: base_decimals.into(),
            symbol: "BASE".to_string(),
            token_contract_instance: Option::Some(token_contract.clone()),
        },
        quote: Asset {
            asset_id: token_contract
                .clone()
                .contract_id()
                .asset_id(&get_symbol_hash("QUOTE")),
            decimals: quote_decimals.into(),
            symbol: "QUOTE".to_string(),
            token_contract_instance: Option::Some(token_contract.clone()),
        },
        random: Asset {
            asset_id: token_contract
                .clone()
                .contract_id()
                .asset_id(&get_symbol_hash("RANDOM")),
            decimals: 10,
            symbol: "RANDOM".to_string(),
            token_contract_instance: Option::Some(token_contract.clone()),
        },
    };

    let storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(MARKET_CONTRACT_STORAGE_PATH);

    let configurables = MarketConfigurables::default()
        .with_BASE_ASSET(assets.base.asset_id.clone())
        .unwrap()
        .with_BASE_ASSET_DECIMALS(assets.base.decimals.try_into().unwrap())
        .unwrap()
        .with_QUOTE_ASSET(assets.quote.asset_id.clone())
        .unwrap()
        .with_QUOTE_ASSET_DECIMALS(assets.quote.decimals.try_into().unwrap())
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
