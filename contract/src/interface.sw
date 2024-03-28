library;

use ::data_structures::Order;
use i64::I64;

abi OrderBook {
    #[storage(read, write)]
    fn create_market(asset: AssetId, decimal: u32);

    #[payable]
    #[storage(read, write)]
    fn open_order(asset: AssetId, size: I64, price: u64) -> b256;

    #[payable]
    #[storage(read, write)]
    fn update_order(order_id: b256, size: I64, price: u64);

    #[storage(read, write)]
    fn cancel_order(order_id: b256);

    #[storage(read, write)]
    fn match_orders(sell_order: b256, buy_order: b256);
}

abi Info {
    #[storage(read)]
    fn trader_orders(trader: Address) -> Vec<b256>;

    #[storage(read)]
    fn order(order: b256) -> Option<Order>;

    #[storage(read)]
    fn market(asset_id: AssetId) -> Option<u32>;

    fn configurables() -> (AssetId, u32, u32);

    fn order_id(trader: Address, asset: AssetId, price: u64) -> b256;
}
