Note: Work in progress, incomplete, use as guide only, only tested in testnet
Note: Fuels SDK version 0.56.0 does not currently work with testnet so the repo is using 0.65.0

Inside the spark project (CLI) run the following commands

# Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

Run from project root folder

## Deploy

./target/release/spark-cli core deploy \
    --base-asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --base-decimals 8 \
    --quote-asset 0xfed3ee85624c79cb18a3a848092239f2e764ed6b0aa156ad10a18bfdbe74269f \
    --quote-decimals 6 \
    --price-decimals 9 \
    --fuel-asset 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07 \
    --rpc "testnet.fuel.network"

Sample output:
Market deployed to: 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Deposit

./target/release/spark-cli core deposit \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

./target/release/spark-cli core deposit \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Withdraw

./target/release/spark-cli core withdraw \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

./target/release/spark-cli core withdraw \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Open Order

./target/release/spark-cli core open \
    --asset-type base \
    --amount 10 \
    --order-type buy \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

./target/release/spark-cli core open \
    --asset-type base \
    --amount 10 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Cancel Order

./target/release/spark-cli core cancel \
    --order-id aef6701b76abb633bb638c4fb5793237f558c40369cfacaaec30876320551f4e \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Match Order Pair

./target/release/spark-cli core match-pair \
    --orders de8797d3e6a5b36bf93f9cfb0a57282d1cce4dcc525104ffbe10fdc922c21bb7 \
    --orders c76080c47d8efc3fb60afa1b3465157ca809e900e93d9b11101259b19f472c4a \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Match Order Many

./target/release/spark-cli core match-many \
    --orders b3025fcc27a013c6c53912a27e37a8068e442a2173017ee8ad3d43fcd9d30fd7 \
    --orders 45e3b1d17d21964c8668e5e2ba4a88506f751fe3cb3874837550069a69a3e4ee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Fulfill Order Many

./target/release/spark-cli core fulfill-many \
    --asset-type base \
    --amount 2 \
    --order-type sell \
    --price 70000000000000 \
    --slippage 100 \
    --orders d2e1011c3fd8755d54d7887b8c4c2f3e5c22f16f00654e0d3b894792fcb50075 \
    --orders 1d958e39a4102e2a2db332bec485095cac5ad4b6249083818d59f3c4da575aa4 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Set Protocol Fee

Sets protocol fee

./target/release/spark-cli core set-protocol-fee \
    --amount 0 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Set Matcher Fee

Sets a matcher fee for the market

./target/release/spark-cli core set-matcher-fee \
    --amount 1000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Withdraw Protocol Fee

Withdraw protocol fee

./target/release/spark-cli core withdraw-protocol-fee \
    --account-to-type address \
    --account-to-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

./target/release/spark-cli info account \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Config

./target/release/spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Protocol Fee

Protocol fee

./target/release/spark-cli info protocol-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Total Protocol Fee

Total Protocol fee collected

./target/release/spark-cli info total-protocol-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Protocol Fee Amount

Calculates Protocol fee for a order amount

./target/release/spark-cli info protocol-fee-amount \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Matcher Fee

Matcher Fee for the market

./target/release/spark-cli info matcher-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Order ID

./target/release/spark-cli info order-id \
    --asset-type  base \
    --order-type sell \
    --owner 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --account-type address \
    --price 70000000000000 \
    --block-height 10000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Order

./target/release/spark-cli info order \
    --order-id 769663aef01812de5e5b4a4cd96f31a1641d4924cd26bdf7665fc00708487007 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## User Orders

./target/release/spark-cli info user-orders \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08

## Deploy Orderbook

./target/release/spark-cli book deploy \
    --rpc "testnet.fuel.network" 

Output:
Orderbook deployed to: 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Register a market

./target/release/spark-cli book register \
    --asset 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418 \
    --market 0x352b69f5198e37a935c745563dc7d6e73c0d77dc61725fa0bf9483ee4b5e6a08 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Unregister a market

./target/release/spark-cli book unregister \
    --asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Get registered markets by assets

./target/release/spark-cli book markets \
    --assets 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --assets 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Config

./target/release/spark-cli book config \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

