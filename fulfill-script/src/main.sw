script;

use std::constants::ZERO_B256;
use std::context::msg_amount;
use std::call_frames::msg_asset_id;
use std::tx::tx_id;
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
    #[storage(read, write), payable]
    fn open_order(base_token: AssetId, base_size: I64, order_price: u64) -> b256;

    #[storage(read, write)]
    fn match_orders(order_sell_id: b256, order_buy_id: b256);

    #[storage(read)]
    fn order_by_id(order_id: b256) -> Option<Order>;

    #[storage(read)]
    fn market_exists(asset_id: AssetId) -> bool;

    fn get_configurables() -> (AssetId, u32, u32);
}

#[payable]
fn main(
    orders: Vec<b256>,
    price: u64,
    base_token: AssetId,
    base_size: I64,
) {
    let caller = abi(OrderBook, ORDER_BOOK_CONTRACT_ID);

    let configurables = caller.get_configurables();

    let payment_asset = msg_asset_id();
    let payment_amount = msg_amount();

    require(
        payment_asset == configurables.0 || caller
            .market_exists(payment_asset),
        228,
    );
    require(caller.market_exists(base_token), 228);

    let new_order_id = caller.open_order {
        coins: payment_amount,
        asset_id: payment_asset.into(),
    }(base_token, base_size, price);

    let new_order = caller.order_by_id(new_order_id).unwrap();
    let is_new_order_sell = new_order.base_size.negative;

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

        if is_new_order_sell && !is_other_order_sell {
            caller.match_orders(new_order.id, other_order.id);
        } else if !is_new_order_sell && is_other_order_sell {
            caller.match_orders(other_order.id, new_order.id);
        }

        index += 1;
    }
}
