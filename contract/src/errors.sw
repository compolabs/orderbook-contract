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

pub enum PriceError {
    PriceCannotBeZero: (),
}

pub enum OrderError {
    NoOrdersFound: (),
    AccessDenied: (),
    OrdersCantBeMatched: (),
    DuplicateOrder: (),
}

pub enum AuthError {
    EOAOnly: (),
}
