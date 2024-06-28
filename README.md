# Spark Orderbook Contracts

The Spark Contract Orderbook is a decentralized order book implementation on the Fuel blockchain, designed to facilitate trading in a secure and transparent manner.

## Integration Constants
https://github.com/compolabs/orderbook-contract/tree/testnet [dev - https://github.com/compolabs/orderbook-contract/tree/feat-contract-split]

## Features

- **Market Deployment**: Enables the creation of a new market for different asset pairs.
- **Orderbook Deployment**: Enables the creation of a new orderbook.
- **Market Management**: Allows owner to register / unregister markets.
- **Order Management**: Allows users to open, cancel, and match orders efficiently.
- **Info Utilities**: Provides functions to retrieve orders by trader and check market existence.

## Getting Started

The framework consist of two contracts:

Market contract
The main contract contains order managemnt logic.

Orderbook contract
Just a market contract_id container. Allows register and keep market addresses in one storage and get market contract_id by asset_id.

To start with the project:

1. Compile contracts by runing `forc build --release`
2. Build test environment `cargo build --release`
3. Run tests `cargo test --release`

The project contains cli command subproject
`cd spark-cli` and follow `spark-cli/README.md` commands (deploy, open_order etc).
Run commands from `spark-cli` folder.

## Spark SDK

The project has two `spark-market-sdk` and `spark-orderbook-sdk` SDKs.
It mightbe used for contract deployment & interaction.

## Contribution

The liquidation mechanism, error codes, and all contract methods are open for the community to contribute to and improve upon. Your contributions are welcome to make the Spark Contract Orderbook a more robust and feature-rich platform.
