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

# Markets

ETH-USDC 0xfe2c524ad8e088f33d232a45dbea43e792861640b71aa1814b30506bf8430ee5
USDC-USDT 0xdafe498b31f24ea5577055e86bf77e96bcba2c39a7ae47abaa819c303a45a352
FUEL-USDC 0x81e83f73530c262b0dbf5414649a875c48a48144de3c08ff68cb9d54b36f2eaa
ezETH-USDC 0xe4f64c6a9facdce0c055ecade9379c8f425411ec3f9523a472d14ce8a4fbce38
FUEL-ETH 0x4391b39d9165917faffb9dcc69d19b6952a6ebf02db593747cf2f5d8298d28c7
pzETH-USDC 0x12f52412e0ef50d4e38e1d03fd80d0a88fbaa7253e47f0cc48ba4e3049bd9ce4
TRUMP-ETH 0x272bc2c2d065e8ca22f0473e328f403bb1ba2e85d71f5fa51dcb83393714ff01
USDT-USDC 0xe4e4844f78e2e470b590d0c76ffc9f4422a87317377813a181a02c60a60bc774
WETH-USDC 0x0bef6eb3018d901818978175feccf650b65dee8e3a8f5b59e138bcf1cf1d0db9
PSYCHO-USDC 0x2eece85eb7c8ec5fd95e639fd6bb7e9dd7103a99d7321521848da246ecef5270
USDF-USDC 0x59020aadb448c59b48136a3cef110f1ddd2865000146514924f19b83f061ceba
USDT-ETH 0x979ea6b1e15c1ec8e79eb76b587af89dd2620b383082e9b2c16049b78e97e4e8
TETH-TUSDC 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a

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


Spark CLI v0.7.0

Market version 0.7.0 (1792) deployed to: 0x544ae99c3beb0a63599334fe2c7e49bfa43c69ceb716ca9913e60513a71a1c97
Deployment cost: 6216
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

### Upgrade ETH-USDC market with a new implementation

```
./target/release/spark-cli upgrade upgrade-eth-usdc-proxy

```

Spark CLI v0.7.1

Proxy target: Some(580cec81fe8086336a5e01405233b5a1324bcfd55da3b151aa6bee9bd9992f44)
Base Asset: 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.5

New target deployed: Bech32ContractId { hrp: "fuel", hash: 79cb71872715125548bc8474f16241590a7a97b53fd27491b0e06824620057d3 }

New proxy target: Some(79cb71872715125548bc8474f16241590a7a97b53fd27491b0e06824620057d3)
Base Asset: 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0xfe2c524ad8e088f33d232a45dbea43e792861640b71aa1814b30506bf8430ee5 upgraded to version 0.7.1 (1793) with target Some(79cb71872715125548bc8474f16241590a7a97b53fd27491b0e06824620057d3)

Deployment cost: 6343
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

### Upgrade USDC-USDT market with a new implementation

```
./target/release/spark-cli upgrade upgrade-usdc-usdt-proxy

```

Spark CLI v0.7.1

Proxy target: Some(9db501e5627c8ac90fc8ec8d44f884264180028453642dde3f743b00d5edc39e)
Base Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Base Asset Decimals: 6
Quote Asset: 0xa0265fb5c32f6e8db3197af3c7eb05c48ae373605b8165b6f4a51c5b0ba4812e
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: aa208ec71203f84dc431848932a8ee4c8cf9826399ad6386450f6f1a9c3926e2 }

New proxy target: Some(aa208ec71203f84dc431848932a8ee4c8cf9826399ad6386450f6f1a9c3926e2)
Base Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Base Asset Decimals: 6
Quote Asset: 0xa0265fb5c32f6e8db3197af3c7eb05c48ae373605b8165b6f4a51c5b0ba4812e
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0xdafe498b31f24ea5577055e86bf77e96bcba2c39a7ae47abaa819c303a45a352 upgraded to version 0.7.1 (1793) with target Some(aa208ec71203f84dc431848932a8ee4c8cf9826399ad6386450f6f1a9c3926e2)

Deployment cost: 6343
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


### Upgrade FUEL-USDC market with a new implementation

```
./target/release/spark-cli upgrade upgrade-fuel-usdc-proxy

```

Spark CLI v0.7.1

Proxy target: Some(cf1290fc83812c53a8043bb291b543937e42a9ac24397ce3b2ba36de540b9bc6)
Base Asset: 0x1d5d97005e41cae2187a895fd8eab0506111e0e2f3331cd3912c15c24e3c1d82
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.11

New target deployed: Bech32ContractId { hrp: "fuel", hash: 7973b86d63a07bc1adddafc5ce1e82817daac4bd66d1589432b4efb180b8588c }

New proxy target: Some(7973b86d63a07bc1adddafc5ce1e82817daac4bd66d1589432b4efb180b8588c)
Base Asset: 0x1d5d97005e41cae2187a895fd8eab0506111e0e2f3331cd3912c15c24e3c1d82
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0x81e83f73530c262b0dbf5414649a875c48a48144de3c08ff68cb9d54b36f2eaa upgraded to version 0.7.1 (1793) with target Some(7973b86d63a07bc1adddafc5ce1e82817daac4bd66d1589432b4efb180b8588c)

Deployment cost: 6343
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

### Upgrade ezETH-USDC market with a new implementation

```
./target/release/spark-cli upgrade upgrade-ezeth-usdc-proxy

```

Spark CLI v0.7.1

Proxy target: Some(581bbe2ff52afa6164b2a532066462d9a4fbaccec771d91182e2e7f1aea33b19)
Base Asset: 0x91b3559edb2619cde8ffb2aa7b3c3be97efd794ea46700db7092abeee62281b0
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: e04df1e7589a71196a47ed2c0bfbb6303fa3eda4478b06c3e7639b3648ecac20 }

New proxy target: Some(e04df1e7589a71196a47ed2c0bfbb6303fa3eda4478b06c3e7639b3648ecac20)
Base Asset: 0x91b3559edb2619cde8ffb2aa7b3c3be97efd794ea46700db7092abeee62281b0
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0xe4f64c6a9facdce0c055ecade9379c8f425411ec3f9523a472d14ce8a4fbce38 upgraded to version 0.7.1 (1793) with target Some(e04df1e7589a71196a47ed2c0bfbb6303fa3eda4478b06c3e7639b3648ecac20)

Deployment cost: 6343
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

### Upgrade FUEL-ETH market with a new implementation

```
./target/release/spark-cli upgrade upgrade-fuel-eth-proxy

```

Spark CLI v0.7.1

Proxy target: Some(710b8bc874982703de5c2b1fa40e6fb250ca1417c9f7c20c2b9126a9e51802e6)
Base Asset: 0x1d5d97005e41cae2187a895fd8eab0506111e0e2f3331cd3912c15c24e3c1d82
Base Asset Decimals: 9
Quote Asset: 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07
Quote Asset Decimals: 9
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: ab547ade4dd54ad1b78b80c26e94af2192f1224f4b27795055f326353bac7042 }

New proxy target: Some(ab547ade4dd54ad1b78b80c26e94af2192f1224f4b27795055f326353bac7042)
Base Asset: 0x1d5d97005e41cae2187a895fd8eab0506111e0e2f3331cd3912c15c24e3c1d82
Base Asset Decimals: 9
Quote Asset: 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07
Quote Asset Decimals: 9
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0x4391b39d9165917faffb9dcc69d19b6952a6ebf02db593747cf2f5d8298d28c7 upgraded to version 0.7.1 (1793) with target Some(ab547ade4dd54ad1b78b80c26e94af2192f1224f4b27795055f326353bac7042)

Deployment cost: 6343
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

### Upgrade pzETH-USDC market with a new implementation

```
./target/release/spark-cli upgrade upgrade-pzeth-usdc-proxy

```

Spark CLI v0.7.1

Proxy target: Some(23c6760a788369b5bb0e8e62b357dbfce7c0f5873df800be5cb784a4eab0a5c2)
Base Asset: 0x1493d4ec82124de8f9b625682de69dcccda79e882b89a55a8c737b12de67bd68
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: 37976c818ea22a417f252a83d86bd6108ab962bcd15bff5bdc03206fade5adf3 }

New proxy target: Some(37976c818ea22a417f252a83d86bd6108ab962bcd15bff5bdc03206fade5adf3)
Base Asset: 0x1493d4ec82124de8f9b625682de69dcccda79e882b89a55a8c737b12de67bd68
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0x12f52412e0ef50d4e38e1d03fd80d0a88fbaa7253e47f0cc48ba4e3049bd9ce4 upgraded to version 0.7.1 (1793) with target Some(37976c818ea22a417f252a83d86bd6108ab962bcd15bff5bdc03206fade5adf3)

Deployment cost: 6343
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

### Upgrade USDT-USDC market with a new implementation

```
./target/release/spark-cli upgrade upgrade-usdt-usdc-proxy

```

Spark CLI v0.7.1

Proxy target: Some(d509b80e7fecc8891d21cf49f0dc6e2aeb676af88c5d5f04189fe2b4da62975a)
Base Asset: 0xa0265fb5c32f6e8db3197af3c7eb05c48ae373605b8165b6f4a51c5b0ba4812e
Base Asset Decimals: 6
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: 3036ddda246b47b19227617b1eeccba1fd8e579b93452609bb61ded529badbd8 }

New proxy target: Some(3036ddda246b47b19227617b1eeccba1fd8e579b93452609bb61ded529badbd8)
Base Asset: 0xa0265fb5c32f6e8db3197af3c7eb05c48ae373605b8165b6f4a51c5b0ba4812e
Base Asset Decimals: 6
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0xe4e4844f78e2e470b590d0c76ffc9f4422a87317377813a181a02c60a60bc774 upgraded to version 0.7.1 (1793) with target Some(3036ddda246b47b19227617b1eeccba1fd8e579b93452609bb61ded529badbd8)

Deployment cost: 6343
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

### Upgrade WETH-USDC market with a new implementation

```
./target/release/spark-cli upgrade upgrade-weth-usdc-proxy

```

Spark CLI v0.7.1

Proxy target: Some(72b6e18c2aee9c418209f2716bdcfb6c776a609dcfe7c73912dba9d85c90ddd1)
Base Asset: 0xa38a5a8beeb08d95744bc7f58528073f4052b254def59eba20c99c202b5acaa3
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: a6fbb341ce173bf1fb2b8ea3ff4b072dd00d816e9ffdc859c0505c90298612e1 }

New proxy target: Some(a6fbb341ce173bf1fb2b8ea3ff4b072dd00d816e9ffdc859c0505c90298612e1)
Base Asset: 0xa38a5a8beeb08d95744bc7f58528073f4052b254def59eba20c99c202b5acaa3
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0x0bef6eb3018d901818978175feccf650b65dee8e3a8f5b59e138bcf1cf1d0db9 upgraded to version 0.7.1 (1793) with target Some(a6fbb341ce173bf1fb2b8ea3ff4b072dd00d816e9ffdc859c0505c90298612e1)

Deployment cost: 6343
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

### Upgrade PSYCHO-USDC market with a new implementation

```
./target/release/spark-cli upgrade upgrade-psycho-usdc-proxy

```

Spark CLI v0.7.1

Proxy target: Some(b217e7d6d722948e77909e675a9a2932caaa4b73635842d09c00060cc02344ba)
Base Asset: 0x86fa05e9fef64f76fa61c03f5906c87a03cb9148120b6171910566173d36fc9e
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: ecc352b9939b132228f21b4b495dd45f9989e709c20e96d1498b80ae4f3ee6b7 }

New proxy target: Some(ecc352b9939b132228f21b4b495dd45f9989e709c20e96d1498b80ae4f3ee6b7)
Base Asset: 0x86fa05e9fef64f76fa61c03f5906c87a03cb9148120b6171910566173d36fc9e
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0x2eece85eb7c8ec5fd95e639fd6bb7e9dd7103a99d7321521848da246ecef5270 upgraded to version 0.7.1 (1793) with target Some(ecc352b9939b132228f21b4b495dd45f9989e709c20e96d1498b80ae4f3ee6b7)

Deployment cost: 6343
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7

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

### Upgrade USDF-USDC market with a new implementation

```
./target/release/spark-cli upgrade upgrade-usdf-usdc-proxy

```

Spark CLI v0.7.1

Proxy target: Some(b8d5e5871bbd4d4712b687e83f44f2fc132c1a4a1f135f79837e78adadb83ae3)
Base Asset: 0x33a6d90877f12c7954cca6d65587c25e9214c7bed2231c188981c7114c1bdb78
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: cc3fbfea841f852f977a2f5fdd9b3e9fd9c087f518031a0b48f59ab8c579491e }

New proxy target: Some(cc3fbfea841f852f977a2f5fdd9b3e9fd9c087f518031a0b48f59ab8c579491e)
Base Asset: 0x33a6d90877f12c7954cca6d65587c25e9214c7bed2231c188981c7114c1bdb78
Base Asset Decimals: 9
Quote Asset: 0x286c479da40dc953bddc3bb4c453b608bba2e0ac483b077bd475174115395e6b
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0x59020aadb448c59b48136a3cef110f1ddd2865000146514924f19b83f061ceba upgraded to version 0.7.1 (1793) with target Some(cc3fbfea841f852f977a2f5fdd9b3e9fd9c087f518031a0b48f59ab8c579491e)

Deployment cost: 6343
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7

### Deploy USDT-ETH market proxy + implementation

```
spark-cli batch deploy-usdt-eth-proxy

```

Spark CLI v0.6.10

Market version 0.6.6 (1542) deployed to: 0x328ab520aa6a07fa288261d37c8186eba9f4dc1492f62f4eb56702dd4817a188
               Proxy deployed to: 0x979ea6b1e15c1ec8e79eb76b587af89dd2620b383082e9b2c16049b78e97e4e8
Deployment cost: 77931
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7
Block height: 12997252

### Upgrade USDT-ETH market with a new implementation

```
./target/release/spark-cli upgrade upgrade-usdt-eth-proxy

```

Spark CLI v0.7.1

Proxy target: Some(328ab520aa6a07fa288261d37c8186eba9f4dc1492f62f4eb56702dd4817a188)
Base Asset: 0xa0265fb5c32f6e8db3197af3c7eb05c48ae373605b8165b6f4a51c5b0ba4812e
Base Asset Decimals: 6
Quote Asset: 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07
Quote Asset Decimals: 9
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.6.6

New target deployed: Bech32ContractId { hrp: "fuel", hash: d9b76d97ba5f6dfd5ed5d7c110de9685480801a4030a0066264c9b666699dffc }

New proxy target: Some(d9b76d97ba5f6dfd5ed5d7c110de9685480801a4030a0066264c9b666699dffc)
Base Asset: 0xa0265fb5c32f6e8db3197af3c7eb05c48ae373605b8165b6f4a51c5b0ba4812e
Base Asset Decimals: 6
Quote Asset: 0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07
Quote Asset Decimals: 9
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0x979ea6b1e15c1ec8e79eb76b587af89dd2620b383082e9b2c16049b78e97e4e8 upgraded to version 0.7.1 (1793) with target Some(d9b76d97ba5f6dfd5ed5d7c110de9685480801a4030a0066264c9b666699dffc)

Deployment cost: 6343
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7

### Deploy TETH-TUSDC market proxy + implementation

```
spark-cli batch deploy-teth-tusdc-proxy

```

Spark CLI v0.7.0

Market version 0.7.0 (1792) deployed to: 0xb6b32862fae03d7dc25b5d916912df5293eb017acc1ba4fa31fda26323fe892e
               Proxy deployed to: 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
Deployment cost: 7321
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7

----------------------
Spark CLI v0.7.1

Proxy target: Some(b6b32862fae03d7dc25b5d916912df5293eb017acc1ba4fa31fda26323fe892e)
Base Asset: 0xf169e13e98ae8908199148380684894458b7916f074b85ebad2aaad489ce0d54
Base Asset Decimals: 9
Quote Asset: 0x22dfb618b9fc621a7d53f0f599dd427fb5688e280062a8de8883a27819d3f276
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.0

New target deployed: Bech32ContractId { hrp: "fuel", hash: d080a3ac06876d8fe70408b1d163f05a021518eab9ad45c21e37723a462d09f4 }

New proxy target: Some(d080a3ac06876d8fe70408b1d163f05a021518eab9ad45c21e37723a462d09f4)
Base Asset: 0xf169e13e98ae8908199148380684894458b7916f074b85ebad2aaad489ce0d54
Base Asset Decimals: 9
Quote Asset: 0x22dfb618b9fc621a7d53f0f599dd427fb5688e280062a8de8883a27819d3f276
Quote Asset Decimals: 6
Owner: Some(Address(1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7))
Price Decimals: 9
Version: 0.7.1

Market 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a upgraded to version 0.7.1 (1793) with target Some(d080a3ac06876d8fe70408b1d163f05a021518eab9ad45c21e37723a462d09f4)

Deployment cost: 6343
Owner address: fuel1rmu7c4gjycy4qtvj8798sv04ptq9uq4a6eq9y23w7x8apundtlrs0u000t
               0x1ef9ec55122609502d923f8a7831f50ac05e02bdd640522a2ef18fd0f26d5fc7

## Deposit

```
spark-cli core deposit \
    --asset-type base \
    --amount 1000000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
```

```
spark-cli core deposit \
    --asset-type quote \
    --amount 1000000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
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
    --amount 100000 \
    --order-type buy \
    --price 3000000000000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
```

```
spark-cli core open \
    --amount 10 \
    --order-type sell \
    --price 70000000000000 \
    --rpc "testnet.fuel.network" \
    --contract-id 0x81acb82a64ff799836c19f4e7f9871cf6d13a1e5d286e815f91c26a1b92a8195
```

## Open Market Order

```
spark-cli core open-market \
    --amount 900000000 \
    --order-type buy \
    --price 500000000000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
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
    --fee "0,4,0" \
    --fee "0,3,10000000000" \
    --fee "0,1,50000000000" \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xfe2c524ad8e088f33d232a45dbea43e792861640b71aa1814b30506bf8430ee5
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

## Set Minimum Order Size

Sets a minimum order size for the market

```
spark-cli core set-min-order-size \
    --size 100000 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
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
    --contract-id 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
```

## Config

```
spark-cli info config \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x544ae99c3beb0a63599334fe2c7e49bfa43c69ceb716ca9913e60513a71a1c97
```

## Paused

```
spark-cli info paused \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
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
    --contract-id 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a
```

## Proxy Owner

Proxy owner market

```
spark-cli info proxy-owner \
    --rpc "mainnet.fuel.network" \
    --contract-id 0x544ae99c3beb0a63599334fe2c7e49bfa43c69ceb716ca9913e60513a71a1c97
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

## Market Order

```
spark-cli info market-order \
    --order-id 769663aef01812de5e5b4a4cd96f31a1641d4924cd26bdf7665fc00708487007 \
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
    --market 0x6eb7a35c43a8eae0a2aeaf8c68b7d2d1cc7d2481d97abb8f68e4fb3cbab86a2a \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xbb91b7f9d31ee562b24e35d756ce20913f9752600582f51008c63b2d3792926b
```

## Unregister a market

```
spark-cli registry unregister \
    --market 0x544ae99c3beb0a63599334fe2c7e49bfa43c69ceb716ca9913e60513a71a1c97 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xbb91b7f9d31ee562b24e35d756ce20913f9752600582f51008c63b2d3792926b
```

## Get registered markets by assets

```
spark-cli registry markets \
    --base 0xf169e13e98ae8908199148380684894458b7916f074b85ebad2aaad489ce0d54 \
    --quote 0x22dfb618b9fc621a7d53f0f599dd427fb5688e280062a8de8883a27819d3f276 \
    --rpc "mainnet.fuel.network" \
    --contract-id 0xbb91b7f9d31ee562b24e35d756ce20913f9752600582f51008c63b2d3792926b
```

## Config

```
spark-cli registry config \
    --rpc "testnet.fuel.network" \
    --contract-id 0xd76662328e464549b6f619401992127bed9b5cff3b46a3516e6b509d810b7035
```
