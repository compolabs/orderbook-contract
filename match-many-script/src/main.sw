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

pub enum Error {
    OrdersCantBeMatched: (),
}


fn main(order_sell_ids: Vec<b256>, order_buy_ids: Vec<b256>) {
    let s_len = order_sell_ids.len();
    let b_len = order_buy_ids.len();
    require(s_len > 0 && b_len > 0, Error::OrdersCantBeMatched);
    
    let caller = abi(OrderBook, ORDER_BOOK_CONTRACT_ID);

    let mut s = 0;
    let mut b = 0;
    while true {
        let sell_id = order_sell_ids.get(s).unwrap();
        let buy_id = order_buy_ids.get(b).unwrap();
        caller.match_orders(sell_id, buy_id);
        
        if caller.order_by_id(sell_id).is_none() {
            s += 1;
            if s == s_len {
                break;
            }
        }
        if caller.order_by_id(buy_id).is_none() {
            b += 1;
            if b == b_len {
                break;
            }
        }
    }
}
