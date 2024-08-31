library;

pub enum AuthError {
    Unauthorized: (),
}

pub enum OrderbookError {
    MarketAlreadyRegistered: (),
    MarketNotRegistered: (),
}
