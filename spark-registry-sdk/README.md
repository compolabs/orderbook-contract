# Spark MarketRegistry Contract Rust SDK

The Spark MarketRegistry Contract SDK is designed for interacting with the Spark MarketRegistry contract.
There are a set of transactional methods such as `deploy`, register/unregister owner methods and `markets` getter method. Given below a detailed explanation of every contract method.

## SparkRegistryContract Type

```rust
pub struct SparkRegistryContract {
    instance: MarketRegistry<WalletUnlocked>,
}
```

## Transactional SparkMarketContract Owner Methods

### Contract Deployment

```rust
pub async fn deploy(owner: WalletUnlocked) -> anyhow::Result<Self>
```

Deploys a new market registry contract with given owner.

`owner` The owner of the market registry contract that manages market list.

Returns a new instance of SparkRegistryContract type.


### Register new Market

```rust
pub async fn register_market(&self, market: ContractId) -> anyhow::Result<CallResponse<()>>
```

Registers a new market by owner.

`self` The SparkMarketContract instance.
`market` The market contract id.

Returns a call result


### Unregister Market

```rust
pub async fn unregister_market(&self, market: ContractId) -> anyhow::Result<CallResponse<()>>
```

Unregisters a market by owner.

`self` The SparkMarketContract instance.
`market` The market contract id.

Returns a call result


## SparkRegistryContract Getter Methods

### Markets Info

```rust
pub async fn markets(
        &self,
        assets: Vec<(AssetId, AssetId)>,
    ) -> anyhow::Result<CallResponse<Vec<(AssetId, AssetId, Option<ContractId>)
```

Retrieves user account inforamtion.

`self` The SparkMarketContract instance.
`assets` The asset pair array [(base_asst_id, quote_asset_id)].

Returns an asset pair and optional market contract id array


### Config

```rust
pub fn config() -> (Option<Identity>, u32);
```

Retrieves contract configurables.

Returns an Option of owner identity and contract version
