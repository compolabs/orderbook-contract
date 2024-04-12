library;

use i64::I64;

pub struct Order {
    id: b256,
    trader: Identity,
    base_token: AssetId,
    base_size: I64,
    base_price: u64,
}
