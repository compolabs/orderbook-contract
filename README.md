# Spark Orderbook Contract

The Spark Orderbook Contract is a decentralized order book implementation on the Fuel blockchain, designed to facilitate trading in a secure and transparent manner.


## Getting Started

The Ordebook framework consist of five parts might by found in root folder.

`market-contract` Sway Market contract with tests.
`orderbook-contract` Sway Orderbook contract is just a collection of registered market contract with tests.
`spark-cli` Spark CLI commands to deploy contract and interact with them. Detailed information might be found in Readme.md in subfolder.
`spark-market-sdk` Spark Market SDK is a Rust library for the Market contract interaction. Detailed information is in Readme.md in subfolder.
`spark-orderbook-sdk` Spark Orderbook SDK is a Rust library for the Orderbook contract interaction. Detailed information is in Readme.md.


## Running All Tests

```
forc build --release & cargo test --release
```

### Running Fuzz Tests:
```
cargo test --release -- --ignored fuzz
```

## Contribution

The liquidation mechanism, error codes, and all contract methods are open for the community to contribute to and improve upon. Your contributions are welcome to make the Spark Contract Orderbook a more robust and feature-rich platform.
