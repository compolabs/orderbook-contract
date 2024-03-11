library;
use ::structs::*;
use i64::I64;

pub struct MarketCreateEvent {
    asset_id: AssetId,
    asset_decimals: u32,
    timestamp: u64,
}

pub struct OrderChangeEvent {
    order_id: b256,
    trader: Address,
    base_token: AssetId,
    base_size_change: I64,
    base_price: u64,
    timestamp: u64,
}

pub struct TradeEvent {
    base_token: AssetId,
    order_matcher: Address,
    seller: Address,
    buyer: Address,
    trade_size: u64,
    trade_price: u64,
    timestamp: u64,
}
