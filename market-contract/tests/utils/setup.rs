use fuels::prelude::{
    abigen, launch_custom_provider_and_get_wallets, Contract, LoadConfiguration,
    StorageConfiguration, TxPolicies, WalletUnlocked, WalletsConfig,
};

abigen!(Contract(
    name = "Market",
    abi = "./out/debug/market-contract-abi.json"
));

const MARKET_CONTRACT_BINARY_PATH: &str = "./out/debug/market-contract.bin";
const MARKET_CONTRACT_STORAGE_PATH: &str = "./out/debug/market-contract-storage_slots.json";

pub(crate) async fn setup() -> Market<WalletUnlocked> {
    let number_of_wallets = 1;
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

    let deployer_wallet = wallets.pop().unwrap();

    let storage_configuration =
        StorageConfiguration::default().add_slot_overrides_from_file(MARKET_CONTRACT_STORAGE_PATH);

    // TODO: set configuration
    let contract_configuration =
        LoadConfiguration::default().with_storage_configuration(storage_configuration.unwrap());

    let contract_id = Contract::load_from(MARKET_CONTRACT_BINARY_PATH, contract_configuration)
        .unwrap()
        .deploy(&deployer_wallet, TxPolicies::default())
        .await
        .unwrap();

    let market = Market::new(contract_id.clone(), deployer_wallet.clone());

    market
}
