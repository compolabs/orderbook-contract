Note: Work in progress, incomplete, use as guide only, only tested in testnet
Note: Fuels SDK version 0.56.0 does not currently work with testnet so the repo is using 0.65.0

Inside the spark project (CLI) run the following commands

# Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

Run from project root folder

## Deploy

./target/release/spark-cli core deploy \
    --base-asset 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc \
    --base-decimals 8 \
    --quote-asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --quote-decimals 6 \
    --price-decimals 9 \
    --fuel-asset 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07 \
    --rpc "testnet.fuel.network"

Sample output:
Market deployed to: 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Deposit

./target/release/spark-cli core deposit \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

./target/release/spark-cli core deposit \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Withdraw

./target/release/spark-cli core withdraw \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

./target/release/spark-cli core withdraw \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Open Order

./target/release/spark-cli core open \
    --amount 10 \
    --order-type buy \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

./target/release/spark-cli core open \
    --amount 10 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Cancel Order

./target/release/spark-cli core cancel \
    --order-id aef6701b76abb633bb638c4fb5793237f558c40369cfacaaec30876320551f4e \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Match Order Pair

./target/release/spark-cli core match-pair \
    --orders 3cd86a9b7fd3d97bae1ec5cb09835a9bc5a1ebd42fe7da9a44d4433e1b8fafe0 \
    --orders 6ef3142fc8e582cc5b1d7ae474e5ed2f4d3e471b0a43f9e7bd201bc91d9f2189 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Match Order Many

./target/release/spark-cli core match-many \
    --orders fc1340a71df69d2c3ee8f939a8fa8baa5f22647090b659addc694f2e918d9666 \
    --orders 4999b7f3305bf0719d73e8e96b2bd023c7c5472f6e8a282411c418b0328679f6 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Fulfill Order Many

./target/release/spark-cli core fulfill-many \
    --amount 2 \
    --order-type sell \
    --limit-type ioc \
    --price 70000000000000 \
    --slippage 100 \
    --orders d2e1011c3fd8755d54d7887b8c4c2f3e5c22f16f00654e0d3b894792fcb50075 \
    --orders 1d958e39a4102e2a2db332bec485095cac5ad4b6249083818d59f3c4da575aa4 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Set Protocol Fee

Sets protocol fee

./target/release/spark-cli core set-protocol-fee \
    --amount 0 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Set Matcher Fee

Sets a matcher fee for the market

./target/release/spark-cli core set-matcher-fee \
    --amount 1000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Withdraw Protocol Fee

Withdraw protocol fee

./target/release/spark-cli core withdraw-protocol-fee \
    --account-to-type address \
    --account-to-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

./target/release/spark-cli info account \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Config

./target/release/spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Protocol Fee

Protocol fee

./target/release/spark-cli info protocol-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Total Protocol Fee

Total Protocol fee collected

./target/release/spark-cli info total-protocol-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Protocol Fee Amount

Calculates Protocol fee for a order amount

./target/release/spark-cli info protocol-fee-amount \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Matcher Fee

Matcher Fee for the market

./target/release/spark-cli info matcher-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Order ID

./target/release/spark-cli info order-id \
    --order-type sell \
    --owner 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --account-type address \
    --price 70000000000000 \
    --block-height 10000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Order

./target/release/spark-cli info order \
    --order-id 769663aef01812de5e5b4a4cd96f31a1641d4924cd26bdf7665fc00708487007 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## User Orders

./target/release/spark-cli info user-orders \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa

## Deploy Orderbook

./target/release/spark-cli book deploy \
    --rpc "testnet.fuel.network" 

Output:
Orderbook deployed to: 0x0713334e61ed73ba9421a3a49891953f9ccb7353828566b569752a82a39803e8
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Register a market

./target/release/spark-cli book register \
    --market 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0713334e61ed73ba9421a3a49891953f9ccb7353828566b569752a82a39803e8

## Unregister a market

./target/release/spark-cli book unregister \
    --market 0x58959d086d8a6ee8cf8eeb572b111edb21661266be4b4885383748d11b72d0aa \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0713334e61ed73ba9421a3a49891953f9ccb7353828566b569752a82a39803e8

## Get registered markets by assets

./target/release/spark-cli book markets \
    --base 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --quote 0xfed3ee85624c79cb18a3a848092239f2e764ed6b0aa156ad10a18bfdbe74269f \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0713334e61ed73ba9421a3a49891953f9ccb7353828566b569752a82a39803e8

## Config

./target/release/spark-cli book config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0713334e61ed73ba9421a3a49891953f9ccb7353828566b569752a82a39803e8

