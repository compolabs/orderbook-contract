use fuels::{
    prelude::{
        abigen, Address, AssetId, CallParameters, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxPolicies, WalletUnlocked,
    },
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::{Bits256, Bytes32, Identity},
};
use rand::Rng;

abigen!(Contract(
    name = "Market",
    abi = "./market-contract/out/release/market-contract-abi.json"
));

const MARKET_CONTRACT_BINARY_PATH: &str = "../market-contract/out/release/market-contract.bin";
const MARKET_CONTRACT_STORAGE_PATH: &str =
    "../market-contract/out/release/market-contract-storage_slots.json";

pub struct MarketContract {
    instance: Market<WalletUnlocked>,
}

impl MarketContract {
    pub async fn deploy(
        base_asset: AssetId,
        base_decimals: u32,
        quote_asset: AssetId,
        quote_decimals: u32,
        price_decimals: u32,
        owner: WalletUnlocked,
    ) -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(MARKET_CONTRACT_STORAGE_PATH);

        let configurables = MarketConfigurables::default()
            .with_BASE_ASSET(base_asset)
            .unwrap()
            .with_BASE_ASSET_DECIMALS(base_decimals)
            .unwrap()
            .with_QUOTE_ASSET(quote_asset)
            .unwrap()
            .with_QUOTE_ASSET_DECIMALS(quote_decimals)
            .unwrap()
            .with_PRICE_DECIMALS(price_decimals)
            .unwrap()
            .with_OWNER(owner.address().into())
            .unwrap();

        let contract_configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration?)
            .with_configurables(configurables);

        let contract_id = Contract::load_from(MARKET_CONTRACT_BINARY_PATH, contract_configuration)?
            .with_salt(salt)
            .deploy(&owner, TxPolicies::default())
            .await?;

        let market = Market::new(contract_id.clone(), owner.clone());

        Ok(Self { instance: market })
    }

    pub async fn new(contract_id: ContractId, wallet: WalletUnlocked) -> Self {
        Self {
            instance: Market::new(contract_id, wallet),
        }
    }

    pub async fn with_account(&self, account: &WalletUnlocked) -> anyhow::Result<Self> {
        Ok(Self {
            instance: self.instance.clone().with_account(account.clone()),
        })
    }

    pub fn id(&self) -> Bytes32 {
        self.instance.contract_id().hash
    }

    pub async fn deposit(
        &self,
        amount: u64,
        asset: AssetId,
    ) -> anyhow::Result<FuelCallResponse<()>> {
        // TODO: configurable?
        let call_params = CallParameters::new(amount, asset, 1_000_000);
        let tx_policies = TxPolicies::default().with_tip(1);

        Ok(self
            .instance
            .methods()
            .deposit()
            .with_tx_policies(tx_policies)
            .call_params(call_params)?
            .call()
            .await?)
    }

    pub async fn withdraw(
        &self,
        amount: u64,
        asset: AssetId,
    ) -> anyhow::Result<FuelCallResponse<()>> {
        // TODO: Spark CLI seems broken here for some reason
        Ok(self
            .instance
            .methods()
            .withdraw(amount, asset)
            .append_variable_outputs(1)
            .call()
            .await?)
    }

    pub async fn open_order(
        &self,
        amount: u64,
        asset: AssetId,
        order_type: OrderType,
        price: u64,
    ) -> anyhow::Result<FuelCallResponse<Bits256>> {
        Ok(self
            .instance
            .methods()
            .open_order(amount, asset, order_type, price)
            .call()
            .await?)
    }

    pub async fn cancel_order(&self, order_id: Bits256) -> anyhow::Result<FuelCallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .cancel_order(order_id)
            .call()
            .await?)
    }

    pub async fn set_fee(
        &self,
        amount: u64,
        user: Option<Identity>,
    ) -> anyhow::Result<FuelCallResponse<()>> {
        Ok(self.instance.methods().set_fee(amount, user).call().await?)
    }

    pub async fn account(
        &self,
        user: Identity,
    ) -> anyhow::Result<FuelCallResponse<Option<Account>>> {
        Ok(self.instance.methods().account(user).simulate().await?)
    }

    pub async fn fee(&self, user: Option<Identity>) -> anyhow::Result<FuelCallResponse<u64>> {
        Ok(self.instance.methods().fee(user).simulate().await?)
    }

    pub async fn order(&self, order: Bits256) -> anyhow::Result<FuelCallResponse<Option<Order>>> {
        Ok(self.instance.methods().order(order).simulate().await?)
    }

    pub async fn user_orders(
        &self,
        user: Identity,
    ) -> anyhow::Result<FuelCallResponse<Vec<Bits256>>> {
        Ok(self.instance.methods().user_orders(user).simulate().await?)
    }

    pub async fn config(
        &self,
    ) -> anyhow::Result<FuelCallResponse<(Address, AssetId, u32, AssetId, u32, u32)>> {
        Ok(self.instance.methods().config().simulate().await?)
    }

    pub async fn order_id(
        &self,
        amount: u64,
        asset_type: AssetType,
        order_type: OrderType,
        owner: Identity,
        price: u64,
    ) -> anyhow::Result<FuelCallResponse<Bits256>> {
        Ok(self
            .instance
            .methods()
            .order_id(amount, asset_type, order_type, owner, price)
            .simulate()
            .await?)
    }
}
