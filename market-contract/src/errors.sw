library;

pub enum AssetError {
    InvalidAsset: (),
}

pub enum OrderError {
    NoOrdersFound: (),
    DuplicateOrder: (),
    PriceCannotBeZero: (),
    AmountCannotBeZero: (),
    FailedToRemove: (),
    LeftShouldBeSellOrder: (),
    RightShouldBeBuyOrder: (),
    AssetMismatch: (),
    InsufficientBuyPrice: (),
}

pub enum AuthError {
    Unauthorized: (),
}

pub enum AccountError {
    InsufficientBalance: (u64, u64),
    InvalidUser: (),
}
