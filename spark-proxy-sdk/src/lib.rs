use fuels::{
    prelude::{
        abigen, Address, Bech32ContractId, Contract, ContractId, LoadConfiguration,
        StorageConfiguration, TxPolicies, WalletUnlocked,
    },
    programs::{calls::Execution, responses::CallResponse},
    tx::StorageSlot,
    types::Bytes32,
};
use rand::Rng;
use std::path::PathBuf;
use std::str::FromStr;

abigen!(Contract(
    name = "SparkProxy",
    abi = "spark-proxy/out/release/spark-proxy-abi.json"
));

const SPARK_PROXY_CONTRACT_BINARY_PATH: &str = "spark-proxy/out/release/spark-proxy.bin";
//const SPARK_PROXY_CONTRACT_STORAGE_PATH: &str =
//    "spark-proxy/out/release/spark-proxy-storage_slots.json";

pub struct SparkProxyContract {
    instance: SparkProxy<WalletUnlocked>,
}

impl SparkProxyContract {
    pub async fn deploy(target: ContractId, owner: WalletUnlocked) -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<[u8; 32]>();

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let target_key0 =
            Bytes32::from_str("0x7bb458adc1d118713319a5baa00a2d049dd64d2916477d2688d76970c898cd55")
                .unwrap();
        let target_key1 =
            Bytes32::from_str("0x7bb458adc1d118713319a5baa00a2d049dd64d2916477d2688d76970c898cd56")
                .unwrap();

        let target_value = Bytes32::new(target.into());
        let mut target_value0 = Bytes32::new([0u8; 32]);
        let mut target_value1 = Bytes32::new([0u8; 32]);
        target_value0[7] = 1;
        for n in 8..32 {
            target_value0[n] = target_value[n - 8];
        }
        for n in 0..8 {
            target_value1[n] = target_value[n + 24];
        }

        let owner_key0 =
            Bytes32::from_str("bb79927b15d9259ea316f2ecb2297d6cc8851888a98278c0a2e03e1a091ea754")
                .unwrap();
        let owner_key1 =
            Bytes32::from_str("bb79927b15d9259ea316f2ecb2297d6cc8851888a98278c0a2e03e1a091ea755")
                .unwrap();

        let owner_value = Bytes32::new(Address::from(owner.address()).into());
        let mut owner_value0 = Bytes32::new([0u8; 32]);
        let mut owner_value1 = Bytes32::new([0u8; 32]);
        owner_value0[7] = 1;
        for n in 16..32 {
            owner_value0[n] = owner_value[n - 16];
        }
        for n in 0..16 {
            owner_value1[n] = owner_value[n + 16];
        }

        let storage_slots = [
            StorageSlot::new(target_key0, target_value0),
            StorageSlot::new(target_key1, target_value1),
            StorageSlot::new(owner_key0, owner_value0),
            StorageSlot::new(owner_key1, owner_value1),
        ];
        let storage_configuration =
            StorageConfiguration::default().add_slot_overrides(storage_slots);
        //let storage_configuration = StorageConfiguration::default()
        //    .add_slot_overrides_from_file(root.join(SPARK_PROXY_CONTRACT_STORAGE_PATH))?;

        let contract_configuration =
            LoadConfiguration::default().with_storage_configuration(storage_configuration);

        let contract_id = Contract::load_from(
            root.join(SPARK_PROXY_CONTRACT_BINARY_PATH),
            contract_configuration,
        )?
        .with_salt(salt)
        .deploy(&owner, TxPolicies::default())
        .await?;

        let proxy = SparkProxy::new(contract_id.clone(), owner.clone());

        Ok(Self { instance: proxy })
    }

    pub async fn new(contract_id: ContractId, wallet: WalletUnlocked) -> Self {
        Self {
            instance: SparkProxy::new(contract_id, wallet),
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

    pub fn contract_id(&self) -> &Bech32ContractId {
        self.instance.contract_id()
    }

    pub async fn set_proxy_target(
        &self,
        new_target: ContractId,
    ) -> anyhow::Result<CallResponse<()>> {
        Ok(self
            .instance
            .methods()
            .set_proxy_target(new_target)
            .call()
            .await?)
    }

    pub async fn proxy_target(&self) -> anyhow::Result<CallResponse<Option<ContractId>>> {
        Ok(self
            .instance
            .methods()
            .proxy_target()
            .simulate(Execution::StateReadOnly)
            .await?)
    }

    pub async fn proxy_owner(&self) -> anyhow::Result<CallResponse<State>> {
        Ok(self
            .instance
            .methods()
            .proxy_owner()
            .simulate(Execution::StateReadOnly)
            .await?)
    }
}
