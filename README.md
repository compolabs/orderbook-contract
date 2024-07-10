# Spark Contract Orderbook

The Spark Contract Orderbook is a decentralized order book implementation on the Fuel blockchain, designed to facilitate trading in a secure and transparent manner.

## Integration Constants
https://github.com/compolabs/orderbook-contract/blob/master/src/constants.rs

## Main contract file
https://github.com/compolabs/orderbook-contract/blob/master/contract/src/main.sw

## Features

- **Market Creation**: Enables the creation of new markets for different asset pairs.
- **Order Management**: Allows users to open, cancel, and match orders efficiently.
- **Query Utilities**: Provides functions to retrieve orders by trader and check market existence.

## Methods

### Market Management
- `create_market`: Registers a new market with a specified asset ID and decimal precision. This is a crucial initial step for asset trading.
- `market_exists`: Checks if a market for a given asset ID already exists.

### Order Operations
- `open_order`: Opens a new order with specified base token, size, and price. Requires a payment to be attached.
- `cancel_order`: Cancels an existing order by its unique identifier.
- `match_orders`: Matches sell and buy orders, allowing trades to be executed.

### Queries
- `orders_by_trader`: Retrieves all orders associated with a specific trader address.
- `order_by_id`: Fetches a single order by its unique identifier.

## Error Handling

The contract is equipped with a robust error handling system to prevent and inform about various operational issues:

- `AccessDenied`: Unauthorized access attempt to a function.
- `NoOrdersFound`: No orders matching the query were found.
- `NoMarketFound`: Specified market does not exist.
- `OrdersCantBeMatched`: Orders cannot be matched due to incompatible conditions.
- `BadAsset`: Incorrect asset ID provided.
- `BadValue`: Value provided does not match the expected.
- `BadPrice`: Incorrect or zero price provided for an order.
- `MarketAlreadyExists`: A market with the provided asset ID is already registered.

## Structures

- `Order`: Represents a trade order with an ID, trader address, base token, size, and price.
- `Market`: Defines a market with an asset ID and decimal precision for the asset.

## Getting Started

To use this contract:

1. Deploy the contract on the Fuel blockchain.
2. Create a market using `create_market`.
3. Begin trading by opening orders with `open_order`.
4. Match orders using `match_orders`.
5. Cancel orders if needed with `cancel_order`.

For a detailed explanation of each method and error, please refer to the inline comments within the contract code.


# Trader SDK Documentation

## Installation

To integrate the SDK, add the following to your `Cargo.toml`:

```toml
[dependencies]
orderbook = { git = "https://github.com/compolabs/orderbook-contract.git", branch = "master" }
```

## Order Fields

An order on the contract consists of the following fields:

- `id: b256` - This is a hash derived from `trader`, `base_token`, and `base_price`.
- `trader: Address` - The address of the person who created the order.
- `base_token: AssetId` - Each order is opened with respect to USD. For example, if you want to buy BTC for USD or sell BTC for USD, the `base_token` will be BTC.
- `base_size: i64` - This is the size of the order. For optimization, if `base_size` is positive, it's a buy order, and if it's negative, it's a sell order.
- `base_price: u64` - This is the price at which the trader placed the order.

## GraphQL API

All order book read operations are available through our GraphQL API.

### Example: Fetching Markets

```rust
use serde_json::json;

async fn get_spot_market_create_events() -> Result<Vec<SpotMarketCreateEvent>, Box<dyn std::error::Error>> {
    let query = json!({
        "query": r#"
            query SpotMarketCreateEventQuery {
                SpotMarketCreateEvent {
                    id,
                    asset_id,
                    asset_decimals,
                    timestamp,
                }
            }
        "#
    });

    let client = reqwest::Client::new();
    let response = client.post("YOUR_GRAPHQL_ENDPOINT")
        .json(&query)
        .send()
        .await?
        .json::<GraphqlResponse<Vec<SpotMarketCreateEvent>>>()
        .await?;

    Ok(response.data.SpotMarketCreateEvent)
}
```

### Example: Fetching Orders

```rust
use serde_json::json;

async fn get_spot_orders(params: SpotOrdersParams) -> Result<Vec<SpotOrder>, Box<dyn std::error::Error>> {
    let mut where_filter = r#"base_size: {_neq: "0"}"#.to_string();

    if let Some(order_type) = params.order_type {
        where_filter = format!(r#"order_type: {{_eq: "{}"}}, {}"#, order_type.to_lowercase(), where_filter);
    }
    if params.is_opened {
        where_filter = format!(r#"base_price: {{_neq: "0"}}, {}"#, where_filter);
    }
    if let Some(trader) = params.trader {
        where_filter = format!(r#"trader: {{_eq: "{}"}}, {}"#, trader, where_filter);
    }
    if let Some(base_token) = params.base_token {
        where_filter = format!(r#"base_token: {{_eq: "{}"}}, {}"#, base_token, where_filter);
    }

    let order_type = if params.order_type.as_deref() == Some("buy") { "desc" } else { "asc" };

    let query = json!({
        "query": format!(
            r#"
            query SpotOrderQuery {{
                SpotOrder(limit: {}, where: {{{}}}, order_by: {{base_price: {}}}) {{
                    id,
                    trader, 
                    order_type,
                    base_token,
                    base_size,
                    base_price,
                    timestamp,
                }}
            }}
            "#,
            params.limit.unwrap_or(100), // Default limit
            where_filter,
            order_type
        )
    });

    let client = reqwest::Client::new();
    let response = client.post("YOUR_GRAPHQL_ENDPOINT")
        .json(&query)
        .send()
        .await?
        .json::<GraphqlResponse<Vec<SpotOrder>>>()
        .await?;

    Ok(response.data.SpotOrder)
}
```

## SDK Methods

### Creating an Order

```rust
pub async fn open_order(
    &self,
    base_token: AssetId,
    base_size: i64,
    base_price: u64,
) -> Result<CallResponse<Bits256>, fuels::types::errors::Error>
```

- `base_token: AssetId` - The asset identifier for the base token.
- `base_size: i64` - The size of the order. Positive for buy orders, negative for sell orders.
- `base_price: u64` - The price at which to place the order.

This method opens a new order in the order book.

### Canceling an Order

```rust
pub async fn cancel_order(
    &self,
    order_id: &Bits256,
) -> Result<CallResponse<()>, fuels::types::errors::Error>
```

- `order_id: &Bits256` - The identifier of the order to be canceled.

This method cancels an existing order in the order book.

## Example Usage of the SDK to Create an Order

```rust
use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Address, ContractId},
};
use orderbook::{
    constants::{ORDERBOOK_CONTRACT_ID, RPC, TOKEN_CONTRACT_ID},
    orderbook_utils::Orderbook,
    print_title,
};
use src20_sdk::token_utils::{Asset, TokenContract};
use std::{env, str::FromStr};

const MARKET_SYMBOL: &str = "BTC";
const BASE_SIZE: i64 = 1; // units
const BASE_PRICE: u64 = 69001; // units

#[tokio::main]
async fn main() {
    print_title("Create Order");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));
    println!("wallet address = {:?}", wallet.address());
    let token_contract = TokenContract::new(
        &ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into(),
        wallet.clone(),
    );

    let token_contract_id = token_contract.contract_id().into();
    let base_asset = Asset::new(wallet.clone(), token_contract_id, MARKET_SYMBOL);
    let quote_asset = Asset::new(wallet.clone(), token_contract_id, "USDC");

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    if BASE_SIZE > 0 {
        let quote_size = quote_asset.parse_units(BASE_SIZE as f64 * BASE_PRICE as f64);
        quote_asset
            .mint(wallet.address().into(), quote_size as u64)
            .await
            .unwrap();
    } else {
        let base_size = base_asset.parse_units(BASE_SIZE.abs() as f64) as u64;
        base_asset
            .mint(wallet.address().into(), base_size)
            .await
            .unwrap();
    }

    let price = BASE_PRICE * 10u64.pow(orderbook.price_decimals as u32);

    match orderbook
        .open_order(base_asset.asset_id, BASE_SIZE, price)
        .await
    {
        Ok(response) => {
            let id = Address::from(response.value.0).to_string();
            println!("Order opened successfully. OrderId: 0x{id}");
            println!("Gas Used: {:?}", response.gas_used);
            println!("Transaction ID: 0x{:?}", response.tx_id.unwrap());
        }
        Err(error) => {
            eprintln!("Failed to open order: {:?}", error);
        }
    }
}
```

This SDK documentation provides a detailed guide on integrating and using the SDK for trading purposes, including setup instructions, field descriptions, GraphQL queries, and example usage.

## Contribution

The liquidation mechanism, error codes, and all contract methods are open for the community to contribute to and improve upon. Your contributions are welcome to make the Spark Contract Orderbook a more robust and feature-rich platform.
