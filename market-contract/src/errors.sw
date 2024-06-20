library;

pub enum AssetError {
    InvalidAsset: (),
}

pub enum ValueError {
    InvalidAmount: (),
    InvalidArrayLength: (),
}

pub enum OrderError {
    OrderNotFound: b256,
    PriceTooSmall: (u64, u64),
    ZeroOrderAmount: (),
    ZeroLockAmount: (),
    FailedToRemove: b256,
}

pub enum MatchError {
    CantMatch: (b256, b256),
    CantMatchMany: (),
}

pub enum AuthError {
    Unauthorized: (),
}

pub enum AccountError {
    InsufficientBalance: (u64, u64),
}
