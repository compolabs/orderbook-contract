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
Spark CLI v0.6.3

BTC/USDC Market version 0.6.3 (1539) deployed to: 0xc5ed0d9b17beedd1c6c10a84bb496f12a5082aa3ce2ad55630bbcac22c64fcf4
Deployment cost: 6729

ETH/USDC Market version 0.6.3 (1539) deployed to: 0x944a3d62e65f3aefa7ac4a065eb9390a98806ef254aaece6df239ee78e6c2998
Deployment cost: 6729

MarketRegistry version 0.6.3 (1539) deployed to: 0x0c26b7134516773469cd02030a783e43776d1fd26e0698b51af3cef4938e2925
Deployment cost: 8433

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
Market deployed to: 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
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
SparkMarket deployed to: 0x3830aa30ddd4843dd13b6af7ae4fb59d8c5933b1a98cba9a80897c8ba5557307
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

### Deploy Proxy market

```
spark-cli batch deploy-proxy \
    --base-asset 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc \
    --base-decimals 8 \
    --quote-asset 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --quote-decimals 6 \
    --price-decimals 9 \
    --rpc "testnet.fuel.network"
```

Sample output:
Spark CLI v0.6.3

Market version 0.6.3 (1539) deployed to: 0x9e71d92577e2771ebe526fc683b69576f5c4622e60663300fc79f028cf035eda
               Proxy deployed to: 0xdeca3101bd9c7e6c053b49597244afd3e51c9476e457c51d894fd390b3b1746b
Deployment cost: 7308
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7

### Deploy TRMP-KMLA market

```
spark-cli core deploy \
    --base-asset 0x0b2d808a898cdae8b8661d398a98f8ff45e1e0f536ba2e498f6c7e53a71932cd \
    --base-decimals 9 \
    --quote-asset 0x368f9275e7d072794527b57d5b54688300008a400f41d926a013195e7074029c \
    --quote-decimals 9 \
    --price-decimals 9 \
    --rpc "mainnet.fuel.network"
```

Sample output:
Spark CLI v0.6.3

Market version 0.6.3 (1539) deployed to: 0x12a5f8666279f841e5900500297ce3c8bcf40103dd191c56dd3ec86f92b9217b
Deployment cost: 6197
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7
## Deposit

```
spark-cli core deposit \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

```
spark-cli core deposit \
    --asset-type quote \
    --amount 10000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Deposit For

```
spark-cli core deposit-for \
    --asset-type base \
    --amount 10 \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Withdraw

```
spark-cli core withdraw \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

```
spark-cli core withdraw \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Withdraw To Market

```
spark-cli core withdraw-to-market \
    --asset-type quote \
    --amount 7000 \
    --market-id 0x3830aa30ddd4843dd13b6af7ae4fb59d8c5933b1a98cba9a80897c8ba5557307 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Open Order

```
spark-cli core open \
    --amount 20000 \
    --order-type buy \
    --price 1000000000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x12a5f8666279f841e5900500297ce3c8bcf40103dd191c56dd3ec86f92b9217b
```

```
spark-cli core open \
    --amount 10 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Cancel Order

```
spark-cli core cancel \
    --order-id e950192bd177292dd7b98c69e6f85a46f5d59d93a0ba2f84af1f9d06d1fdf821 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Match Order Pair

```
spark-cli core match-pair \
    --orders b51e154e4b975fe86c126faa08ca5da1bfcb1b81535b6cca14433f7f255cfc88 \
    --orders 51e7fb9b58c88e0e5f70bf33b023dff09628d474caacbf6830bc81ee11939412 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x12a5f8666279f841e5900500297ce3c8bcf40103dd191c56dd3ec86f92b9217b
```

## Match Order Many

```
spark-cli core match-many \
    --orders 0a96241df0a2606ead475af4cf66f89097bcbec27fdb59ff5cdb30a7525393e2 \
    --orders 2a6273b795e682f9fc4723097e682e0097c29c16f0419d7dc6132f77151e27ca \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
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
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
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
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Set Matcher Fee

Sets a matcher fee for the market

```
spark-cli core set-matcher-fee \
    --amount 1000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Set Epoch

Sets a epoch and duration for the market

```
spark-cli core set-epoch \
    --epoch 4611686020155120000 \
    --epoch-duration 5020000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x12a5f8666279f841e5900500297ce3c8bcf40103dd191c56dd3ec86f92b9217b
```

## Set Store Order Change Info

Sets a store order change info for the market

```
spark-cli core set-store-order-change-info \
    --store \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Set Minimum Order Size

Sets a minimum order size for the market

```
spark-cli core set-min-order-size \
    --size 1000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x12a5f8666279f841e5900500297ce3c8bcf40103dd191c56dd3ec86f92b9217b
```


## Set Minimum Order Price

Sets a minimum order size for the market

```
spark-cli core set-min-order-price \
    --price 1000000000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x12a5f8666279f841e5900500297ce3c8bcf40103dd191c56dd3ec86f92b9217b
```



# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

```
spark-cli info account \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Config

```
spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Epoch

```
spark-cli info epoch \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Protocol Fee

Protocol fee

```
spark-cli info protocol-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Protocol Fee User

Gets Protocol user fee

```
spark-cli info protocol-fee-user \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Protocol Fee User Amount

Calculates Protocol user fee for a order amount

```
spark-cli info protocol-fee-user-amount \
    --amount 10 \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Matcher Fee

Matcher Fee for the market

```
spark-cli info matcher-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Minimum Order Size

Minimum Order Size for the market

```
spark-cli info min-order-size \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Minimum Order Price

Minimum Order Price for the market

```
spark-cli info min-order-price \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x12a5f8666279f841e5900500297ce3c8bcf40103dd191c56dd3ec86f92b9217b
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
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Order

```
spark-cli info order \
    --order-id 769663aef01812de5e5b4a4cd96f31a1641d4924cd26bdf7665fc00708487007 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Store Order Change Info

Store Order Change Info for the market

```
spark-cli info store-order-change-info \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## User Orders

```
spark-cli info user-orders \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Deploy Market Registry

```
spark-cli registry deploy \
    --rpc "testnet.fuel.network" 
```

Output:
SparkRegistry deployed to: 0xd76662328e464549b6f619401992127bed9b5cff3b46a3516e6b509d810b7035
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Register a market

```
spark-cli registry register \
    --market 0x12a5f8666279f841e5900500297ce3c8bcf40103dd191c56dd3ec86f92b9217b \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xbb91b7f9d31ee562b24e35d756ce20913f9752600582f51008c63b2d3792926b
```

## Unregister a market

```
spark-cli registry unregister \
    --market 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xd76662328e464549b6f619401992127bed9b5cff3b46a3516e6b509d810b7035
```

## Get registered markets by assets

```
spark-cli registry markets \
    --base 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc \
    --quote 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --rpc "testnet.fuel.network" \
    --contract-id 0xd76662328e464549b6f619401992127bed9b5cff3b46a3516e6b509d810b7035
```

## Config

```
spark-cli registry config \
    --rpc "testnet.fuel.network" \
    --contract-id 0xd76662328e464549b6f619401992127bed9b5cff3b46a3516e6b509d810b7035
```
