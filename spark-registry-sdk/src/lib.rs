use fuels::{
    prelude::{
        abigen, AssetId, Bech32ContractId, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxPolicies, WalletUnlocked,
    },
    programs::{calls::Execution, responses::CallResponse},
    types::{Bytes32, Identity},
};
use rand::Rng;
use std::path::PathBuf;

abigen!(
    Contract(
        name = "SparkRegistry",
        abi = "spark-registry/out/release/spark-registry-abi.json"
    ),
    Contract(
        name = "SparkProxy",
        abi = "spark-proxy/out/release/spark-proxy-abi.json"
    )
);

const SPARK_REGISTRY_CONTRACT_BINARY_PATH: &str = "spark-registry/out/release/spark-registry.bin";
const SPARK_REGISTRY_CONTRACT_STORAGE_PATH: &str =
    "spark-registry/out/release/spark-registry-storage_slots.json";

pub struct SparkRegistryContract {
    instance: SparkRegistry<WalletUnlocked>,
}

impl SparkRegistryContract {
    pub async fn deploy(owner: WalletUnlocked, version: u32) -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let storage_configuration = StorageConfiguration::default()
            .add_slot_overrides_from_file(root.join(SPARK_REGISTRY_CONTRACT_STORAGE_PATH));

        let configurables = SparkRegistryConfigurables::default()
            .with_VERSION(version)
            .unwrap();

        let contract_configuration = LoadConfiguration::default()
            .with_storage_configuration(storage_configuration?)
            .with_configurables(configurables);

        let contract_id = Contract::load_from(
            root.join(SPARK_REGISTRY_CONTRACT_BINARY_PATH),
            contract_configuration,
        )?
        .with_salt(salt)
        .deploy(&owner, TxPolicies::default())
        .await?;

        let market_registry = SparkRegistry::new(contract_id.clone(), owner.clone());

        market_registry
            .methods()
            .initialize_ownership(owner.address().into())
            .call()
            .await?;

        Ok(Self {
            instance: market_registry,
        })
    }

    pub async fn new(contract_id: ContractId, wallet: WalletUnlocked) -> Self {
        let _self = Self {
            instance: SparkRegistry::new(contract_id, wallet),
        };
        assert!(
            _self.contract_version().await.unwrap() & 0xFF0000 == Self::sdk_version() & 0xFF0000,
            "SparkRegistry contract version mismatch with SDK version"
        );
        _self
    }

    pub fn with_account(&self, account: &WalletUnlocked) -> Self {
        Self {
            instance: self.instance.clone().with_account(account.clone()),
        }
    }

    pub fn id(&self) -> Bytes32 {
        self.instance.contract_id().hash
    }

    pub fn contract_id(&self) -> &Bech32ContractId {
        self.instance.contract_id()
    }

    pub async fn contract_version(&self) -> anyhow::Result<u32> {
        let (_, version) = self.config().await?.value;
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

    async fn market_implementation(&self, market: ContractId) -> ContractId {
        let proxy = SparkProxy::new(market, self.instance.account().clone());
        let result = proxy
            .methods()
            .proxy_target()
            .simulate(Execution::StateReadOnly)
            .await;
        match result {
            Ok(response) => response.value.unwrap(),
            Err(_) => market,
        }
    }
    pub async fn register_market(&self, market: ContractId) -> anyhow::Result<CallResponse<()>> {
        let implementation = self.market_implementation(market).await;
        let contract_ids = if implementation == market {
            vec![market.into()]
        } else {
            vec![market.into(), implementation.into()]
        };
        Ok(self
            .instance
            .methods()
            .register_market(market)
            .with_contract_ids(&contract_ids)
            .call()
            .await?)
    }

    pub async fn unregister_market(&self, market: ContractId) -> anyhow::Result<CallResponse<()>> {
        let implementation = self.market_implementation(market).await;
        let contract_ids = if implementation == market {
            vec![market.into()]
        } else {
            vec![market.into(), implementation.into()]
        };
        Ok(self
            .instance
            .methods()
            .unregister_market(market)
            .with_contract_ids(&contract_ids)
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
            .call()
            .await?)
    }

    pub async fn owner(&self) -> anyhow::Result<CallResponse<State>> {
        Ok(self
            .instance
            .methods()
            .owner()
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn config(&self) -> anyhow::Result<CallResponse<(Option<Identity>, u32)>> {
        Ok(self
            .instance
            .methods()
            .config()
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn markets(
        &self,
        assets: Vec<(AssetId, AssetId)>,
    ) -> anyhow::Result<CallResponse<Vec<(AssetId, AssetId, Option<ContractId>)>>> {
        Ok(self
            .instance
            .methods()
            .markets(assets)
            .simulate(Execution::StateReadOnly)
            .await?)
    }
}
