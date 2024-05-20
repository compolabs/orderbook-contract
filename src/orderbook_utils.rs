use std::{path::PathBuf, str::FromStr};

use fuels::{
    accounts::wallet::WalletUnlocked,
    macros::abigen,
    programs::{
        call_response::FuelCallResponse,
        call_utils::TxDependencyExtension,
        contract::{CallParameters, Contract, LoadConfiguration},
    },
    types::{bech32::Bech32Address, transaction::TxPolicies, AssetId, Bits256, ContractId},
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
    pub async fn get_order_change_events_by_order(
        &self,
        ordr_id: Bits256,
    ) -> Result<FuelCallResponse<Vec<OrderChangeEvent>>, fuels::types::errors::Error> {
        self.instance
            .methods()
            .get_order_change_events_by_order(ordr_id)
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
        self.instance
            .methods()
            .order_by_id(*id)
            .with_tx_policies(TxPolicies::default())
            .simulate()
            .await
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
        base_token: AssetId,
        base_size: i64,
        base_price: u64,
    ) -> Result<FuelCallResponse<Bits256>, fuels::types::errors::Error> {
        let call_params: CallParameters = if base_size.is_negative() {
            CallParameters::default()
                .with_asset_id(base_token)
                .with_amount(base_size.abs() as u64)
        } else {
            let market = self.get_market_by_id(base_token).await.unwrap().value;
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
            .open_order(
                base_token,
                I64::new(base_size.unsigned_abs(), base_size < 0),
                base_price,
            )
            .append_variable_outputs(2)
            .call_params(call_params)
            .unwrap()
            .with_tx_policies(TxPolicies::default().with_tip(1))
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
            .match_orders(sell_order_id.clone(), buy_order_id.clone())
            .append_variable_outputs(2)
            .with_tx_policies(TxPolicies::default())
            .call()
            .await
    }
    pub async fn get_configurables(
        &self,
    ) -> Result<FuelCallResponse<(AssetId, u32, u32)>, fuels::types::errors::Error> {
        self.instance.methods().get_configurables().simulate().await
    }
    // pub async fn match_orders_many(
    //     &self,
    //     sell_order_ids: Vec<Bits256>,
    //     buy_order_ids: Vec<Bits256>,
    // ) -> Result<FuelCallResponse<()>, fuels::types::errors::Error> {
    //     self.instance
    //         .methods()
    //         .match_orders_many(sell_order_ids, buy_order_ids)
    //         .append_variable_outputs(2)
    //         .with_tx_policies(TxPolicies::default())
    //         .call()
    //         .await
    // }

    pub fn with_account(&self, account: &WalletUnlocked) -> Self {
        Self {
            instance: self.instance.clone().with_account(account.clone()),
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
            .unwrap()
            .with_QUOTE_TOKEN_DECIMALS(quote_token_decimals.try_into().unwrap())
            .unwrap()
            .with_PRICE_DECIMALS(price_decimals.try_into().unwrap())
            .unwrap();
        let config = LoadConfiguration::default().with_configurables(configurables);

        let bin_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("contract/out/debug/orderbook.bin");
        let id = Contract::load_from(bin_path, config)
            .unwrap()
            .with_salt(salt)
            .deploy(wallet, TxPolicies::default().with_max_fee(250000))
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

impl I64 {
    pub fn as_i64(self) -> i64 {
        self.value as i64 * if self.negative { -1 } else { 1 }
    }
}
