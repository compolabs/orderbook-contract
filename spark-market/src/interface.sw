library;

use ::data_structures::{
    account::Account,
    asset_type::AssetType,
    limit_type::LimitType,
    order::Order,
    order_change::OrderChangeInfo,
    order_type::OrderType,
    protocol_fee::ProtocolFee,
};

abi SparkMarket {
    #[payable]
    #[storage(read, write)]
    fn deposit();

    #[storage(read, write)]
    fn withdraw(amount: u64, asset_type: AssetType);

    #[storage(read, write)]
    fn open_order(amount: u64, order_type: OrderType, price: u64) -> b256;

    #[storage(read, write)]
    fn cancel_order(order_id: b256);

    #[storage(read, write)]
    fn match_order_pair(order0_id: b256, order1_id: b256);

    #[storage(read, write)]
    fn match_order_many(orders: Vec<b256>);

    #[payable]
    #[storage(read, write)]
    fn fulfill_order_many(
        amount: u64,
        order_type: OrderType,
        limit_type: LimitType,
        price: u64,
        slippage: u64,
        orders: Vec<b256>,
    ) -> b256;

    #[storage(write)]
    fn set_epoch(epoch: u64, epoch_duration: u64);

    #[storage(write)]
    fn set_protocol_fee(protocol_fee: Vec<ProtocolFee>);

    #[storage(read, write)]
    fn set_matcher_fee(amount: u64);
}

abi SparkMarketInfo {
    #[storage(read)]
    fn account(user: Identity) -> Account;

    #[storage(read)]
    fn get_epoch() -> (u64, u64);

    #[storage(read)]
    fn matcher_fee() -> u64;

    #[storage(read)]
    fn protocol_fee() -> Vec<ProtocolFee>;

    #[storage(read)]
    fn protocol_fee_user(user: Identity) -> (u64, u64);

    #[storage(read)]
    fn protocol_fee_user_amount(amount: u64, user: Identity) -> (u64, u64);

    #[storage(read)]
    fn order(order: b256) -> Option<Order>;

    #[storage(read)]
    fn user_orders(user: Identity) -> Vec<b256>;

    #[storage(read)]
    fn order_change_info(order_id: b256) -> Vec<OrderChangeInfo>;

    fn config() -> (AssetId, u32, AssetId, u32, Identity, u32, u32);

    fn order_id(
        order_type: OrderType,
        owner: Identity,
        price: u64,
        block_height: u32,
        order_height: u64,
    ) -> b256;
}
