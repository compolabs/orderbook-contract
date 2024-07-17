use fuels::{
    prelude::{
        abigen, Address, AssetId, CallParameters, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxPolicies, VariableOutputPolicy, WalletUnlocked,
    },
    programs::responses::CallResponse,
    types::{Bits256, Bytes32, Identity},
};

use rand::Rng;
use std::path::PathBuf;

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

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(root.join(MARKET_CONTRACT_STORAGE_PATH));

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

        let contract_id = Contract::load_from(
            root.join(MARKET_CONTRACT_BINARY_PATH),
            contract_configuration,
        )?
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

    pub async fn deposit(&self, amount: u64, asset: AssetId) -> anyhow::Result<CallResponse<()>> {
        let call_params = CallParameters::new(amount, asset, 1_000_000);

        Ok(self
            .instance
            .methods()
            .deposit()
            .call_params(call_params)?
            .call()
            .await?)
    }

    pub async fn withdraw(
        &self,
        amount: u64,
        asset_type: AssetType,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .withdraw(amount, asset_type)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn open_order(
        &self,
        amount: u64,
        asset_type: AssetType,
        order_type: OrderType,
        price: u64,
    ) -> anyhow::Result<CallResponse<Bits256>> {
        let matcher_fee = self.matcher_fee().await?.value;
        let call_params = CallParameters::default().with_amount(matcher_fee.into());
        Ok(self
            .instance
            .methods()
            .open_order(amount, asset_type, order_type, price)
            .call_params(call_params)?
            .call()
            .await?)
    }

    pub async fn open_order_with_matcher_fee(
        &self,
        amount: u64,
        asset_type: AssetType,
        order_type: OrderType,
        price: u64,
        matcher_fee: u32,
    ) -> anyhow::Result<CallResponse<Bits256>> {
        let call_params = CallParameters::default().with_amount(matcher_fee.into());
        Ok(self
            .instance
            .methods()
            .open_order(amount, asset_type, order_type, price)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call_params(call_params)?
            .call()
            .await?)
    }

    pub async fn cancel_order(&self, order_id: Bits256) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .cancel_order(order_id)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn match_order_pair(
        &self,
        order_id0: Bits256,
        order_id1: Bits256,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .match_order_pair(order_id0, order_id1)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn match_order_many(&self, orders: Vec<Bits256>) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .match_order_many(orders)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn fulfill_many(
        &self,
        amount: u64,
        asset_type: AssetType,
        order_type: OrderType,
        price: u64,
        slippage: u64,
        orders: Vec<Bits256>,
    ) -> anyhow::Result<CallResponse<Bits256>> {
        Ok(self
            .instance
            .methods()
            .fulfill_order_many(amount, asset_type, order_type, price, slippage, orders)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn set_protocol_fee(&self, amount: u32) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_protocol_fee(amount)
            .call()
            .await?)
    }

    pub async fn set_matcher_fee(&self, amount: u32) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_matcher_fee(amount)
            .call()
            .await?)
    }

    pub async fn account(&self, user: Identity) -> anyhow::Result<CallResponse<Option<Account>>> {
        Ok(self.instance.methods().account(user).simulate().await?)
    }

    pub async fn protocol_fee(&self) -> anyhow::Result<CallResponse<u32>> {
        Ok(self.instance.methods().protocol_fee().simulate().await?)
    }

    pub async fn matcher_fee(&self) -> anyhow::Result<CallResponse<u32>> {
        Ok(self.instance.methods().matcher_fee().simulate().await?)
    }

    pub async fn order(&self, order: Bits256) -> anyhow::Result<CallResponse<Option<Order>>> {
        Ok(self.instance.methods().order(order).simulate().await?)
    }

    pub async fn user_orders(&self, user: Identity) -> anyhow::Result<CallResponse<Vec<Bits256>>> {
        Ok(self.instance.methods().user_orders(user).simulate().await?)
    }

    pub async fn order_change_info(
        &self,
        order_id: Bits256,
    ) -> anyhow::Result<CallResponse<Vec<OrderChangeInfo>>> {
        Ok(self
            .instance
            .methods()
            .order_change_info(order_id)
            .simulate()
            .await?)
    }

    pub async fn config(
        &self,
    ) -> anyhow::Result<CallResponse<(Address, AssetId, u32, AssetId, u32, u32)>> {
        Ok(self.instance.methods().config().simulate().await?)
    }

    pub async fn order_id(
        &self,
        asset_type: AssetType,
        order_type: OrderType,
        owner: Identity,
        price: u64,
        block_height: u32,
    ) -> anyhow::Result<CallResponse<Bits256>> {
        Ok(self
            .instance
            .methods()
            .order_id(asset_type, order_type, owner, price, block_height)
            .simulate()
            .await?)
    }
}
