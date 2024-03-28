library;

pub enum Error {
    FirstArgumentShouldBeOrderSellSecondOrderBuy: (),
}

pub enum AssetError {
    InvalidAssetAmount: (),
    InvalidAsset: (),
}

pub enum MarketError {
    DuplicateMarket: (),
    NoMarketFound: (),
}

pub enum OrderError {
    AssetMismatch: (),
    NoOrdersFound: (),
    AccessDenied: (),
    InsufficientBuyPrice: (),
    DuplicateOrder: (),
    PriceCannotBeZero: (),
    SizeCannotBeZero: (),
}

pub enum AuthError {
    EOAOnly: (),
}
