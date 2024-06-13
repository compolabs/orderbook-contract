Note: Work in progress, incomplete, use as guide only, only tested in testnet
Note: Fuels SDK version 0.56.0 does not currently work with testnet so the repo is using 0.55.0

Inside the spark project (CLI) run the following commands

# Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

Run from `spark-cli` folder

## Deploy

../target/debug/spark-cli core deploy \
    --base-asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --base-decimals 8 \
    --quote-asset 0xfed3ee85624c79cb18a3a848092239f2e764ed6b0aa156ad10a18bfdbe74269f \
    --quote-decimals 6 \
    --price-decimals 9 \
    --rpc "testnet.fuel.network"

Sample output:
Market deployed to: 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Deposit

../target/debug/spark-cli core deposit \
    --asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --amount 100 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

## Withdraw (bugged)

../target/debug/spark-cli core withdraw \
    --asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --amount 100 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

## Open Order (not implemented)

../target/debug/spark-cli core open \
    --asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --amount 10 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

## Cancel Order (not implemented)

../target/debug/spark-cli core cancel \
    --order-id e3e990df5aa7f62121d94b5967c0ff78030617ee4dae3e6de1476589648c9dea \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

## Match Order Pair (not implemented)

## Set Fee

Sets a fee for a user

../target/debug/spark-cli core set-fee \
    --amount 2 \
    --account-type address \
    --account-id 0x4ea14e5787d00813f944c744a52cb40d4b5293315b448a76d16110b8b9da0cba \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

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
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

## Config

../target/debug/spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

## Fee

Fee for a specific user

../target/debug/spark-cli info fee \
    --account-type address \
    --account-id 0x4ea14e5787d00813f944c744a52cb40d4b5293315b448a76d16110b8b9da0cba \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

Fee for entire market contract

../target/debug/spark-cli info fee \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Order ID

../target/debug/spark-cli info order-id \
    --amount 10 \
    --asset-type  base \
    --order-type sell \
    --owner 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --account-type address \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

## Order

../target/debug/spark-cli info order \
    --order-id e3e990df5aa7f62121d94b5967c0ff78030617ee4dae3e6de1476589648c9dea \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5

## User Orders

../target/debug/spark-cli info user-orders \
    --account-type address \
    --account-id <your wallet address in hex> \
    --rpc "testnet.fuel.network" \
    --contract-id <contract-id here>

## Deploy Orderbook

../target/debug/spark-cli book deploy \
    --rpc "testnet.fuel.network" 

Output:
Orderbook deployed to: 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Register a market

../target/debug/spark-cli book register \
    --asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --market 0x0d62c0861a51e052566b39540b0b8078cd03b53490d59e28913779ac997c12b5 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Unregister a market

../target/debug/spark-cli book unregister \
    --asset 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Get registered markets by assets

../target/debug/spark-cli book markets \
    --assets 0xccceae45a7c23dcd4024f4083e959a0686a191694e76fa4fb76c449361ca01f7 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

## Config

../target/debug/spark-cli book config \
    --rpc "testnet.fuel.network" \
    --contract-id 0xf88a9eea6c45e2a77cf2f684fbb7d175cbf8cf58c079d07f9d24f51329de4418

