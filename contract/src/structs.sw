library;
use i64::I64;

pub struct Order {
    pub id: b256,
    pub trader: Address,
    pub base_token: AssetId,
    pub base_size: I64,
    pub base_price: u64,
}

pub struct Market {
    pub asset_id: AssetId,
    pub asset_decimals: u32,
}
