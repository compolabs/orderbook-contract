library;
use ::structs::*;
use i64::I64;

struct MarketCreateEvent {
    asset_id: AssetId,
    decimal: u32,
    timestamp: u64,
}

struct OrderChangeEvent {
    id: b256,
    trader: Address,
    base_token: AssetId,
    base_size: I64,
    order_price: u64,
    timestamp: u64,
}

struct TradeEvent {
    base_token: AssetId,
    matcher: Address,
    trade_amount: u64,
    price: u64,
    timestamp: u64,
}