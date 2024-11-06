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
    --contract-id 0xfe2c524ad8e088f33d232a45dbea43e792861640b71aa1814b30506bf8430ee5
```

## Paused

```
spark-cli info paused \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x1b3aec515957737fe9bc12aab47e55aedfc9f182369b5cb79732872f9ae78889
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
