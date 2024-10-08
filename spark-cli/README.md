Note: Work in progress, incomplete, use as guide only, only tested in testnet

Inside the spark project (CLI) run the following commands

# Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

Run from project root folder

## Setup

```
cargo install spark-cli
```

Create `.env` file in the project root and initialize `WALLET_SECRET=` with Fuel private key value, or initialize `MNEMONIC=` to use a 12 word mnemonic.

## Deploy

ETH address 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07
BTC address 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc
USDC address 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05

### Batch Deploy

Deploys BTC/USDC, ETH/USDC markets. Setup fees and epoches. Deploy registry and register markets.

```
spark-cli batch deploy-all \
    --rpc "testnet.fuel.network"
```

Sample output:
Spark CLI v0.6.0

BTC/USDC Market version 0.6.0 (1536) deployed to: 0x006377122bdcdf5b645a773c55140dc9dfb7878a17150116aea9e8420eb02d88
Deployment cost: 758

ETH/USDC Market version 0.6.0 (1536) deployed to: 0x0fa62a46c633726d31976a20f74ce6693cf9d764bbe2c2128250a17420fb047e
Deployment cost: 758

MarketRegistry version 0.6.0 (1536) deployed to: 0x5085ba483a9a278130d566c603bb93862b16040b86ad871c568338c87e18490c
Deployment cost: 934

### Deploy BTC-USDC market

```
spark-cli core deploy \
    --base-asset 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc \
    --base-decimals 8 \
    --quote-asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --quote-decimals 6 \
    --price-decimals 9 \
    --rpc "testnet.fuel.network"
```

Sample output:
Market deployed to: 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

### Deploy ETH-USDC market

```
spark-cli core deploy \
    --base-asset 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07 \
    --base-decimals 9 \
    --quote-asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --quote-decimals 6 \
    --price-decimals 9 \
    --rpc "testnet.fuel.network"
```

Sample output:
SparkMarket deployed to: 0xc18094a283193c9b4726d2f644ed07ec9806bbe60a0688d45bffb26c379c1428
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Deposit

```
spark-cli core deposit \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

```
spark-cli core deposit \
    --asset-type quote \
    --amount 10000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Deposit For

```
spark-cli core deposit-for \
    --asset-type base \
    --amount 10 \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Withdraw

```
spark-cli core withdraw \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

```
spark-cli core withdraw \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Withdraw To Market

```
spark-cli core withdraw-to-market \
    --asset-type quote \
    --amount 7000 \
    --market-id 0xc18094a283193c9b4726d2f644ed07ec9806bbe60a0688d45bffb26c379c1428 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Open Order

```
spark-cli core open \
    --amount 10 \
    --order-type buy \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

```
spark-cli core open \
    --amount 10 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Cancel Order

```
spark-cli core cancel \
    --order-id e950192bd177292dd7b98c69e6f85a46f5d59d93a0ba2f84af1f9d06d1fdf821 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Match Order Pair

```
spark-cli core match-pair \
    --orders f8051a6f690347e7446eb9a777e883b68a1f825b7a55d021a91412abacfca48a \
    --orders 3d66b9caf0628903e037eaa65318d926fb63bf34e7277d7413e349edddd5b0f0 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Match Order Many

```
spark-cli core match-many \
    --orders 0a96241df0a2606ead475af4cf66f89097bcbec27fdb59ff5cdb30a7525393e2 \
    --orders 2a6273b795e682f9fc4723097e682e0097c29c16f0419d7dc6132f77151e27ca \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Fulfill Order Many

```
spark-cli core fulfill-many \
    --amount 2 \
    --order-type sell \
    --limit-type ioc \
    --price 70000000000000 \
    --slippage 100 \
    --orders 0d0d7540d7350222b39a453452067cb3e1d1a29773a1678293771c9a0a12fe6f \
    --orders 12e7c70e34d437960fe455ce41ee9f839a93f5b317d19a9708c3ef51dffb89d0 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Set Protocol Fee

Sets protocol fee

```
spark-cli core set-protocol-fee \
    --fee "10,15,0" \
    --fee "8,13,10000000000" \
    --fee "6,11,50000000000" \
    --fee "4,9,100000000000" \
    --fee "2,7,500000000000" \
    --fee "1,5,1000000000000" \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Set Matcher Fee

Sets a matcher fee for the market

```
spark-cli core set-matcher-fee \
    --amount 1000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Set Epoch

Sets a epoch and duration for the market

```
spark-cli core set-epoch \
    --epoch 4611686020155120000 \
    --epoch-duration 2332800 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Set Store Order Change Info

Sets a store order change info for the market

```
spark-cli core set-store-order-change-info \
    --store \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```



# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

```
spark-cli info account \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Config

```
spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Epoch

```
spark-cli info epoch \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Protocol Fee

Protocol fee

```
spark-cli info protocol-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Protocol Fee User

Gets Protocol user fee

```
spark-cli info protocol-fee-user \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Protocol Fee User Amount

Calculates Protocol user fee for a order amount

```
spark-cli info protocol-fee-user-amount \
    --amount 10 \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Matcher Fee

Matcher Fee for the market

```
spark-cli info matcher-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Order ID

```
spark-cli info order-id \
    --order-type sell \
    --owner 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --account-type address \
    --price 70000000000000 \
    --block-height 10000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Order

```
spark-cli info order \
    --order-id 769663aef01812de5e5b4a4cd96f31a1641d4924cd26bdf7665fc00708487007 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Store Order Change Info

Store Order Change Info for the market

```
spark-cli info store-order-change-info \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## User Orders

```
spark-cli info user-orders \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0
```

## Deploy Market Registry

```
spark-cli registry deploy \
    --rpc "testnet.fuel.network" 
```

Output:
SparkRegistry deployed to: 0x194987ad2314d2de50646078ac1841f00b2dffda863a7d3dd421d220eb83d019
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Register a market

```
spark-cli registry register \
    --market 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x194987ad2314d2de50646078ac1841f00b2dffda863a7d3dd421d220eb83d019
```

## Unregister a market

```
spark-cli registry unregister \
    --market 0x7b88385ae73dd3ccc62012e7a52cddd05c7e82ad54a5df721dfa0c1f8b5998f0 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x194987ad2314d2de50646078ac1841f00b2dffda863a7d3dd421d220eb83d019
```

## Get registered markets by assets

```
spark-cli registry markets \
    --base 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc \
    --quote 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x194987ad2314d2de50646078ac1841f00b2dffda863a7d3dd421d220eb83d019
```

## Config

```
spark-cli registry config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x194987ad2314d2de50646078ac1841f00b2dffda863a7d3dd421d220eb83d019
```
