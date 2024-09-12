use fuels::{
    prelude::{
        abigen, AssetId, CallParameters, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxPolicies, VariableOutputPolicy, WalletUnlocked,
    },
    programs::responses::CallResponse,
    types::{bech32::Bech32ContractId, Bits256, Bytes32, Identity},
};

use rand::Rng;
use std::path::PathBuf;

abigen!(Contract(
    name = "SparkMarket",
    abi = "spark-market/out/release/spark-market-abi.json"
));

const MARKET_CONTRACT_BINARY_PATH: &str = "spark-market/out/release/spark-market.bin";
const MARKET_CONTRACT_STORAGE_PATH: &str =
    "spark-market/out/release/spark-market-storage_slots.json";

pub struct SparkMarketContract {
    instance: SparkMarket<WalletUnlocked>,
}

impl SparkMarketContract {
    pub async fn deploy(
        base_asset: AssetId,
        base_decimals: u32,
        quote_asset: AssetId,
        quote_decimals: u32,
        owner: WalletUnlocked,
        price_decimals: u32,
        version: u32,
    ) -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(root.join(MARKET_CONTRACT_STORAGE_PATH));

        let configurables = SparkMarketConfigurables::default()
            .with_BASE_ASSET(base_asset)
            .unwrap()
            .with_BASE_ASSET_DECIMALS(base_decimals)
            .unwrap()
            .with_QUOTE_ASSET(quote_asset)
            .unwrap()
            .with_QUOTE_ASSET_DECIMALS(quote_decimals)
            .unwrap()
            .with_OWNER(owner.address().into())
            .unwrap()
            .with_PRICE_DECIMALS(price_decimals)
            .unwrap()
            .with_VERSION(version)
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

        let market = SparkMarket::new(contract_id.clone(), owner.clone());

        Ok(Self { instance: market })
    }

    pub async fn new(contract_id: ContractId, wallet: WalletUnlocked) -> Self {
        let _self = Self {
            instance: SparkMarket::new(contract_id, wallet),
        };
        assert!(
            _self.contract_version().await.unwrap() & 0xFF0000 == Self::sdk_version() & 0xFF0000,
            "Market contract version mismatch with SDK version"
        );
        _self
    }

    pub fn get_instance(&self) -> &SparkMarket<WalletUnlocked> {
        &self.instance
    }

    pub async fn with_account(&self, account: &WalletUnlocked) -> anyhow::Result<Self> {
        Ok(Self {
            instance: self.instance.clone().with_account(account.clone()),
        })
    }

    pub fn id(&self) -> Bytes32 {
        self.instance.contract_id().hash
    }

    pub fn contract_id(&self) -> &Bech32ContractId {
        self.instance.contract_id()
    }

    pub async fn contract_version(&self) -> anyhow::Result<u32> {
        let (_, _, _, _, _, _, version) = self.config().await?.value;
        Ok(version)
    }

    pub async fn contract_str_version(&self) -> anyhow::Result<String> {
        let version = self.contract_version().await?;
        Ok(format!(
            "{}.{}.{}",
            (version & 0xFF0000) >> 16,
            (version & 0xFF00) >> 8,
            version & 0xFF
        ))
    }

    pub fn sdk_version() -> u32 {
        let s_version = Self::sdk_str_version();
        // Converts "0.1.1" string version to 257u32 (0x000101)
        let version = s_version.split('.').collect::<Vec<&str>>();
        let len = version.len();
        version
            .iter()
            .enumerate()
            .map(|(i, &x)| x.parse::<u32>().unwrap() << (8 * (len - i - 1)))
            .collect::<Vec<u32>>()
            .iter()
            .sum()
    }

    pub fn sdk_str_version() -> String {
        env!("CARGO_PKG_VERSION").into()
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
        order_type: OrderType,
        price: u64,
    ) -> anyhow::Result<CallResponse<Bits256>> {
        Ok(self
            .instance
            .methods()
            .open_order(amount, order_type, price)
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
        order_type: OrderType,
        limit_type: LimitType,
        price: u64,
        slippage: u64,
        orders: Vec<Bits256>,
    ) -> anyhow::Result<CallResponse<Bits256>> {
        Ok(self
            .instance
            .methods()
            .fulfill_order_many(amount, order_type, limit_type, price, slippage, orders)
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn set_protocol_fee(
        &self,
        fee: Vec<ProtocolFee>,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self.instance.methods().set_protocol_fee(fee).call().await?)
    }

    pub async fn set_matcher_fee(&self, amount: u64) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_matcher_fee(amount)
            .call()
            .await?)
    }

    pub async fn account(&self, user: Identity) -> anyhow::Result<CallResponse<Account>> {
        Ok(self.instance.methods().account(user).simulate().await?)
    }

    pub async fn protocol_fee(&self) -> anyhow::Result<CallResponse<Vec<ProtocolFee>>> {
        Ok(self.instance.methods().protocol_fee().simulate().await?)
    }

    pub async fn protocol_fee_user(
        &self,
        user: Identity,
    ) -> anyhow::Result<CallResponse<(u64, u64)>> {
        Ok(self
            .instance
            .methods()
            .protocol_fee_user(user)
            .simulate()
            .await?)
    }

    pub async fn protocol_fee_user_amount(
        &self,
        amount: u64,
        user: Identity,
    ) -> anyhow::Result<CallResponse<(u64, u64)>> {
        Ok(self
            .instance
            .methods()
            .protocol_fee_user_amount(amount, user)
            .simulate()
            .await?)
    }

    pub async fn matcher_fee(&self) -> anyhow::Result<CallResponse<u64>> {
        Ok(self.instance.methods().matcher_fee().simulate().await?)
    }

    pub async fn get_epoch(&self) -> anyhow::Result<CallResponse<(u64, u64)>> {
        Ok(self.instance.methods().get_epoch().simulate().await?)
    }

    pub async fn set_epoch(
        &self,
        epoch: u64,
        epoch_duration: u64,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_epoch(epoch, epoch_duration)
            .call()
            .await?)
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
    ) -> anyhow::Result<CallResponse<(AssetId, u32, AssetId, u32, Identity, u32, u32)>> {
        Ok(self.instance.methods().config().simulate().await?)
    }

    pub async fn order_id(
        &self,
        order_type: OrderType,
        owner: Identity,
        price: u64,
        block_height: u32,
        order_height: u64,
    ) -> anyhow::Result<CallResponse<Bits256>> {
        Ok(self
            .instance
            .methods()
            .order_id(order_type, owner, price, block_height, order_height)
            .simulate()
            .await?)
    }
}
