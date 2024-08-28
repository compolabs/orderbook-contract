library;

use ::data_structures::{asset_type::AssetType, order_type::OrderType, protocol_fee::ProtocolFee};

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
    pub order_type: OrderType,
    pub order_id: b256,
    pub price: u64,
    pub user: Identity,
}

pub struct SetEpochEvent {
    pub epoch: u64,
    pub epoch_duration: u64,
}

pub struct SetProtocolFeeEvent {
    pub protocol_fee: Vec<ProtocolFee>,
}

pub struct SetMatcherRewardEvent {
    pub amount: u64,
}

pub struct MatchOrderEvent {
    pub order_id: b256,
    pub asset: AssetId,
    pub order_matcher: Identity,
    pub owner: Identity,
    pub counterparty: Identity,
    pub match_size: u64,
    pub match_price: u64,
}

pub struct TradeOrderEvent {
    pub base_sell_order_id: b256,
    pub base_buy_order_id: b256,
    pub order_matcher: Identity,
    pub trade_size: u64,
    pub trade_price: u64,
    pub block_height: u32,
    pub tx_id: b256,
}

pub struct WithdrawEvent {
    pub amount: u64,
    pub asset: AssetId,
    pub user: Identity,
}
