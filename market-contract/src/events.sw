library;

use ::data_structures::{asset_type::AssetType, order_type::OrderType};

pub struct CancelOrderEvent {
    order_id: b256,
}

pub struct DepositEvent {
    amount: u64,
    asset: AssetId,
    user: Identity,
}

pub struct OpenOrderEvent {
    base_amount: u64,
    order_type: OrderType,
    order_id: b256,
    price: u64,
    user: Identity,
}

pub struct SetFeeEvent {
    amount: u64,
    user: Option<Identity>,
}

// TODO: trading events
pub struct TradeEvent {
    asset: AssetId,
    order_matcher: Address,
    seller: Address,
    buyer: Address,
    trade_size: u64,
    trade_price: u64,
}

pub struct WithdrawEvent {
    amount: u64,
    asset: AssetId,
    user: Identity,
}
