library;

pub enum AssetError {
    InvalidAsset: (),
    InvalidFeeAsset: (),
}

pub enum ValueError {
    InvalidAmount: (),
    InvalidSlippage: (),
    InvalidArrayLength: (),
    InvalidFeeAmount: (u32, u32),
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
    CantFulfillMany: (),
}

pub enum AuthError {
    Unauthorized: (),
}

pub enum AccountError {
    InsufficientBalance: (u64, u64),
}
