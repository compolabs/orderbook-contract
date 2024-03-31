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
    InvalidUpdate: (),
}

pub enum AuthError {
    Unauthorized: (),
}

pub enum AccountError {
    InsufficientBalance: (),
    InvalidUser: (),
}
