library;

use i64::I64;

pub struct CreateMarketEvent {
    asset: AssetId,
    decimals: u32,
}

pub struct OpenOrderEvent {
    order_id: b256,
    trader: Address,
    asset: AssetId,
    size: I64,
    price: u64,
}

pub struct UpdateOrderEvent {
    order_id: b256,
    size: I64,
    price: u64,
}

pub struct TradeEvent {
    asset: AssetId,
    order_matcher: Address,
    seller: Address,
    buyer: Address,
    trade_size: u64,
    trade_price: u64,
}

pub struct CancelOrderEvent {
    order_id: b256,
}
