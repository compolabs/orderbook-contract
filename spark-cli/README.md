Note: Work in progress, incomplete, use as guide only, only tested in testnet
Note: Fuels SDK version 0.56.0 does not currently work with testnet so the repo is using 0.65.0

Inside the spark project (CLI) run the following commands

# Core functions

These contract calls change the state of the market contract so they require the wallet to have enough funds to make a call

Run from project root folder

## Setup

```
cargo install spark-cli
```

Create `.env` file in the project root and initialize `WALLET_SECRET=` with Fuel private key value there

## Deploy

ETH address 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07
BTC address 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc
USDC address 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05

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
Market deployed to: 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
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
SparkMarket deployed to: 0x2e9f781674f292d4db1ad150e7685e1f1ebad3c1ba403a64fff54b019ed70765
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Deposit

```
spark-cli core deposit \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

```
spark-cli core deposit \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Withdraw

```
spark-cli core withdraw \
    --asset-type base \
    --amount 10 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

```
spark-cli core withdraw \
    --asset-type quote \
    --amount 7000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Open Order

```
spark-cli core open \
    --amount 10 \
    --order-type buy \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

```
spark-cli core open \
    --amount 10 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Cancel Order

```
spark-cli core cancel \
    --order-id e950192bd177292dd7b98c69e6f85a46f5d59d93a0ba2f84af1f9d06d1fdf821 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Match Order Pair

```
spark-cli core match-pair \
    --orders 75be33ea94dd4d9513fbd2fb8fcf83afde867336768220d50af178dc67dea3bc \
    --orders 26c3a311a96f647bc5e7fd77e4e201ec0b670bf680960dcebb63f21daea9bde9 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Match Order Many

```
spark-cli core match-many \
    --orders 0a96241df0a2606ead475af4cf66f89097bcbec27fdb59ff5cdb30a7525393e2 \
    --orders 2a6273b795e682f9fc4723097e682e0097c29c16f0419d7dc6132f77151e27ca \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
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
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
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
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Set Matcher Fee

Sets a matcher fee for the market

```
spark-cli core set-matcher-fee \
    --amount 1000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Set Epoch

Sets a epoch and duration for the market

```
spark-cli core set-epoch \
    --epoch 4611686020152825242 \
    --epoch-duration 2332800 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```



# Info

These functions return the state of the contract. They simulate calls and therefore are free to call.

## Account

```
spark-cli info account \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Config

```
spark-cli info config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Epoch

```
spark-cli info epoch \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Protocol Fee

Protocol fee

```
spark-cli info protocol-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Protocol Fee User

Gets Protocol user fee

```
spark-cli info protocol-fee-user \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Protocol Fee User Amount

Calculates Protocol user fee for a order amount

```
spark-cli info protocol-fee-user-amount \
    --amount 10 \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Matcher Fee

Matcher Fee for the market

```
spark-cli info matcher-fee \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
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
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Order

```
spark-cli info order \
    --order-id 769663aef01812de5e5b4a4cd96f31a1641d4924cd26bdf7665fc00708487007 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## User Orders

```
spark-cli info user-orders \
    --account-type address \
    --account-id 0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf \
    --rpc "testnet.fuel.network" \
    --contract-id 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5
```

## Deploy Orderbook

```
spark-cli book deploy \
    --rpc "testnet.fuel.network" 
```

Output:
SparkRegistry deployed to: 0x0911d52d95a71dd484690636fb81db8596f54ee18fe5eb7e33842025d1dd80de
Deployment cost: 0
Owner address: fuel173lqaa6y4jxfjd2suq730uwys3zfg4f6zt9vzx4cc45v3xvlmwlszdvdpz
               0xf47e0ef744ac8c993550e03d17f1c4844494553a12cac11ab8c568c8999fdbbf

## Register a market

```
spark-cli book register \
    --market 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0911d52d95a71dd484690636fb81db8596f54ee18fe5eb7e33842025d1dd80de
```

## Unregister a market

```
spark-cli book unregister \
    --market 0x19aace5264843a3cd3f7a67da3835e4aa9a2e3d935961bf54b35dd5a45c305a5 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0911d52d95a71dd484690636fb81db8596f54ee18fe5eb7e33842025d1dd80de
```

## Get registered markets by assets

```
spark-cli book markets \
    --base 0x38e4ca985b22625fff93205e997bfc5cc8453a953da638ad297ca60a9f2600bc \
    --quote 0x336b7c06352a4b736ff6f688ba6885788b3df16e136e95310ade51aa32dc6f05 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0911d52d95a71dd484690636fb81db8596f54ee18fe5eb7e33842025d1dd80de
```

## Config

```
spark-cli book config \
    --rpc "testnet.fuel.network" \
    --contract-id 0x0911d52d95a71dd484690636fb81db8596f54ee18fe5eb7e33842025d1dd80de
```
