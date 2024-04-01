use fuels::{
    prelude::{
        abigen, launch_custom_provider_and_get_wallets, Address, AssetId, Contract,
        LoadConfiguration, StorageConfiguration, TxPolicies, WalletUnlocked, WalletsConfig,
    },
    types::Identity,
};

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

pub(crate) struct Asset {
    pub(crate) id: AssetId,
    pub(crate) decimals: u32,
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
) -> (Market<WalletUnlocked>, Identity, Identity, Assets) {
    let number_of_wallets = 2;
    let coins_per_wallet = 1;
    let amount_per_coin = 100_000_000;

    let config = WalletsConfig::new(
        Some(number_of_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );

    let mut wallets = launch_custom_provider_and_get_wallets(config, None, None)
        .await
        .unwrap();

    let owner = wallets.pop().unwrap();
    let user = wallets.pop().unwrap();
    let assets = Assets {
        base: Asset {
            id: AssetId::new([0; 32]),
            decimals: base_decimals,
        },
        quote: Asset {
            id: AssetId::new([1; 32]),
            decimals: quote_decimals,
        },
        random: Asset {
            id: AssetId::new([2; 32]),
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

    (
        market,
        Identity::Address(Address::from(owner.address())),
        Identity::Address(Address::from(user.address())),
        assets,
    )
}
