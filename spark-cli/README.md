# Overview

This CLI tool allows the user to interact with the market contract.
It enables deployment to the network and various interactions such as opening/closing orders and trading.

> Dev note: Fuels SDK version 0.56.0 does not currently work with Beta-5 so this repo is using version 0.55.0.

# Installation

The CLI is written in [Rust](https://www.rust-lang.org/) and uses Cargo for installation.

Once installed run the following command from inside the `spark-cli` directory to install the CLI.

## Global installation

```bash
cargo install --path .
```

After installation run the following command to verify installation. A help prompt should be displayed upon successful installation.

```bash
spark-cli --help
```

## Build from source

Alternatively, you may build and use the crate without installing it globally.

### Unoptimized

Build the crate.

```bash
cargo build
```

Use it locally by calling it from the installation path.

```bash
../target/debug/spark-cli --help
```

### Optimized

Build the crate.

```bash
cargo build --release
```

Use it locally by calling it from the installation path.

```bash
../target/release/spark-cli --help
```

# Usage

Prior to using the tool we must set an environment variable to allow access to your wallet for the purpose of paying for transactions.

Set the following variable to your wallet secret prior to using the CLI.

```bash
WALLET_SECRET=
```

After setting the environment variable we may use either of the following commands:

- [Core (state changing)](#core-functions)
- [Info (read-only)](#info)

## Core functions

Using the following commands will change the state of the market contract and thus they require a wallet to have sufficient funds to pay the gas for the interaction.

### Deploy

To deploy the contract to the network use the following command

```bash
spark-cli core deploy \
    --base-asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --base-decimals 9 \
    --quote-asset 0x593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746 \
    --quote-decimals 9 \
    --price-decimals 9 \
    --rpc "beta-5.fuel.network"
```

## Deposit

Before opening an order a user must have funds to deposit. Run the following command to deposit into the deployed contract:

```bash
spark-cli core deposit \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --amount 10 \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

## Withdraw

To withdraw unlocked funds from the contract run the following command:

```bash
spark-cli core withdraw \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --amount 10 \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

## Open Order

To open an order run the following command:

> TODO: post example command

## Cancel Order

To close an order run the following command:

> TODO: post example command

## Batch Fulfill

To match orders run the following command:

> TODO: post example command

## Set Fee

To set a fee for a user run the following command:

```bash
spark-cli core set-fee \
    --amount 2 \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

To set a fee for entire market run the following command:

```bash
spark-cli core set-fee \
    --amount 2 \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

To return account information about a user run the following command:

```bash
spark-cli info account \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

## Config

To retrieve configuration information about a market run the following command:

```bash
spark-cli info config \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

## Fee

To view the fee for a specific user run the following command:

```bash
../target/debug/spark-cli info fee \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

To view the fee for entire market contract run the following command:

```bash
spark-cli info fee \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

## Order ID

To create an order ID run the following command:

```bash
spark-cli info order-id \
    --amount 10 \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --order-type buy \
    --owner <your wallet address in hex> \
    --account-type address \
    --price 70000 \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

## Order

To retrieve information regarding an order run the following command:

```bash
spark-cli info order \
    --order-id <ID> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```

## User Orders

To retrieve user orders run the following command:

```bash
spark-cli info user-orders \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id>
```
