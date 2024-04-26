use fuels::{
    prelude::{
        abigen, AssetId, CallParameters, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxPolicies, WalletUnlocked,
    },
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    tx::Bytes32,
    types::{bech32::Bech32Address, Bits256},
};
use rand::Rng;

abigen!(Contract(
    name = "Orderbook",
    abi = "./contract/out/debug/orderbook-abi.json"
));

const ORDERBOOK_CONTRACT_BINARY_PATH: &str = "../contract/out/debug/orderbook.bin";
const ORDERBOOK_CONTRACT_STORAGE_PATH: &str = "../contract/out/debug/orderbook-storage_slots.json";

pub struct OrderbookContract {
    instance: Orderbook<WalletUnlocked>,
    pub quote_token: AssetId,
    pub quote_token_decimals: u32,
    pub price_decimals: u32,
}

impl OrderbookContract {
    pub async fn deploy(
        wallet: &WalletUnlocked,
        quote_token: AssetId,
        quote_token_decimals: u32,
        price_decimals: u32,
    ) -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(ORDERBOOK_CONTRACT_STORAGE_PATH);

        let configurables = OrderbookConfigurables::default()
            .with_QUOTE_TOKEN(quote_token)
            .with_QUOTE_TOKEN_DECIMALS(quote_token_decimals)
            .with_PRICE_DECIMALS(price_decimals);

        let contract_configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration?)
            .with_configurables(configurables);

        let id = Contract::load_from(ORDERBOOK_CONTRACT_BINARY_PATH, contract_configuration)?
            .with_salt(salt)
            .deploy(wallet, TxPolicies::default())
            .await?;

        let instance = Orderbook::new(id, wallet.clone());

        Ok(Self {
            instance,
            quote_token,
            quote_token_decimals,
            price_decimals,
        })
    }

    pub async fn new(contract_id: ContractId, wallet: WalletUnlocked) -> anyhow::Result<Self> {
        let instance = Orderbook::new(contract_id, wallet);
        let (quote_token, quote_token_decimals, price_decimals) = instance
            .methods()
            .get_configurables()
            .simulate()
            .await?
            .value;

        Ok(Self {
            instance,
            quote_token,
            quote_token_decimals,
            price_decimals,
        })
    }

    pub fn id(&self) -> Bytes32 {
        self.instance.contract_id().hash
    }

    pub fn with_account(&self, account: &WalletUnlocked) -> anyhow::Result<Self> {
        Ok(Self {
            instance: self.instance.with_account(account.clone())?,
            quote_token: self.quote_token,
            quote_token_decimals: self.quote_token_decimals,
            price_decimals: self.price_decimals,
        })
    }

    pub async fn create_market(
        &self,
        asset_id: AssetId,
        decimal: u32,
    ) -> anyhow::Result<FuelCallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .create_market(asset_id, decimal)
            .call()
            .await?)
    }

    pub async fn open_order(
        &self,
        base_token: AssetId,
        base_size: i64,
        base_price: u64,
    ) -> anyhow::Result<FuelCallResponse<Bits256>> {
        let call_params: CallParameters = if base_size.is_negative() {
            CallParameters::default()
                .with_asset_id(base_token)
                .with_amount(base_size.unsigned_abs())
        } else {
            let market = self.get_market_by_id(base_token).await?.value;
            let quote_size = base_size.unsigned_abs() as u128 * base_price as u128
                / 10u128
                    .pow(self.price_decimals + market.asset_decimals - self.quote_token_decimals);
            CallParameters::default()
                .with_asset_id(self.quote_token)
                .with_amount(quote_size as u64)
        };

        Ok(self
            .instance
            .methods()
            .open_order(
                base_token,
                I64::new(base_size.unsigned_abs(), base_size < 0),
                base_price,
            )
            .append_variable_outputs(2)
            .call_params(call_params)?
            .with_tx_policies(TxPolicies::default().with_gas_price(1))
            .call()
            .await?)
    }

    pub async fn cancel_order(&self, order_id: &Bits256) -> anyhow::Result<FuelCallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .cancel_order(*order_id)
            .append_variable_outputs(1)
            .call()
            .await?)
    }

    pub async fn match_orders(
        &self,
        sell_order_id: &Bits256,
        buy_order_id: &Bits256,
    ) -> anyhow::Result<FuelCallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .match_orders(*sell_order_id, *buy_order_id)
            .append_variable_outputs(2)
            // .with_tx_policies(TxPolicies::default())
            .call()
            .await?)
    }

    pub async fn get_market_by_id(
        &self,
        asset_id: AssetId,
    ) -> anyhow::Result<FuelCallResponse<Market>> {
        Ok(self
            .instance
            .methods()
            .get_market_by_id(asset_id)
            .simulate()
            .await?)
    }

    pub async fn market_exists(&self, asset_id: AssetId) -> anyhow::Result<FuelCallResponse<bool>> {
        Ok(self
            .instance
            .methods()
            .market_exists(asset_id)
            .simulate()
            .await?)
    }

    pub async fn order_by_id(
        &self,
        id: &Bits256,
    ) -> anyhow::Result<FuelCallResponse<Option<Order>>> {
        Ok(self
            .instance
            .methods()
            .order_by_id(*id)
            // .with_tx_policies(TxPolicies::default())
            .simulate()
            .await?)
    }

    pub async fn orders_by_trader(
        &self,
        trader: &Bech32Address,
    ) -> anyhow::Result<FuelCallResponse<Vec<Bits256>>> {
        Ok(self
            .instance
            .methods()
            .orders_by_trader(trader)
            .simulate()
            .await?)
    }
}

impl I64 {
    pub fn as_i64(self) -> i64 {
        self.value as i64 * if self.negative { -1 } else { 1 }
    }
}
