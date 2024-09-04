# Spark Orderbook Contract

The Spark Orderbook Contract is a decentralized order book implemented on the Fuel blockchain, designed to facilitate secure and transparent trading.

## Getting Started

The Orderbook framework consists of five components, which can be found in the root folder:

- **`spark-market`:** This Sway contract contains all the trading logic, including functionalities for opening, closing, and matching orders. It serves as the core of the trading operations, with corresponding tests included.
- **`spark-registry`:** This Sway contract allows for the registration of new markets, specifically for asset trading pairs. It acts as a registry that keeps track of all available markets, with tests included.
- **`spark-cli`:** Spark CLI tools for deploying and interacting with contracts. Detailed information is available in the README file within the subfolder.
- **`spark-market-sdk`:** A Rust library (SDK) for interacting with the SparkMarket contract. Additional details can be found in the README file within its subfolder.
- **`spark-registry-sdk`:** A Rust library (SDK) for interacting with the SparkRegistry contract. More information is available in the README file within its subfolder.


## Running All Tests

To build the project and run all tests, execute the following command:

```
forc build --release & cargo test --release
```

### Running Fuzz Tests:

To run fuzz tests, use the following command:

```
cargo test --release -- --ignored fuzz --test-threads=$(nproc || sysctl -n hw.ncpu)
```

## Contribution

The liquidation mechanism, error codes, and all contract methods are open for community contributions. Your input is welcome to help improve and expand the Spark Orderbook Contract, making it a more robust and feature-rich platform.
