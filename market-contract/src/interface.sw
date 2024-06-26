library;

use ::data_structures::{
    account::Account,
    asset_type::AssetType,
    order::Order,
    order_change::OrderChangeInfo,
    order_type::OrderType,
};

abi Market {
    #[payable]
    #[storage(read, write)]
    fn deposit();

    #[storage(read, write)]
    fn withdraw(amount: u64, asset_type: AssetType);

    #[payable]
    #[storage(read, write)]
    fn open_order(
        amount: u64,
        asset_type: AssetType,
        order_type: OrderType,
        price: u64,
    ) -> b256;

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
        asset_type: AssetType,
        order_type: OrderType,
        price: u64,
        slippage: u64,
        orders: Vec<b256>,
    ) -> b256;

    #[storage(write)]
    fn set_fee(amount: u64, user: Option<Identity>);

    #[storage(write)]
    fn set_matcher_fee(amount: u32);
}

abi Info {
    #[storage(read)]
    fn account(user: Identity) -> Option<Account>;

    #[storage(read)]
    fn fee(user: Option<Identity>) -> u64;

    #[storage(read)]
    fn matcher_fee() -> u32;

    #[storage(read)]
    fn order(order: b256) -> Option<Order>;

    #[storage(read)]
    fn user_orders(user: Identity) -> Vec<b256>;

    #[storage(read)]
    fn order_change_info(order_id: b256) -> Vec<OrderChangeInfo>;

    fn config() -> (Address, AssetId, u32, AssetId, u32, u32);

    fn order_id(
        asset_type: AssetType,
        order_type: OrderType,
        owner: Identity,
        price: u64,
        block_height: u32,
    ) -> b256;
}
