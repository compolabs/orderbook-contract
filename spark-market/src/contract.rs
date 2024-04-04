use fuels::{
    prelude::{
        abigen, Address, AssetId, CallParameters, Contract, LoadConfiguration,
        StorageConfiguration, TxPolicies, WalletUnlocked,
    },
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::{Bits256, Identity},
};
use rand::Rng;

abigen!(Contract(
    name = "Market",
    abi = "./market-contract/out/debug/market-contract-abi.json"
));

// TODO: check if paths work
const MARKET_CONTRACT_BINARY_PATH: &str = "./out/debug/market-contract.bin";
const MARKET_CONTRACT_STORAGE_PATH: &str = "./out/debug/market-contract-storage_slots.json";

pub struct MarketContract {
    instance: Market<WalletUnlocked>,
}

impl MarketContract {
    pub async fn new(
        base_asset: AssetId,
        base_decimals: u32,
        quote_asset: AssetId,
        quote_decimals: u32,
        price_decimals: u32,
        owner: WalletUnlocked,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(MARKET_CONTRACT_STORAGE_PATH);

        let configurables = MarketConfigurables::default()
            .with_BASE_ASSET(base_asset)
            .with_BASE_ASSET_DECIMALS(base_decimals)
            .with_QUOTE_ASSET(quote_asset)
            .with_QUOTE_ASSET_DECIMALS(quote_decimals)
            .with_PRICE_DECIMALS(price_decimals)
            .with_OWNER(owner.address().into());

        let contract_configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration.unwrap())
            .with_configurables(configurables);

        let contract_id = Contract::load_from(MARKET_CONTRACT_BINARY_PATH, contract_configuration)
            .unwrap()
            .with_salt(salt)
            .deploy(&owner, TxPolicies::default())
            .await
            .unwrap();

        let market = Market::new(contract_id.clone(), owner.clone());

        Self { instance: market }
    }

    pub async fn deposit(&self, amount: u64, asset: AssetId) -> FuelCallResponse<()> {
        // TODO: custom?
        let tx_params = TxPolicies::new(Some(0), Some(2_000_000), None, None, None);
        let call_params = CallParameters::new(amount, asset, 1_000_000);

        self.instance
            .methods()
            .deposit()
            .with_tx_policies(tx_params)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
            .unwrap()
    }

    pub async fn withdraw(&self, amount: u64, asset: AssetId) -> FuelCallResponse<()> {
        self.instance
            .methods()
            .withdraw(amount, asset)
            .append_variable_outputs(1)
            .call()
            .await
            .unwrap()
    }

    pub async fn open_order(
        &self,
        amount: u64,
        asset: AssetId,
        order_type: OrderType,
        price: u64,
    ) -> FuelCallResponse<Bits256> {
        self.instance
            .methods()
            .open_order(amount, asset, order_type, price)
            .call()
            .await
            .unwrap()
    }

    pub async fn cancel_order(&self, order_id: Bits256) -> FuelCallResponse<()> {
        self.instance
            .methods()
            .cancel_order(order_id)
            .call()
            .await
            .unwrap()
    }

    pub async fn batch_fulfill(
        &self,
        order_id: Bits256,
        orders: Vec<Bits256>,
    ) -> FuelCallResponse<()> {
        self.instance
            .methods()
            .batch_fulfill(order_id, orders)
            .call()
            .await
            .unwrap()
    }

    pub async fn set_fee(&self, amount: u64, user: Option<Identity>) -> FuelCallResponse<()> {
        self.instance
            .methods()
            .set_fee(amount, user)
            .call()
            .await
            .unwrap()
    }

    pub async fn account(&self, user: Identity) -> FuelCallResponse<Option<Account>> {
        self.instance.methods().account(user).call().await.unwrap()
    }

    pub async fn fee(&self, user: Option<Identity>) -> FuelCallResponse<u64> {
        self.instance.methods().fee(user).call().await.unwrap()
    }

    pub async fn order(&self, order: Bits256) -> FuelCallResponse<Option<Order>> {
        self.instance.methods().order(order).call().await.unwrap()
    }

    pub async fn user_orders(&self, user: Identity) -> FuelCallResponse<Vec<Bits256>> {
        self.instance
            .methods()
            .user_orders(user)
            .call()
            .await
            .unwrap()
    }

    pub async fn config(&self) -> FuelCallResponse<(Address, AssetId, u32, AssetId, u32, u32)> {
        self.instance.methods().config().call().await.unwrap()
    }

    pub async fn order_id(
        &self,
        amount: u64,
        asset: AssetId,
        order_type: OrderType,
        owner: Identity,
        price: u64,
    ) -> FuelCallResponse<Bits256> {
        self.instance
            .methods()
            .order_id(amount, asset, order_type, owner, price)
            .call()
            .await
            .unwrap()
    }

    pub async fn with_account(&self, account: &WalletUnlocked) -> Self {
        Self {
            instance: self.instance.with_account(account.clone()).unwrap(),
        }
    }
}
