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

## Contribution

The liquidation mechanism, error codes, and all contract methods are open for the community to contribute to and improve upon. Your contributions are welcome to make the Spark Contract Orderbook a more robust and feature-rich platform.
