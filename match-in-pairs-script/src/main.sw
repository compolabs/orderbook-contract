script;

use std::constants::ZERO_B256;

configurable {
    ORDER_BOOK_CONTRACT_ID: b256 = ZERO_B256,
}

abi OrderBook {
    #[storage(read, write)]
    fn match_orders(sell_order_id: b256, buy_order_id: b256);
}

fn main(orders: Vec<(b256, b256)>) {
    let caller = abi(OrderBook, ORDER_BOOK_CONTRACT_ID);
    let mut i = 0;
    while (i < orders.len()) {
        let (sell_id, buy_id) = orders.get(i).unwrap();
        caller.match_orders(sell_id, buy_id);
        i += 1;
    }
}
