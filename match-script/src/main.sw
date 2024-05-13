script;

use std::constants::ZERO_B256;
use i64::I64;

configurable {
    ORDER_BOOK_CONTRACT_ID: b256 = ZERO_B256,
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

fn main(order_id: b256, orders: Vec<b256>) {
    let caller = abi(OrderBook, ORDER_BOOK_CONTRACT_ID);

    let mut order = caller.order_by_id(order_id).unwrap();
    let is_order_sell = order.base_size.negative;

    let mut index = 0;
    while index < orders.len() {
        let other_order_id = orders.get(index).unwrap();
        let other_order_option = caller.order_by_id(other_order_id);
        if other_order_option.is_none() {
            index += 1;
            continue;
        }

        let other_order = other_order_option.unwrap();
        let is_other_order_sell = other_order.base_size.negative;

        if is_order_sell && !is_other_order_sell {
            caller.match_orders(order.id, other_order.id);
        } else if !is_order_sell && is_other_order_sell {
            caller.match_orders(other_order.id, order.id);
        }

        let order_option = caller.order_by_id(order_id);

        if order_option.is_none() {
            return
        } else {
            order = order_option.unwrap()
        }

        index += 1;
    }
}
