library;
use ::structs::*;
use i64::I64;

pub struct MarketCreateEvent {
    pub asset_id: AssetId,
    pub asset_decimals: u32,
    pub timestamp: u64,
    pub tx_id: b256,
}

pub struct TradeEvent {
    pub base_token: AssetId,
    pub order_matcher: Address,
    pub seller: Address,
    pub buyer: Address,
    pub trade_size: u64,
    pub trade_price: u64,
    pub sell_order_id: b256,
    pub buy_order_id: b256,
    pub timestamp: u64,
    pub tx_id: b256,
}

enum OrderChangeEventIdentifier {
    OrderOpenEvent: (),
    OrderCancelEvent: (),
    OrderMatchEvent: (),
}

pub struct OrderChangeEvent {
    pub order_id: b256,
    pub sender: Identity,
    pub timestamp: u64,
    pub identifier: OrderChangeEventIdentifier,
    pub tx_id: b256,
    pub order: Option<Order>,
}

impl OrderChangeEvent {
    pub fn open(id: b256, order: Option<Order>) -> self {
        Self {
            order_id: id,
            order,
            sender: std::auth::msg_sender().unwrap(),
            identifier: OrderChangeEventIdentifier::OrderOpenEvent,
            timestamp: std::block::timestamp(),
            tx_id: std::tx::tx_id(),
        }
    }

    pub fn cancel(order_id: b256, order: Option<Order>) -> self {
        Self {
            order_id,
            order,
            sender: std::auth::msg_sender().unwrap(),
            identifier: OrderChangeEventIdentifier::OrderCancelEvent,
            timestamp: std::block::timestamp(),
            tx_id: std::tx::tx_id(),
        }
    }

    pub fn match_orders(id: b256, order: Option<Order>) -> self {
        Self {
            order_id: id,
            order,
            sender: std::auth::msg_sender().unwrap(),
            identifier: OrderChangeEventIdentifier::OrderMatchEvent,
            timestamp: std::block::timestamp(),
            tx_id: std::tx::tx_id(),
        }
    }
}
