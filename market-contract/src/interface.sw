library;

use ::data_structures::{account::Account, order::Order, order_type::OrderType};

abi Market {
    #[payable]
    #[storage(read, write)]
    fn deposit();

    #[storage(read, write)]
    fn withdraw(amount: u64, asset: AssetId);

    #[storage(read, write)]
    fn open_order(
        amount: u64,
        asset: AssetId,
        order_type: OrderType,
        price: u64,
    ) -> b256;

    #[storage(read, write)]
    fn cancel_order(order_id: b256);

    // #[storage(read, write)]
    // fn fulfill(order_id: b256);

    #[storage(read, write)]
    fn batch_fulfill(order_sell_id: b256, order_buy_id: b256);

    #[storage(write)]
    fn set_fee(amount: u64, user: Option<Identity>);
}

abi Info {
    #[storage(read)]
    fn account(user: Identity) -> Option<Account>;

    #[storage(read)]
    fn fee(user: Option<Identity>) -> u64;

    #[storage(read)]
    fn order(order: b256) -> Option<Order>;

    #[storage(read)]
    fn user_orders(user: Identity) -> Vec<b256>;

    fn config() -> (Address, AssetId, u32, AssetId, u32, u32);

    fn order_id(
        amount: u64,
        asset: AssetId,
        order_type: OrderType,
        owner: Identity,
        price: u64,
    ) -> b256;
}
