library;
use ::structs::*;
use i64::I64;

pub struct MarketCreateEvent {
    asset_id: AssetId,
    asset_decimals: u32,
    timestamp: u64,
    tx_id: b256
}

pub struct TradeEvent {
    base_token: AssetId,
    order_matcher: Address,
    seller: Address,
    buyer: Address,
    trade_size: u64,
    trade_price: u64,
    sell_order_id: b256,
    buy_order_id: b256,
    timestamp: u64,
    tx_id: b256
}


enum OrderChangeEventIdentifier{
    OrderOpenEvent: (),
    OrderCancelEvent: (),
    OrderMatchEvent: (),
}


pub struct OrderChangeEvent {
    order_id: b256,
    sender: Identity,
    timestamp: u64,
    identifier: OrderChangeEventIdentifier,
    tx_id: b256,
    order: Option<Order>,
}

impl OrderChangeEvent{
    pub fn open(id: b256, order: Option<Order>) -> self {
        Self {
            order_id: id,
            order, 
            sender: std::auth::msg_sender().unwrap(),
            identifier: OrderChangeEventIdentifier::OrderOpenEvent,
            timestamp: std::block::timestamp(), 
            tx_id: std::tx::tx_id()
        }
    }
    
    
    pub fn cancel(order_id: b256, order: Option<Order>)-> self{
        Self {
            order_id,
            order,
            sender: std::auth::msg_sender().unwrap(),
            identifier: OrderChangeEventIdentifier::OrderCancelEvent,
            timestamp: std::block::timestamp(), 
            tx_id: std::tx::tx_id()
        }
    }
    
    
    pub fn match_orders(id: b256, order: Option<Order>)-> self{
        Self {
            order_id: id,
            order, 
            sender: std::auth::msg_sender().unwrap(),
            identifier: OrderChangeEventIdentifier::OrderMatchEvent,
            timestamp: std::block::timestamp(), 
            tx_id: std::tx::tx_id()
        }
    }    

}


