script;

use std::constants::ZERO_B256;
use i64::I64;

configurable {
    ORDER_BOOK_CONTRACT_ID: b256 = ZERO_B256
}

pub struct Order {
    id: b256,
    trader: Address,
    base_token: AssetId,
    base_size: I64,
    base_price: u64,
}

abi OrderBook {
    #[storage(read, write)]
    fn match_orders(sell_order_id: b256, buy_order_id: b256);

    #[storage(read)]
    fn order_by_id(order_id: b256) -> Option<Order>;
}

fn main(order_sell_ids: Vec<b256>, order_buy_ids: Vec<b256>) {
    require(
        order_sell_ids
            .len > 0 && order_buy_ids
            .len > 0,
        "Error::OrdersCantBeMatched",
    );
    let caller = abi(OrderBook, ORDER_BOOK_CONTRACT_ID);

    let mut s = 0;
    let mut b = 0;
    while (s < order_sell_ids.len && b < order_buy_ids.len) {
        let sid = order_sell_ids.get(s).unwrap();
        let bid = order_buy_ids.get(b).unwrap();
        caller.match_orders(sid, bid);
        
        if caller.order_by_id(sid).is_none() {
            s += 1;
        }
        if caller.order_by_id(bid).is_none() {
            b += 1;
        }
    }
}
