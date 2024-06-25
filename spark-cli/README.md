Note: Work in progress, incomplete, use as guide only, only tested in testnet
Note: Fuels SDK version 0.56.0 does not currently work with testnet so the repo is using 0.64.0

Inside the spark project (CLI) run the following commands

# Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

Run from `spark-cli` folder

## Deploy

../target/release/spark-cli core deploy \
    --base-asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --base-decimals 8 \
    --quote-asset 0xfed3ee85624c79cb18a3a848092239f2e764ed6b0aa156ad10a18bfdbe74269f \
    --quote-decimals 6 \
    --price-decimals 9 \
    --rpc "testnet.fuel.network"

Sample output:
Market deployed to: 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Deposit

../target/release/spark-cli core deposit \
    --asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --amount 1 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

../target/release/spark-cli core deposit \
    --asset 0xfed3ee85624c79cb18a3a848092239f2e764ed6b0aa156ad10a18bfdbe74269f \
    --amount 700 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## Withdraw

../target/release/spark-cli core withdraw \
    --asset-type base \
    --amount 100 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## Open Order

../target/release/spark-cli core open \
    --asset-type base \
    --amount 1 \
    --order-type buy \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

../target/release/spark-cli core open \
    --asset-type base \
    --amount 1 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x1b01206d39aa298c8d8373c23b6a2c07cda5be0a142cc642518fd8d9d6bbfc7f

## Cancel Order

../target/release/spark-cli core cancel \
    --order-id 3c9cd1539a2e1ee85f10c0faff25cab648af51a779d1236cbb8ee2d2ec27cd1f \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## Match Order Pair

../target/release/spark-cli core match-pair \
    --orders 91b3e452dc537663d2970017831b7aeea5a77e7c308a6a2c4ab0951c47a11009 \
    --orders 485adfbcdb980899bf460be61c21772e1771c20f310b45210dcd69ddddd26d16 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## Match Order Many

../target/release/spark-cli core match-many \
    --orders 178a812dab4714a07c297cde72ab43266db9a75106da5468f0ec813de3384237 \
    --orders bd2778ddc257433fd1dbd3dfa87dc628a01da78cf37419f0ae65907aaae3cd92 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## Fulfill Order Many

../target/release/spark-cli core fulfill-many \
    --asset-type base \
    --amount 1 \
    --order-type sell \
    --price 70000000000000 \
    --slippage 100 \
    --orders 178a812dab4714a07c297cde72ab43266db9a75106da5468f0ec813de3384237 \
    --orders bd2778ddc257433fd1dbd3dfa87dc628a01da78cf37419f0ae65907aaae3cd92 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## Set Fee

Sets a fee for a user

../target/release/spark-cli core set-fee \
    --amount 2 \
    --account-type address \
    --account-id 0x4ea14e5787d00813f944c744a52cb40d4b5293315b448a76d16110b8b9da0cba \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

Set fee for entire market

../target/release/spark-cli core set-fee \
    --amount 2 \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

../target/release/spark-cli info account \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x1b01206d39aa298c8d8373c23b6a2c07cda5be0a142cc642518fd8d9d6bbfc7f

## Config

../target/release/spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## Fee

Fee for a specific user

../target/release/spark-cli info fee \
    --account-type address \
    --account-id 0x4ea14e5787d00813f944c744a52cb40d4b5293315b448a76d16110b8b9da0cba \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

Fee for entire market contract

../target/release/spark-cli info fee \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Order ID

../target/release/spark-cli info order-id \
    --asset-type  base \
    --order-type sell \
    --owner 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --account-type address \
    --price 70000000000000 \
    --block-height 10000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## Order

../target/release/spark-cli info order \
    --order-id 3c9cd1539a2e1ee85f10c0faff25cab648af51a779d1236cbb8ee2d2ec27cd1f \
    --rpc "testnet.fuel.network" \
    --contract-id 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a

## User Orders

../target/release/spark-cli info user-orders \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x1b01206d39aa298c8d8373c23b6a2c07cda5be0a142cc642518fd8d9d6bbfc7f

## Deploy Orderbook

../target/release/spark-cli book deploy \
    --rpc "testnet.fuel.network" 

Output:
Orderbook deployed to: 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Register a market

../target/release/spark-cli book register \
    --asset 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418 \
    --market 0x08ca18ed550d6229f001641d43aac58e00f9eb7e25c9bea6d33716af61e43b2a \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Unregister a market

../target/release/spark-cli book unregister \
    --asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Get registered markets by assets

../target/release/spark-cli book markets \
    --assets 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --assets 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Config

../target/release/spark-cli book config \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

