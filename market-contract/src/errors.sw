library;

pub enum AssetError {
    InvalidAsset: (),
    InvalidFeeAsset: (),
}

pub enum ValueError {
    InvalidAmount: (),
    InvalidSlippage: (),
    InvalidArrayLength: (),
    InvalidFeeAmount: (u64, u64),
    InvalidEpoch: (u64, u64, u64, u64),
    InvalidFeeSorting: (),
    InvalidFeeZeroBased: (),
}

pub enum OrderError {
    OrderDuplicate: b256,
    OrderNotFound: b256,
    PriceTooSmall: (u64, u64),
    ZeroOrderAmount: (),
    ZeroLockAmount: (),
    ZeroUnlockAmount: (),
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
