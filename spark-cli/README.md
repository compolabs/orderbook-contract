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
    --rpc "testnet.fuel.network"

Sample output:
Market deployed to: 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Deposit

./target/release/spark-cli core deposit \
    --asset-type base \
    --amount 1 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

./target/release/spark-cli core deposit \
    --asset-type quote \
    --amount 700 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Withdraw

./target/release/spark-cli core withdraw \
    --asset-type base \
    --amount 1 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

./target/release/spark-cli core withdraw \
    --asset-type quote \
    --amount 700 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Open Order

./target/release/spark-cli core open \
    --asset-type base \
    --amount 1 \
    --order-type buy \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

./target/release/spark-cli core open \
    --asset-type base \
    --amount 1 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Cancel Order

./target/release/spark-cli core cancel \
    --order-id aef6701b76abb633bb638c4fb5793237f558c40369cfacaaec30876320551f4e \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Match Order Pair

./target/release/spark-cli core match-pair \
    --orders de8797d3e6a5b36bf93f9cfb0a57282d1cce4dcc525104ffbe10fdc922c21bb7 \
    --orders c76080c47d8efc3fb60afa1b3465157ca809e900e93d9b11101259b19f472c4a \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Match Order Many

./target/release/spark-cli core match-many \
    --orders b3025fcc27a013c6c53912a27e37a8068e442a2173017ee8ad3d43fcd9d30fd7 \
    --orders 45e3b1d17d21964c8668e5e2ba4a88506f751fe3cb3874837550069a69a3e4ee \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

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
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Set Protocol Fee

Sets protocol fee

./target/release/spark-cli core set-protocol-fee \
    --amount 2 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Set Matcher Fee

Sets a matcher fee for the market

./target/release/spark-cli core set-matcher-fee \
    --amount 1000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

./target/release/spark-cli info account \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Config

./target/release/spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Fee

Fee for a specific user

./target/release/spark-cli info fee \
    --account-type address \
    --account-id 0x4ea14e5787d00813f944c744a52cb40d4b5293315b448a76d16110b8b9da0cba \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

Fee for entire market contract

./target/release/spark-cli info fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Matcher Fee

Matcher Fee for the market

./target/release/spark-cli info matcher-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Order ID

./target/release/spark-cli info order-id \
    --asset-type  base \
    --order-type sell \
    --owner 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --account-type address \
    --price 70000000000000 \
    --block-height 10000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## Order

./target/release/spark-cli info order \
    --order-id 3c9cd1539a2e1ee85f10c0faff25cab648af51a779d1236cbb8ee2d2ec27cd1f \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

## User Orders

./target/release/spark-cli info user-orders \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19

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
    --market 0xf3b44eaeed7c31eb41f3df10fecf07b5c2f3585e32d22230c93a2f68b0c24b19 \
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

