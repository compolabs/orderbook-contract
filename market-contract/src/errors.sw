library;

pub enum AssetError {
    InvalidAsset: (),
}

pub enum ValueError {
    InvalidAmount: (),
    InvalidLength: (),
}

pub enum OrderError {
    OrderNotFound: b256,
    PriceTooSmall: (u64, u64),
    AmountCannotBeZero: (),
    FailedToRemove: b256,
}

pub enum MatchError {
    CantMatch: (b256, b256),
    CantBatchMatch: (),
}

pub enum AuthError {
    Unauthorized: (),
}

pub enum AccountError {
    InsufficientBalance: (u64, u64),
}
