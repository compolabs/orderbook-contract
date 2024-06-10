library;

pub enum AssetError {
    InvalidAsset: (),
}

pub enum ValueError {
    InvalidAmount: (),
}

pub enum OrderError {
    NoOrdersFound: (),
    DuplicateOrder: (),
    PriceCannotBeZero: (),
    AmountCannotBeZero: (),
    FailedToRemove: (),
}

pub enum TradeError {
    CannotTrade: (),
}

pub enum AuthError {
    Unauthorized: (),
}

pub enum AccountError {
    InsufficientBalance: (u64, u64),
}
