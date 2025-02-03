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
TBTC address 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc
TUSDC address 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05

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


### Deploy tETH-tUSDC market implementation

```
spark-cli batch deploy-teth-tusdc-impl --rpc "mainnet.fuel.network" 

```

Spark CLI v0.6.5

Market version 0.6.5 (1541) deployed to: 0x131ac8a64da9504a48ba4cfc452df6689404d676701d4b47c146fed11c598da2
Deployment cost: 6395
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy ETH-USDC market proxy + implementation

```
spark-cli batch deploy-eth-usdc-proxy

```

Spark CLI v0.6.5

Market version 0.6.5 (1541) deployed to: 0x580cec81fe8086336a5e01405233b5a1324bcfd55da3b151aa6bee9bd9992f44
               Proxy deployed to: 0xfe2c524ad8e088f33d232a45dbea43e792861640b71aa1814b30506bf8430ee5
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7

### Deploy USDC-USDT market proxy + implementation

```
spark-cli batch deploy-usdc-usdt-proxy

```

Spark CLI v0.6.7

Market version 0.6.6 (1542) deployed to: 0x9db501e5627c8ac90fc8ec8d44f884264180028453642dde3f743b00d5edc39e
               Proxy deployed to: 0xdafe498b31f24ea5577055e86bf77e96bcba2c39a7ae47abaa819c303a45a352
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy FUEL-USDC market proxy + implementation

```
spark-cli batch deploy-fuel-usdc-proxy

```

Spark CLI v0.6.8

Market version 0.6.6 (1542) deployed to: 0xee459fbb07d7afa5feeac4ecd54cb83bf0c9d7d0ec292a34dbab2ec558728f7f
               Proxy deployed to: 0x81e83f73530c262b0dbf5414649a875c48a48144de3c08ff68cb9d54b36f2eaa
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy ezETH-USDC market proxy + implementation

```
spark-cli batch deploy-ezeth-usdc-proxy

```

Spark CLI v0.6.9

Market version 0.6.6 (1542) deployed to: 0x581bbe2ff52afa6164b2a532066462d9a4fbaccec771d91182e2e7f1aea33b19
               Proxy deployed to: 0xe4f64c6a9facdce0c055ecade9379c8f425411ec3f9523a472d14ce8a4fbce38
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy FUEL-ETH market proxy + implementation

```
spark-cli batch deploy-fuel-eth-proxy

```

Spark CLI v0.6.9

Market version 0.6.6 (1542) deployed to: 0x710b8bc874982703de5c2b1fa40e6fb250ca1417c9f7c20c2b9126a9e51802e6
               Proxy deployed to: 0x4391b39d9165917faffb9dcc69d19b6952a6ebf02db593747cf2f5d8298d28c7
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy pzETH-USDC market proxy + implementation

```
spark-cli batch deploy-pzeth-usdc-proxy

```

Spark CLI v0.6.9

Market version 0.6.6 (1542) deployed to: 0x23c6760a788369b5bb0e8e62b357dbfce7c0f5873df800be5cb784a4eab0a5c2
               Proxy deployed to: 0x12f52412e0ef50d4e38e1d03fd80d0a88fbaa7253e47f0cc48ba4e3049bd9ce4
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy TRUMP-ETH market proxy + implementation

```
spark-cli batch deploy-trump-eth-proxy

```

Spark CLI v0.6.9

Market version 0.6.6 (1542) deployed to: 0x9fa52a1d8a2f7af1fbbd8c5a571ac5163e97da5dc0a0f722cb723cb4e1dc4d02
               Proxy deployed to: 0x272bc2c2d065e8ca22f0473e328f403bb1ba2e85d71f5fa51dcb83393714ff01
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy USDT-USDC market proxy + implementation

```
spark-cli batch deploy-usdt-usdc-proxy

```

Spark CLI v0.6.9

Market version 0.6.6 (1542) deployed to: 0xd509b80e7fecc8891d21cf49f0dc6e2aeb676af88c5d5f04189fe2b4da62975a
               Proxy deployed to: 0xe4e4844f78e2e470b590d0c76ffc9f4422a87317377813a181a02c60a60bc774
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy WETH-USDC market proxy + implementation

```
spark-cli batch deploy-weth-usdc-proxy

```

Spark CLI v0.6.9

Market version 0.6.6 (1542) deployed to: 0x72b6e18c2aee9c418209f2716bdcfb6c776a609dcfe7c73912dba9d85c90ddd1
               Proxy deployed to: 0x0bef6eb3018d901818978175feccf650b65dee8e3a8f5b59e138bcf1cf1d0db9
Deployment cost: 7798
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7


### Deploy PSYCHO-USDC market proxy + implementation

```
spark-cli batch deploy-psycho-usdc-proxy

```

Spark CLI v0.6.10

Market version 0.6.6 (1542) deployed to: 0xb217e7d6d722948e77909e675a9a2932caaa4b73635842d09c00060cc02344ba
               Proxy deployed to: 0x2eece85eb7c8ec5fd95e639fd6bb7e9dd7103a99d7321521848da246ecef5270
Deployment cost: 77931
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7
Block height: 12996694


### Deploy USDF-USDC market proxy + implementation

```
spark-cli batch deploy-usdf-usdc-proxy

```

Spark CLI v0.6.10

Market version 0.6.6 (1542) deployed to: 0xb8d5e5871bbd4d4712b687e83f44f2fc132c1a4a1f135f79837e78adadb83ae3
               Proxy deployed to: 0x59020aadb448c59b48136a3cef110f1ddd2865000146514924f19b83f061ceba
Deployment cost: 77931
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7
Block height: 12997072


### Deploy USDT-ETH market proxy + implementation

```
spark-cli batch deploy-usdf-usdc-proxy

```

Spark CLI v0.6.10

Market version 0.6.6 (1542) deployed to: 0x328ab520aa6a07fa288261d37c8186eba9f4dc1492f62f4eb56702dd4817a188
               Proxy deployed to: 0x979ea6b1e15c1ec8e79eb76b587af89dd2620b383082e9b2c16049b78e97e4e8
Deployment cost: 77931
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7
Block height: 12997252


## Deposit

```
spark-cli core deposit \
    --asset-type base \
    --amount 1000000000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x1b3aec515957737fe9bc12aab47e55aedfc9f182369b5cb79732872f9ae78889
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
    --amount 900000000 \
    --order-type buy \
    --price 500000000000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xb4d0cb6591fd480404bc389b90f86c05afdaf29bad9378faf45a797b9bfd847b
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
    --orders b5f097d38312119e246634658fb4bfc1179da51f257dd0f227d4ac5d5d02d25e \
    --orders 11a486fff78cfe58b983806d04c0399a7444f00de75a6abfc6a36e46c90243d9 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xb4d0cb6591fd480404bc389b90f86c05afdaf29bad9378faf45a797b9bfd847b
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

## Set Paused

```
spark-cli core set-paused \
    --paused \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xee459fbb07d7afa5feeac4ecd54cb83bf0c9d7d0ec292a34dbab2ec558728f7f
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


## Set Proxy Target

Sets a proxy target market

```
spark-cli core set-proxy-target \
    --target 0x9e7e4d65d9bda041dde75f09d81400bc4c0ce52fabb9c2419ef64710d0413f22 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x4c9010a055ab636c38caa0e4c7cf9eb4ad8d6f44ff6e094f23b3dcdd291ee093
```



# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

```
spark-cli info account \
    --account-type address \
    --account-id 0x1Ef9Ec55122609502D923F8A7831f50ac05E02bdD640522A2EF18Fd0F26d5Fc7 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x4c9010a055ab636c38caa0e4c7cf9eb4ad8d6f44ff6e094f23b3dcdd291ee093
```

## Config

```
spark-cli info config \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x81e83f73530c262b0dbf5414649a875c48a48144de3c08ff68cb9d54b36f2eaa
```

## Paused

```
spark-cli info paused \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xee459fbb07d7afa5feeac4ecd54cb83bf0c9d7d0ec292a34dbab2ec558728f7f
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
    --rpc "mainnet.fuel.network" \
    --contract-id 0x81e83f73530c262b0dbf5414649a875c48a48144de3c08ff68cb9d54b36f2eaa
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
    --rpc "mainnet.fuel.network" \
    --contract-id 0x4c9010a055ab636c38caa0e4c7cf9eb4ad8d6f44ff6e094f23b3dcdd291ee093
```

## Proxy Target

Proxy target market

```
spark-cli info proxy-target \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xfe2c524ad8e088f33d232a45dbea43e792861640b71aa1814b30506bf8430ee5
```

## Proxy Owner

Proxy owner market

```
spark-cli info proxy-owner \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xfe2c524ad8e088f33d232a45dbea43e792861640b71aa1814b30506bf8430ee5
```

## Minimum Order Size

Minimum Order Size for the market

```
spark-cli info min-order-size \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x9f7c554b235320a1001621010069b0323661068dd6d02f0f766838e3001c3b31
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
    --market 0x81e83f73530c262b0dbf5414649a875c48a48144de3c08ff68cb9d54b36f2eaa \
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
