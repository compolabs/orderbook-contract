Note: Work in progress, incomplete, use as guide only, only tested in beta-5

Inside the spark project (CLI) run the following commands

# Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

## Deploy

../target/debug/spark core deploy \
    --base-asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --base-decimals 9 \
    --quote-asset 0x593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746 \
    --quote-decimals 9 \
    --price-decimals 9 \
    --rpc "beta-5.fuel.network"

## Deposit

../target/debug/spark core deposit \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --amount 10 \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

## Withdraw (bugged)

../target/debug/spark core withdraw \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --amount 10 \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

## Open Order (not implemented)

## Cancel Order (not implemented)

## Batch Fulfill (not implemented)

## Set Fee

Sets a fee for a user

../target/debug/spark core set-fee \
    --amount 2 \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

Set fee for entire market

../target/debug/spark core set-fee \
    --amount 2 \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

../target/debug/spark info account \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

## Config

../target/debug/spark info config \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

## Fee

Fee for a specific user

../target/debug/spark info fee \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

Fee for entire market contract

../target/debug/spark info fee \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

## Order ID

../target/debug/spark info order-id \
    --amount 10 \
    --asset 0x0000000000000000000000000000000000000000000000000000000000000000 \
    --order-type buy \
    --owner <your wallet address in hex> \
    --account-type address \
    --price 70000 \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

## Order

../target/debug/spark info order \
    --order-id <ID> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>

## User Orders

../target/debug/spark info user-orders \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "beta-5.fuel.network" \
    --contract-id <contract-id here>
