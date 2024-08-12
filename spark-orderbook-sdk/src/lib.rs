use fuels::{
    prelude::{
        abigen, AssetId, Contract, ContractId, LoadConfiguration, StorageConfiguration, TxPolicies,
        WalletUnlocked,
    },
    programs::responses::CallResponse,
    types::{Address, Bytes32},
};
use rand::Rng;
use std::path::PathBuf;

abigen!(Contract(
    name = "Orderbook",
    abi = "orderbook-contract/out/release/orderbook-contract-abi.json"
));

const ORDERBOOK_CONTRACT_BINARY_PATH: &str =
    "../orderbook-contract/out/release/orderbook-contract.bin";
const ORDERBOOK_CONTRACT_STORAGE_PATH: &str =
    "../orderbook-contract/out/release/orderbook-contract-storage_slots.json";

pub struct OrderbookContract {
    instance: Orderbook<WalletUnlocked>,
}

impl OrderbookContract {
    pub async fn deploy(owner: WalletUnlocked) -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(root.join(ORDERBOOK_CONTRACT_STORAGE_PATH));

        let configurables = OrderbookConfigurables::default()
            .with_OWNER(owner.address().into())
            .unwrap();

        let contract_configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration?)
            .with_configurables(configurables);

        let contract_id = Contract::load_from(
            root.join(ORDERBOOK_CONTRACT_BINARY_PATH),
            contract_configuration,
        )?
        .with_salt(salt)
        .deploy(&owner, TxPolicies::default())
        .await?;

        let orderbook = Orderbook::new(contract_id.clone(), owner.clone());

        Ok(Self {
            instance: orderbook,
        })
    }

    pub async fn new(contract_id: ContractId, wallet: WalletUnlocked) -> Self {
        Self {
            instance: Orderbook::new(contract_id, wallet),
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

    pub async fn register_market(&self, market: ContractId) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .register_market(market)
            .with_contract_ids(&[market.into()])
            .call()
            .await?)
    }

    pub async fn unregister_market(&self, market: ContractId) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .unregister_market(market)
            .with_contract_ids(&[market.into()])
            .call()
            .await?)
    }

    pub async fn config(&self) -> anyhow::Result<CallResponse<Address>> {
        Ok(self.instance.methods().config().simulate().await?)
    }

    pub async fn markets(
        &self,
        assets: Vec<(AssetId, AssetId)>,
    ) -> anyhow::Result<CallResponse<Vec<(AssetId, AssetId, Option<ContractId>)>>> {
        Ok(self.instance.methods().markets(assets).simulate().await?)
    }
}
