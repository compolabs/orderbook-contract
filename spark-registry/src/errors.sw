library;

pub enum AuthError {
    Unauthorized: (),
}

pub enum MarketRegistryError {
    MarketAlreadyRegistered: (),
    MarketNotRegistered: (),
}
