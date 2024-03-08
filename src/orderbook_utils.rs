use fuels::{
    prelude::abigen,
    programs::{call_utils::TxDependencyExtension, contract::CallParameters},
    types::{bech32::Bech32Address, Bits256},
};

pub mod orderbook_interactions {

    use fuels::accounts::wallet::WalletUnlocked;
    use fuels::prelude::Account;
    use fuels::prelude::Bech32Address;
    use fuels::prelude::TxPolicies;
    use fuels::tx::Bytes32;
    use fuels::tx::Receipt;
    use fuels::types::AssetId;

    pub async fn _create_market(
        wallet: &WalletUnlocked,
        predicate_root: &Bech32Address,
        asset_id: AssetId,
        amount: u64,
    ) -> Result<(Bytes32, Vec<Receipt>), fuels::prelude::Error> {
        let policies = TxPolicies::default().with_gas_price(1);
        wallet
            .transfer(predicate_root, amount, asset_id, policies)
            .await
    }
}

use std::str::FromStr;

use fuels::{
    accounts::wallet::WalletUnlocked,
    programs::{
        call_response::FuelCallResponse,
        contract::{Contract, LoadConfiguration},
    },
    types::{transaction::TxPolicies, AssetId, ContractId},
};
use rand::Rng;

use self::abigen_bindings::orderbook_contract_mod;

abigen!(Contract(
    name = "OrderbookContract",
    abi = "contract/out/debug/orderbook-abi.json"
));

pub struct Orderbook {
    pub instance: OrderbookContract<WalletUnlocked>,
    pub quote_token: AssetId,
    pub quote_token_decimals: u64,
    pub price_decimals: u64,
}

impl Orderbook {
    pub async fn _create_market(
        &self,
        asset_id: AssetId,
        decimal: u32,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        self.instance
            .methods()
            .create_market(asset_id, decimal)
            .call()
            .await
    }

    pub async fn get_market_by_id(
        &self,
        asset_id: AssetId,
    ) -> Result<FuelCallResponse<orderbook_contract_mod::Market>, fuels::types::errors::Error> {
        self.instance
            .methods()
            .get_market_by_id(asset_id)
            .simulate()
            .await
    }

    pub async fn market_exists(
        &self,
        asset_id: AssetId,
    ) -> Result<FuelCallResponse<bool>, fuels::types::errors::Error> {
        self.instance
            .methods()
            .market_exists(asset_id)
            .simulate()
            .await
    }

    pub async fn order_by_id(
        &self,
        id: &Bits256,
    ) -> Result<FuelCallResponse<Option<Order>>, fuels::types::errors::Error> {
        self.instance.methods().order_by_id(*id).simulate().await
    }

    pub async fn orders_by_trader(
        &self,
        trader: &Bech32Address,
    ) -> Result<FuelCallResponse<Vec<Bits256>>, fuels::types::errors::Error> {
        self.instance
            .methods()
            .orders_by_trader(trader)
            .simulate()
            .await
    }

    pub async fn open_order(
        &self,
        market: AssetId,
        base_size: i64,
        base_price: u64,
    ) -> Result<FuelCallResponse<Bits256>, fuels::types::errors::Error> {
        let call_params: CallParameters = if base_size.is_negative() {
            CallParameters::default()
                .with_asset_id(market)
                .with_amount(base_size.abs() as u64)
        } else {
            let market = self.get_market_by_id(market).await.unwrap().value;
            let quote_size = base_size.abs() as u128 * base_price as u128
                / 10u128.pow(
                    self.price_decimals as u32 + market.asset_decimals
                        - self.quote_token_decimals as u32,
                );
            CallParameters::default()
                .with_asset_id(self.quote_token)
                .with_amount(quote_size as u64)
        };

        self.instance
            .methods()
            .open_order(market, I64::from(base_size), base_price)
            .append_variable_outputs(2)
            .call_params(call_params)
            .unwrap()
            .call()
            .await
    }

    pub async fn cancel_order(
        &self,
        order_id: &Bits256,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        self.instance
            .methods()
            .cancel_order(*order_id)
            .append_variable_outputs(1)
            .call()
            .await
    }

    pub async fn match_orders(
        &self,
        sell_order_id: &Bits256,
        buy_order_id: &Bits256,
    ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
        self.instance
            .methods()
            .match_orders(*sell_order_id, *buy_order_id)
            .append_variable_outputs(2)
            .call()
            .await
    }

    pub fn with_account(&self, account: &WalletUnlocked) -> Self {
        Self {
            instance: self.instance.with_account(account.clone()).unwrap(),
            quote_token: self.quote_token,
            quote_token_decimals: self.quote_token_decimals,
            price_decimals: self.price_decimals,
        }
    }

    pub async fn new(wallet: &WalletUnlocked, contract_id: &str) -> Self {
        let instance = OrderbookContract::new(
            &ContractId::from_str(contract_id).unwrap().into(),
            wallet.clone(),
        );
        let (quote_token, quote_token_decimals, price_decimals) = instance
            .methods()
            .get_configurables()
            .simulate()
            .await
            .unwrap()
            .value;
        Orderbook {
            instance,
            quote_token,
            quote_token_decimals: quote_token_decimals as u64,
            price_decimals: price_decimals as u64,
        }
    }

    pub async fn deploy(
        wallet: &WalletUnlocked,
        quote_token: AssetId,
        quote_token_decimals: u64,
        price_decimals: u64,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let configurables = OrderbookContractConfigurables::default()
            .with_QUOTE_TOKEN(quote_token)
            .with_QUOTE_TOKEN_DECIMALS(quote_token_decimals.try_into().unwrap())
            .with_PRICE_DECIMALS(price_decimals.try_into().unwrap());
        let config = LoadConfiguration::default().with_configurables(configurables);

        let id = Contract::load_from("contract/out/debug/orderbook.bin", config)
            .unwrap()
            .with_salt(salt)
            .deploy(wallet, TxPolicies::default().with_gas_price(1))
            .await
            .unwrap();

        let instance = OrderbookContract::new(id, wallet.clone());

        Orderbook {
            instance,
            quote_token,
            quote_token_decimals,
            price_decimals,
        }
    }
}

//todo вынести в отдельный файл
impl I64 {
    pub fn neg_from(value: u64) -> Self {
        I64::new(value, true)
    }

    pub fn as_i64(&self) -> i64 {
        if self.negative {
            -(self.value as i64)
        } else {
            self.value as i64
        }
    }
}

impl From<u64> for I64 {
    fn from(value: u64) -> Self {
        I64::new(value, false)
    }
}
impl From<i64> for I64 {
    fn from(value: i64) -> Self {
        I64::new(value.abs() as u64, value.is_negative())
    }
}
impl From<f64> for I64 {
    fn from(value: f64) -> Self {
        I64::new(value.abs() as u64, value.is_sign_negative())
    }
}
