use fuels::prelude::abigen;

pub mod orderbook_interactions {

//    use fuels::accounts::predicate::Predicate;
    use fuels::accounts::wallet::WalletUnlocked;
    use fuels::prelude::Account;
    use fuels::prelude::Bech32Address;
    use fuels::prelude::TxPolicies;
//    use fuels::programs::call_response::FuelCallResponse;
//    use fuels::programs::script_calls::ScriptCallHandler;
    use fuels::tx::Bytes32;
    use fuels::tx::Receipt;
//    use fuels::types::unresolved_bytes::UnresolvedBytes;
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

abigen!(Contract(
    name = "OrderbookContract",
    abi = "contract/out/debug/orderbook-abi.json"
));

pub struct Orderbook {
    pub instance: OrderbookContract<WalletUnlocked>,
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

    pub fn with_account(&self, account: &WalletUnlocked) -> Self {
        Self {
            instance: self.instance.with_account(account.clone()).unwrap(),
        }
    }

    pub fn new(wallet: &WalletUnlocked, contract_id: &str) -> Self {
        Orderbook {
            instance: OrderbookContract::new(
                &ContractId::from_str(contract_id).unwrap().into(),
                wallet.clone(),
            ),
        }
    }

    pub async fn deploy(wallet: &WalletUnlocked, quote_token: AssetId, quote_token_decimals: u64) -> Self {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let configurables = OrderbookContractConfigurables::default()
        .with_QUOTE_TOKEN(quote_token)
        .with_QUOTE_TOKEN_DECIMALS(quote_token_decimals);
        let config = LoadConfiguration::default().with_configurables(configurables);

        let id = Contract::load_from("contract/out/debug/orderbook.bin", config)
            .unwrap()
            .with_salt(salt)
            .deploy(wallet, TxPolicies::default().with_gas_price(1))
            .await
            .unwrap();

        let instance = OrderbookContract::new(id, wallet.clone());

        Orderbook { instance }
    }
}
