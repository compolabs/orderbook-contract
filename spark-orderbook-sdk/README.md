# Spark Orderbook Contract Rust SDK

The Spark Orderbook Contract SDK designed for Spark Market contract communication.
There are a set of transactional methods such as `deploy`, register/unregister owner methods and `markets` getter method. Given below a detailed explanation of every contract method.

## OrderbookContract Type

```

pub struct OrderbookContract {
    instance: Orderbook<WalletUnlocked>,
}

```

## Transactional MarketContract Owner Methods

### Contract Deployment

```

pub async fn deploy(owner: WalletUnlocked) -> anyhow::Result<Self>

```

Deploys a new orderbook contract with given owner.

`owner` The owner of the orderbook contract that manages market list.

Returns a new instance of OrderbookContract type.


### Register Market

```

pub async fn register_market(&self, market: ContractId) -> anyhow::Result<CallResponse<()>>

```

Registers a new market by owner.

`self` The MarketContract instance.
`market` The market contract id.

Returns a call result


### Unregister Market

```

pub async fn unregister_market(&self, market: ContractId) -> anyhow::Result<CallResponse<()>>

```

Unregisters a market by owner.

`self` The MarketContract instance.
`market` The market contract id.

Returns a call result


## OrdebookContract Getter Methods

### Markets Info

```

pub async fn markets(
        &self,
        assets: Vec<(AssetId, AssetId)>,
    ) -> anyhow::Result<CallResponse<Vec<(AssetId, AssetId, Option<ContractId>)
```

Retrieves user account inforamtion.

`self` The MarketContract instance.
`assets` The asset pair array [(base_asst_id, quote_asset_id)].

Returns an asset pair and optional market contract id array
