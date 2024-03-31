library;

use ::data_structures::{asset_type::AssetType, order_type::OrderType};

pub struct CancelOrderEvent {
    pub order_id: b256,
}

pub struct DepositEvent {
    pub amount: u64,
    pub asset: AssetId,
    pub user: Identity,
}

pub struct OpenOrderEvent {
    pub amount: u64,
    pub asset: AssetId,
    pub asset_type: AssetType,
    pub order_type: OrderType,
    pub order_id: b256,
    pub price: u64,
    pub user: Identity,
}

pub struct SetFeeEvent {
    pub amount: u64,
    pub user: Option<Identity>,
}

// TODO: trading events
pub struct TradeEvent {
    pub asset: AssetId,
    pub order_matcher: Address,
    pub seller: Address,
    pub buyer: Address,
    pub trade_size: u64,
    pub trade_price: u64,
}

pub struct UpdateOrderEvent {
    pub amount: Option<u64>,
    pub new_order_id: b256,
    pub order_id: b256,
    pub price: Option<u64>,
}

pub struct WithdrawEvent {
    pub amount: u64,
    pub asset: AssetId,
    pub user: Identity,
}
