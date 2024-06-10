Note: Work in progress, incomplete, use as guide only, only tested in testnet
Note: Fuels SDK version 0.56.0 does not currently work with testnet so the repo is using 0.55.0

Inside the spark project (CLI) run the following commands

# Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

Run from `spark-cli` folder

## Deploy

../target/debug/spark-cli core deploy \
    --base-asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --base-decimals 9 \
    --quote-asset 0x593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746 \
    --quote-decimals 9 \
    --price-decimals 9 \
    --rpc "testnet.fuel.network"

## Deposit

../target/debug/spark-cli core deposit \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Withdraw (bugged)

../target/debug/spark-cli core withdraw \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Open Order (not implemented)

## Cancel Order (not implemented)

## Batch Fulfill (not implemented)

## Set Fee

Sets a fee for a user

../target/debug/spark-cli core set-fee \
    --amount 2 \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

Set fee for entire market

../target/debug/spark-cli core set-fee \
    --amount 2 \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

../target/debug/spark-cli info account \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Config

../target/debug/spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Fee

Fee for a specific user

../target/debug/spark-cli info fee \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

Fee for entire market contract

../target/debug/spark-cli info fee \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Order ID

../target/debug/spark-cli info order-id \
    --amount 10 \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --order-type buy \
    --owner <your wallet address in hex> \
    --account-type address \
    --price 70000 \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Order

../target/debug/spark-cli info order \
    --order-id <ID> \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## User Orders

../target/debug/spark-cli info user-orders \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Deploy Orderbook

../target/debug/spark-cli book deploy \
    --rpc "testnet.fuel.network" 

## Register a market

../target/debug/spark-cli book register \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --market <contract-id here> \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Unregister a market

../target/debug/spark-cli book unregister \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>
