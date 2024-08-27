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
Market deployed to: 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Deposit

./target/release/spark-cli core deposit \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

./target/release/spark-cli core deposit \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Withdraw

./target/release/spark-cli core withdraw \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

./target/release/spark-cli core withdraw \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Open Order

./target/release/spark-cli core open \
    --amount 10 \
    --order-type buy \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

./target/release/spark-cli core open \
    --amount 10 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Cancel Order

./target/release/spark-cli core cancel \
    --order-id e950192bd177292dd7b98c69e6f85a46f5d59d93a0ba2f84af1f9d06d1fdf821 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Match Order Pair

./target/release/spark-cli core match-pair \
    --orders dcd473a361832d1c479d8c3ae498390cdb69d1e96ebacf5a779522935299b2f4 \
    --orders 130aa3aece55b47c5b723f33c1c25dce99ff689382173d40188043df19bbf9be \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Match Order Many

./target/release/spark-cli core match-many \
    --orders 0a96241df0a2606ead475af4cf66f89097bcbec27fdb59ff5cdb30a7525393e2 \
    --orders 2a6273b795e682f9fc4723097e682e0097c29c16f0419d7dc6132f77151e27ca \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Fulfill Order Many

./target/release/spark-cli core fulfill-many \
    --amount 2 \
    --order-type sell \
    --limit-type ioc \
    --price 70000000000000 \
    --slippage 100 \
    --orders 0d0d7540d7350222b39a453452067cb3e1d1a29773a1678293771c9a0a12fe6f \
    --orders 12e7c70e34d437960fe455ce41ee9f839a93f5b317d19a9708c3ef51dffb89d0 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Set Protocol Fee

Sets protocol fee

./target/release/spark-cli core set-protocol-fee \
    --fee "0,0,0" \
    --fee "100,150,10000" \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Set Matcher Fee

Sets a matcher fee for the market

./target/release/spark-cli core set-matcher-fee \
    --amount 1000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Set Epoch

Sets a epoch and duration for the market

./target/release/spark-cli core set-epoch \
    --epoch 4611686020152148916 \
    --epoch-duration 1000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7





# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

./target/release/spark-cli info account \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Config

./target/release/spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Epoch

./target/release/spark-cli info epoch \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Protocol Fee

Protocol fee

./target/release/spark-cli info protocol-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Protocol Fee User

Gets Protocol user fee

./target/release/spark-cli info protocol-fee-user \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Protocol Fee User Amount

Calculates Protocol user fee for a order amount

./target/release/spark-cli info protocol-fee-user-amount \
    --amount 10 \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Matcher Fee

Matcher Fee for the market

./target/release/spark-cli info matcher-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Order ID

./target/release/spark-cli info order-id \
    --order-type sell \
    --owner 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --account-type address \
    --price 70000000000000 \
    --block-height 10000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## Order

./target/release/spark-cli info order \
    --order-id 769663aef01812de5e5b4a4cd96f31a1641d4924cd26bdf7665fc00708487007 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

## User Orders

./target/release/spark-cli info user-orders \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7

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
    --market 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0713334e61ed73ba9421a3a49891953f9ccb7353828566b569752a82a39803e8

## Unregister a market

./target/release/spark-cli book unregister \
    --market 0x67632a67764130dc9d14e4213c5a5e1ccfe51f184e8789fa68c7f34447c671a7 \
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

