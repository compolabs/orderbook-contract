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

pub struct MatchOrderEvent {
    pub order_id: b256,
    pub asset: AssetId,
    pub order_matcher: Identity,
    pub seller: Identity,
    pub buyer: Identity,
    pub match_size: u64,
    pub match_price: u64,
}

pub struct WithdrawEvent {
    pub amount: u64,
    pub asset: AssetId,
    pub user: Identity,
}
