use fuels::{
    prelude::{
        abigen, AssetId, CallParameters, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxPolicies, VariableOutputPolicy, WalletUnlocked,
    },
    programs::{
        calls::{CallHandler, ContractCall, Execution},
        responses::CallResponse,
    },
    types::{bech32::Bech32ContractId, Bits256, Bytes32, Identity},
};

use rand::Rng;
use std::path::PathBuf;

abigen!(
    Contract(
        name = "SparkMarket",
        abi = "spark-market/out/release/spark-market-abi.json"
    ),
    Contract(
        name = "SparkProxy",
        abi = "spark-proxy/out/release/spark-proxy-abi.json"
    )
);

const MARKET_CONTRACT_BINARY_PATH: &str = "spark-market/out/release/spark-market.bin";
const MARKET_CONTRACT_STORAGE_PATH: &str =
    "spark-market/out/release/spark-market-storage_slots.json";

pub struct SparkMarketContract {
    instance: SparkMarket<WalletUnlocked>,
    implementation: ContractId,
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

        market
            .methods()
            .initialize_ownership(owner.address().into())
            .call()
            .await?;

        Ok(Self {
            instance: market,
            implementation: contract_id.into(),
        })
    }

    pub async fn new(contract_id: ContractId, wallet: WalletUnlocked) -> Self {
        let proxy = SparkProxy::new(contract_id, wallet.clone());
        let result = proxy
            .methods()
            .proxy_target()
            .simulate(Execution::StateReadOnly)
            .await;
        let implementation = match result {
            Ok(response) => response.value.unwrap(),
            Err(_) => contract_id,
        };
        let _self = Self {
            instance: SparkMarket::new(contract_id, wallet),
            implementation: implementation,
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

    pub fn with_account(&self, account: &WalletUnlocked) -> Self {
        Self {
            instance: self.instance.clone().with_account(account.clone()),
            implementation: self.implementation,
        }
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
        Ok(self
            .deposit_call_handler(amount, asset)
            .await
            .call()
            .await?)
    }

    pub async fn deposit_call_handler(
        &self,
        amount: u64,
        asset: AssetId,
    ) -> CallHandler<WalletUnlocked, ContractCall, ()> {
        let call_params = CallParameters::new(amount, asset, 1_000_000);

        self.instance
            .methods()
            .deposit()
            .with_contract_ids(&[self.implementation.into()])
            .call_params(call_params)
            .unwrap()
    }

    pub async fn deposit_for(
        &self,
        amount: u64,
        asset: AssetId,
        user: Identity,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .deposit_for_call_handler(amount, asset, user)
            .await
            .call()
            .await?)
    }

    pub async fn deposit_for_call_handler(
        &self,
        amount: u64,
        asset: AssetId,
        user: Identity,
    ) -> CallHandler<WalletUnlocked, ContractCall, ()> {
        let call_params = CallParameters::new(amount, asset, 1_000_000);

        self.instance
            .methods()
            .deposit_for(user)
            .with_contract_ids(&[self.implementation.into()])
            .call_params(call_params)
            .unwrap()
    }

    pub async fn withdraw(
        &self,
        amount: u64,
        asset_type: AssetType,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .withdraw_call_handler(amount, asset_type)
            .await
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn withdraw_call_handler(
        &self,
        amount: u64,
        asset_type: AssetType,
    ) -> CallHandler<WalletUnlocked, ContractCall, ()> {
        self.instance
            .methods()
            .withdraw(amount, asset_type)
            .with_contract_ids(&[self.implementation.into()])
    }

    pub async fn withdraw_to_market(
        &self,
        amount: u64,
        asset_type: AssetType,
        market: &Bech32ContractId,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .withdraw_to_market_call_handler(amount, asset_type, market)
            .await
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn withdraw_to_market_call_handler(
        &self,
        amount: u64,
        asset_type: AssetType,
        market: &Bech32ContractId,
    ) -> CallHandler<WalletUnlocked, ContractCall, ()> {
        self.instance
            .methods()
            .withdraw_to_market(amount, asset_type, market)
            .with_contract_ids(&[self.implementation.into(), market.clone()])
    }

    pub async fn open_order(
        &self,
        amount: u64,
        order_type: OrderType,
        price: u64,
    ) -> anyhow::Result<CallResponse<Bits256>> {
        Ok(self
            .open_order_call_handler(amount, order_type, price)
            .await
            .call()
            .await?)
    }

    pub async fn open_order_call_handler(
        &self,
        amount: u64,
        order_type: OrderType,
        price: u64,
    ) -> CallHandler<WalletUnlocked, ContractCall, Bits256> {
        self.instance
            .methods()
            .open_order(amount, order_type, price)
            .with_contract_ids(&[self.implementation.into()])
    }

    pub async fn cancel_order(&self, order_id: Bits256) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .cancel_order_call_handler(order_id)
            .await
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn cancel_order_call_handler(
        &self,
        order_id: Bits256,
    ) -> CallHandler<WalletUnlocked, ContractCall, ()> {
        self.instance
            .methods()
            .cancel_order(order_id)
            .with_contract_ids(&[self.implementation.into()])
    }

    pub async fn match_order_pair(
        &self,
        order_id0: Bits256,
        order_id1: Bits256,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .match_order_pair_call_handler(order_id0, order_id1)
            .await
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn match_order_pair_call_handler(
        &self,
        order_id0: Bits256,
        order_id1: Bits256,
    ) -> CallHandler<WalletUnlocked, ContractCall, ()> {
        self.instance
            .methods()
            .match_order_pair(order_id0, order_id1)
            .with_contract_ids(&[self.implementation.into()])
    }

    pub async fn match_order_many(&self, orders: Vec<Bits256>) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .match_order_many_call_handler(orders)
            .await
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn match_order_many_call_handler(
        &self,
        orders: Vec<Bits256>,
    ) -> CallHandler<WalletUnlocked, ContractCall, ()> {
        self.instance
            .methods()
            .match_order_many(orders)
            .with_contract_ids(&[self.implementation.into()])
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
            .fulfill_many_call_handler(amount, order_type, limit_type, price, slippage, orders)
            .await
            .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
            .call()
            .await?)
    }

    pub async fn fulfill_many_call_handler(
        &self,
        amount: u64,
        order_type: OrderType,
        limit_type: LimitType,
        price: u64,
        slippage: u64,
        orders: Vec<Bits256>,
    ) -> CallHandler<WalletUnlocked, ContractCall, Bits256> {
        self.instance
            .methods()
            .fulfill_order_many(amount, order_type, limit_type, price, slippage, orders)
            .with_contract_ids(&[self.implementation.into()])
    }

    pub async fn set_protocol_fee(
        &self,
        fee: Vec<ProtocolFee>,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_protocol_fee(fee)
            .with_contract_ids(&[self.implementation.into()])
            .call()
            .await?)
    }

    pub async fn set_matcher_fee(&self, amount: u64) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_matcher_fee(amount)
            .with_contract_ids(&[self.implementation.into()])
            .call()
            .await?)
    }

    pub async fn set_store_order_change_info(
        &self,
        store: bool,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_store_order_change_info(store)
            .with_contract_ids(&[self.implementation.into()])
            .call()
            .await?)
    }

    pub async fn set_min_order_size(&self, size: u64) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_min_order_size(size)
            .with_contract_ids(&[self.implementation.into()])
            .call()
            .await?)
    }

    pub async fn set_min_order_price(&self, price: u64) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_min_order_price(price)
            .with_contract_ids(&[self.implementation.into()])
            .call()
            .await?)
    }

    pub async fn initialize_ownership(
        &self,
        new_owner: Identity,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .initialize_ownership(new_owner)
            .with_contract_ids(&[self.implementation.into()])
            .call()
            .await?)
    }

    pub async fn transfer_ownership(
        &self,
        new_owner: Identity,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .transfer_ownership(new_owner)
            .with_contract_ids(&[self.implementation.into()])
            .call()
            .await?)
    }

    pub async fn owner(&self) -> anyhow::Result<CallResponse<State>> {
        Ok(self
            .instance
            .methods()
            .owner()
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn account(&self, user: Identity) -> anyhow::Result<CallResponse<Account>> {
        Ok(self
            .instance
            .methods()
            .account(user)
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn protocol_fee(&self) -> anyhow::Result<CallResponse<Vec<ProtocolFee>>> {
        Ok(self
            .instance
            .methods()
            .protocol_fee()
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn protocol_fee_user(
        &self,
        user: Identity,
    ) -> anyhow::Result<CallResponse<(u64, u64)>> {
        Ok(self
            .instance
            .methods()
            .protocol_fee_user(user)
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
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
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn matcher_fee(&self) -> anyhow::Result<CallResponse<u64>> {
        Ok(self
            .instance
            .methods()
            .matcher_fee()
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn store_order_change_info(&self) -> anyhow::Result<CallResponse<bool>> {
        Ok(self
            .instance
            .methods()
            .store_order_change_info()
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn get_epoch(&self) -> anyhow::Result<CallResponse<(u64, u64)>> {
        Ok(self
            .instance
            .methods()
            .get_epoch()
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
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
            .with_contract_ids(&[self.implementation.into()])
            .call()
            .await?)
    }

    pub async fn order(&self, order: Bits256) -> anyhow::Result<CallResponse<Option<Order>>> {
        Ok(self
            .instance
            .methods()
            .order(order)
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn user_orders(&self, user: Identity) -> anyhow::Result<CallResponse<Vec<Bits256>>> {
        Ok(self
            .instance
            .methods()
            .user_orders(user)
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn user_order_height(&self, user: Identity) -> anyhow::Result<CallResponse<u64>> {
        Ok(self
            .instance
            .methods()
            .user_order_height(user)
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn order_change_info(
        &self,
        order_id: Bits256,
    ) -> anyhow::Result<CallResponse<Vec<OrderChangeInfo>>> {
        Ok(self
            .instance
            .methods()
            .order_change_info(order_id)
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn min_order_size(&self) -> anyhow::Result<CallResponse<u64>> {
        Ok(self
            .instance
            .methods()
            .min_order_size()
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn min_order_price(&self) -> anyhow::Result<CallResponse<u64>> {
        Ok(self
            .instance
            .methods()
            .min_order_price()
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn config(
        &self,
    ) -> anyhow::Result<CallResponse<(AssetId, u32, AssetId, u32, Option<Identity>, u32, u32)>>
    {
        Ok(self
            .instance
            .methods()
            .config()
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
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
            .with_contract_ids(&[self.implementation.into()])
            .simulate(Execution::StateReadOnly)
            .await?)
    }
}
