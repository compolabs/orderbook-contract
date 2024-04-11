library;

pub enum Error {
    AccessDenied: (),
    NoOrdersFound: (),
    NoMarketFound: (),
    OrdersCantBeMatched: (),
    FirstArgumentShouldBeOrderSellSecondOrderBuy: (),
    ZeroAssetAmountToSend: (),
    MarketAlreadyExists: (),
    BadAsset: (),
    BadValue: (),
    BadPrice: (),
}
