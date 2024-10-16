# Spark Market Contract Rust SDK

The Spark Market Contract SDK designed for Spark Market contract communication.
There are a set of transactional methods such as `deploy`, asset deposit/withdraw for order provision, order submittion/cancellation and a set of getter methods for order information. Given below a detailed explanation of every contract method.

## SparkMarketContract Type

The sdk object as contract instance wrapper.

```rust
pub struct SparkMarketContract {
    instance: Market<WalletUnlocked>,
}
```


## Transactional SparkMarketContract Common Methods

### Asset Deposit

```rust
pub async fn deposit(&self, amount: u64, asset: AssetId) -> anyhow::Result<CallResponse<()>>
```

Deposits assets to market caller account. It is a payble method. Caller should have at least `amount` of `asset` on his account before transfer it to market.

`self` The SparkMarketContract instance
`amount` The amount to deposit
`asset` The asset for deposit either `base_asset` or `quote_asset`

Returns a call result


### Asset Deposit For an user

```rust
pub async fn deposit_for(&self, amount: u64, asset: AssetId, user: Identity) -> anyhow::Result<CallResponse<()>>
```

Deposits assets to market `user` account. It is a payble method. Caller should have at least `amount` of `asset` on his account before transfer it to market.

`self` The SparkMarketContract instance
`amount` The amount to deposit
`asset` The asset for deposit either `base_asset` or `quote_asset`
`user` The user account to deposit for

Returns a call result


### Asset Withdraw

```rust
pub async fn withdraw(&self, amount: u64, asset: AssetId) -> anyhow::Result<CallResponse<()>>
```

Withdraws assets from market caller account.

`self` The SparkMarketContract instance
`amount` The amount to withdraw
`asset` The asset for withdraw either `base_asset` or `quote_asset`

Returns a call result


### Asset Withdraw To Market

```rust
pub async fn withdraw_to_market(&self, amount: u64, asset_type: AssetType, market: &Bech32ContractId) -> anyhow::Result<CallResponse<()>>
```

Withdraws user assets from market to another market user account.

`self` The SparkMarketContract instance
`amount` The amount to withdraw
`asset` The asset for withdraw either `base_asset` or `quote_asset`
`market` The market for withdraw to

Returns a call result


### Open GoodTillCancel Order

```rust
pub async fn open_order(
        &self,
        amount: u64,
        order_type: OrderType,
        price: u64,
    ) -> anyhow::Result<CallResponse<Bits256>>
```

Opens GoodTillCancel order from market caller account.

`self` The SparkMarketContract instance
`amount` The order amount in `base_asset` numbers
`order_type` The order type, either sell or buy
`price` The order price in 10.pow of `quote_decimals` multiplied by 10.pow of `price_decimals`

Returns a new order id


### Open ImmediateOrCancel/FillOrKill Order

```rust
    pub async fn fulfill_many(
        &self,
        amount: u64,
        order_type: OrderType,
        limit_type: LimitType,
        price: u64,
        slippage: u64,
        orders: Vec<Bits256>,
    ) -> anyhow::Result<CallResponse<Bits256>>   
```

Opens ImmediateOrCancel or FillOrKill order from market caller account.

`self` The SparkMarketContract instance
`amount` The order amount in `base_asset` numbers
`order_type` The order type, either sell or buy
`limit_type` The limit type IOC or FOK
`price` The order price in 10.pow of `quote_decimals` multiplied by 10.pow of `price_decimals`
`slippage` Price slippage in `price` terms
`orders` The order ids to fill the new one, should be another direction

Returns a new order id (order could be fully filled and removed)


### Cancel Order

```rust
pub async fn cancel_order(&self, order_id: Bits256) -> anyhow::Result<CallResponse<()>>
```

Cancels order and refunds matcher fee from market caller account.

`self` The SparkMarketContract instance
`order_id` The order id to cancel

Returns a call result


### Match Order Pair

```rust
pub async fn match_order_pair(
        &self,
        order_id0: Bits256,
        order_id1: Bits256,
    ) -> anyhow::Result<CallResponse<()>>
```

Matches GoodTillCancel order pair, should be different direction.

`self` The SparkMarketContract instance
`order_id0` The first order id for matching
`order_id1` The second order id for matching

Returns a call result


### Match Orders

```rust
pub async fn match_order_many(&self, orders: Vec<Bits256>) -> anyhow::Result<CallResponse<()>>
```

Matches GoodTillCancel orders, should be different direction, at least one pair should match for method succeed.

`self` The SparkMarketContract instance
`orders` The order id for matching

Returns a call result



## Transactional SparkMarketContract Owner Methods

### Contract Deployment

```rust
pub async fn deploy(
        base_asset: AssetId,
        base_decimals: u32,
        quote_asset: AssetId,
        quote_decimals: u32,
        price_decimals: u32,
        owner: WalletUnlocked,
        fuel_asset: AssetId,
    ) -> anyhow::Result<Self>
```

Deploys a new market contract with given asset ids and its decimals.

`base_asset` The asset id for order opennings
`base_decimals` The decimals of `base_asset`
`quote_asset` The asset id for order payments
`quote_decimals` The decimals of `quote_asset`
`price_decimals` The decimals for order pricing
`owner` The owner of the market contract that manages protocol fees
`fuel_asset` The asset id used for fee payment

Returns a new instance of SparkMarketContract type.


### Set Protocol Fee

```rust
pub struct ProtocolFee {
    pub maker_fee: u64,
    pub taker_fee: u64,
    pub volume_threshold: u64,
}

pub async fn set_protocol_fee(&self, protocol_fee: Vec<ProtocolFee>) -> anyhow::Result<CallResponse<()>>
```

Owner sets protocol fee as percent of trade volume.

`self` The SparkMarketContract instance
`amount` The protocol fee amount, (10_000 == 100%) 

Returns a call result


### Set Matcher Fee

```rust
pub async fn set_matcher_fee(&self, amount: u64) -> anyhow::Result<CallResponse<()>>
```

Owner sets fixed matcher reward for single order match.

`self` The SparkMarketContract instance
`amount` The matcher fee amount in quote token

Returns a call result


### Set Epoch

```rust
pub async fn set_epoch(&self, epoch: u64, epoch_duration) -> anyhow::Result<CallResponse<()>>
```

Owner resets epoch for cumulative trade volumes.

`self` The SparkMarketContract instance
`epoch` The epoch timestamp
`epoch_duration` The epoch duration in seconds

Returns a call result


### Set Minimum Order Size

```rust
pub async fn set_order_size(&self, amount: u64) -> anyhow::Result<CallResponse<()>>
```

Owner sets minimum order size in BASE_ASSET units.

`self` The SparkMarketContract instance
`size` The minimum order size

Returns a call result




## SparkMarketContract Getter Methods

### Configurable Info

```rust
pub async fn config(
    &self,
) -> anyhow::Result<CallResponse<(AssetId, u32, AssetId, u32, Option<Identity>, u32, u32)>>
```

Retrieves contract configurables

`self` The SparkMarketContract instance

Returns base / quote asset ids and its decimals, owner identity, price decimals and version.


### Account Info

```rust
pub async fn account(&self, user: Identity) -> anyhow::Result<CallResponse<Option<Account>>>
```

Retrieves user account inforamtion.

`self` The SparkMarketContract instance
`user` The user address

Returns an optional Account type result

```rust
pub struct Account {
    // Available funds
    pub liquid: Balance,
    // Open orders
    pub locked: Balance,
}

pub struct Balance {
    base: u64,
    quote: u64,
}
```

### Epoch Info

```rust
pub async fn get_epoch(&self) -> anyhow::Result<CallResponse<(u64, u64)>>
```

Retrieves epoch and its duration.

`self` The SparkMarketContract instance

Returns epoch and epoch duration



### Protocol Fee Info

```rust
pub async fn protocol_fee(&self) -> anyhow::Result<CallResponse<Vec<ProtocolFee>>>
```

Retrieves protocol fee stucture vector.

`self` The SparkMarketContract instance

Returns protocol fee percent, 10_000 == 100%


### Protocol Fee User Info

```rust
pub async fn protocol_fee_user(&self, user: Identity) -> anyhow::Result<CallResponse<Vec<ProtocolFee>>>
```

Retrieves user maker and taker protocol fees.

`self` The SparkMarketContract instance
`user` The user address

Returns protocol maker & taker fee percents, 10_000 == 100%


### Protocol Fee Amount Info

```rust
pub async fn protocol_fee_user_amount(&self, amount: u64, user: Identity) -> anyhow::Result<CallResponse<u64>>
```

Calculates protocol fee amount that needs to be extra order size submission.

`self` The SparkMarketContract instance
`amount` The order size to be submitted
`user` The user address

Returns calculated protocol fee amount


### Matcher Fee Info

```rust
pub async fn matcher_fee(&self) -> anyhow::Result<CallResponse<u64>>
```

Retrieves matcher fee set by Market owner.

`self` The SparkMarketContract instance

Returns matcher fee amount


### User Order Info

```rust
pub async fn order(&self, order: Bits256) -> anyhow::Result<CallResponse<Option<Order>>>
```

Retrieves matcher fee set by Market owner.

`self` The SparkMarketContract instance
`order` Order id

Returns optional order information if order was submitted and wasn't fully matched

```rust
pub struct Order {
    pub amount: u64,
    pub asset_type: AssetType,
    pub order_type: OrderType,
    pub owner: Identity,
    pub price: u64,
    pub block_height: u32,
    pub matcher_fee: u32,
    pub protocol_fee: u64,
}

```


### All User Order IDs Info

```rust
pub async fn user_orders(&self, user: Identity) -> anyhow::Result<CallResponse<Vec<Bits256>>>
```

Retrieves user order ids.

`self` The SparkMarketContract instance
`user` The user address

Returns order ids


### User Order Height Info

```rust
    pub async fn user_order_height(&self, user: Identity) -> anyhow::Result<CallResponse<u64>>
```

Retrieves user order height.

`self` The SparkMarketContract instance
`user` The user address

Returns user order height


### Minimum Order Size Info

```rust
pub async fn min_order_size(&self) -> anyhow::Result<CallResponse<u64>>
```

Retrieves minimum order size set by Market owner.

`self` The SparkMarketContract instance

Returns minimum order size
