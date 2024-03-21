library;

use ::data_structures::{Market, Order};
use i64::I64;

abi OrderBook {
    #[storage(read, write)]
    fn create_market(asset_id: AssetId, decimal: u32);

    #[payable]
    #[storage(read, write)]
    fn open_order(base_token: AssetId, base_size: I64, order_price: u64) -> b256;

    #[storage(read, write)]
    fn cancel_order(order_id: b256);

    #[storage(read, write)]
    fn match_orders(order_sell_id: b256, order_buy_id: b256);
}

abi Info {
    #[storage(read)]
    fn orders_by_trader(trader: Address) -> Vec<b256>;

    #[storage(read)]
    fn order_by_id(order: b256) -> Option<Order>;

    #[storage(read)]
    fn market_exists(asset_id: AssetId) -> bool;

    #[storage(read)]
    fn get_market_by_id(asset_id: AssetId) -> Market;

    fn get_configurables() -> (AssetId, u32, u32);
}
