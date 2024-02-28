library;
use i64::I64;

pub struct Order {
    id: b256,
    trader: Address,
    base_token: AssetId,
    base_size: I64,
    base_price: u64,
}

pub struct Market {
    asset_id: AssetId,
    asset_decimals: u32,
}
