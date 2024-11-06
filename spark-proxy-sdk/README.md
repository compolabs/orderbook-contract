# Spark PROXY Contract Rust SDK

The Spark Proxy Contract SDK designed for Spark Market contract communication.

## SparkProxyContract Type

The sdk object as contract instance wrapper.

```rust
pub struct SparkProxyContract {
    instance: SparkProxy<WalletUnlocked>,
}
```


## Transactional SparkProxyContract Common Methods

### Proxy Deploy

```rust
pub async fn deploy(target: ContractId, owner: WalletUnlocked) -> anyhow::Result<Self>
```

Deploys proxy with market target.

`target` The SparkMarketContract instance
`owner` The Wallet object

Returns a SparkProxyContract instance


### Set Proxy Target

```rust
pub async fn set_proxy_target(
        &self,
        new_target: ContractId,
    ) -> anyhow::Result<CallResponse<()>>
```

Sets a new target(implementation) for proxy. Only proxy owner can call.

`self` The SparkProxyContract instance
`new_target` The SparkMarketContract instance

Returns a call result


### Get Proxy Target

```rust
async fn proxy_target(&self) -> anyhow::Result<CallResponse<Option<ContractId>>>
```

Withdraws assets from market caller account.

`self` The SparkProxyContract instance

Returns an optional market contract id


### Set Proxy Owner

```rust
pub async fn set_proxy_owner(
        &self,
        new_proxy_owner: State,
    ) -> anyhow::Result<CallResponse<()>>
```

Sets a new proxy owner. Ony proxy owner can call.

`self` The SparkProxyContract instance
`new_proxy_owner` A new proxy owner

Returns a call result


### Get Proxy Owner

```rust
pub async fn proxy_owner(&self) -> anyhow::Result<CallResponse<State>>
```

Retrieves a proxy owner.

`self` The SparkProxyContract instance

Returns a proxy owner
