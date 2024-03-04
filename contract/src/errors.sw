library;

pub enum Error {
    AccessDenied: (),
    FreeCollateralMoreThanZero: (),
    NoOrdersFound: (),
    NoMarketFound: (),
    OrdersCantBeMatched: (),
    NoMarketPriceForMarket: (),
    FirstArgumentShouldBeOrderSellSecondOrderBuy: (),
    ZeroAssetAmountToSend: (),
    MarketAlreadyExists: (),
    BadAsset: (),
    BadValue: (),
    BadPrice: (),
}
