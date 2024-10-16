library;

pub enum AssetError {
    InvalidAsset: (),
    InvalidFeeAsset: (),
    InvalidMarketAsset: (),
}

pub enum ValueError {
    InvalidAmount: (),
    InvalidSlippage: (),
    InvalidArrayLength: (),
    InvalidFeeAmount: (u64, u64),
    InvalidEpoch: (u64, u64, u64, u64),
    InvalidFeeSorting: (),
    InvalidFeeZeroBased: (),
    InvalidValueSame: (),
    InvalidMarketSame: (),
}

pub enum OrderError {
    OrderDuplicate: b256,
    OrderNotFound: b256,
    PriceTooSmall: (u64, u64),
    OrderSizeTooSmall: u64,
    ZeroLockAmount: (),
    ZeroUnlockAmount: (),
    ZeroTransferAmount: (),
    FailedToRemove: b256,
}

pub enum MatchError {
    CantMatch: (b256, b256),
    CantMatchMany: (),
    CantFulfillMany: (),
    CantFulfillFOK: (),
}

pub enum AuthError {
    Unauthorized: (),
}

pub enum AccountError {
    InsufficientBalance: (u64, u64, bool),
}

pub enum MathError {
    Overflow: (),
}
