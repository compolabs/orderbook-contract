contract;

// TODO: compiler regression, order matters or it won't compile
mod errors;
mod data_structures;
mod events;
mod interface;
mod math;

use ::data_structures::{
    account::Account,
    asset_type::AssetType,
    balance::Balance,
    order::Order,
    order_type::OrderType,
};
use ::errors::{AccountError, AssetError, AuthError, OrderError, ValueError};
use ::events::{
    CancelOrderEvent,
    DepositEvent,
    OpenOrderEvent,
    SetFeeEvent,
    TradeEvent,
    WithdrawEvent,
};
use ::interface::{Info, Market};
use ::math::*;

use std::{
    asset::transfer,
    call_frames::msg_asset_id,
    constants::ZERO_B256,
    context::msg_amount,
    hash::{
        Hash,
        sha256,
    },
    storage::storage_vec::*,
};

configurable {
    BASE_ASSET: AssetId = AssetId::from(ZERO_B256),
    BASE_ASSET_DECIMALS: u32 = 9,
    OWNER: Address = Address::from(ZERO_B256),
    PRICE_DECIMALS: u32 = 9,
    QUOTE_ASSET: AssetId = AssetId::from(ZERO_B256),
    QUOTE_ASSET_DECIMALS: u32 = 9,
}

storage {
    // Balance of each user
    account: StorageMap<Identity, Account> = StorageMap {},
    // Global fee for regular users
    fee: u64 = 0,
    // All of the currently open orders
    orders: StorageMap<b256, Order> = StorageMap {},
    // Internal handling of indexes for user_orders
    user_order_indexes: StorageMap<Identity, StorageMap<b256, u64>> = StorageMap {},
    // Fee for premium users which ought to be smaller than the global fee
    // Map(user => fee)
    premium_user: StorageMap<Identity, u64> = StorageMap {},
    // Indexing orders by user
    user_orders: StorageMap<Identity, StorageVec<b256>> = StorageMap {},
}

impl Market for Contract {
    #[payable]
    #[storage(read, write)]
    fn deposit() {
        let amount = msg_amount();
        require(amount > 0, ValueError::InvalidAmount);

        let asset = msg_asset_id();
        let asset_type = get_asset_type(asset);
        let user = msg_sender().unwrap();

        let mut account = storage.account.get(user).try_read().unwrap_or(Account::new());
        account.liquid.credit(amount, asset_type);
        storage.account.insert(user, account);

        log(DepositEvent {
            amount,
            asset,
            user,
        });
    }

    #[storage(read, write)]
    fn withdraw(amount: u64, asset: AssetId) {
        require(amount > 0, ValueError::InvalidAmount);

        let asset_type = get_asset_type(asset);
        let user = msg_sender().unwrap();

        let mut account = storage.account.get(user).try_read().unwrap_or(Account::new());

        account.liquid.debit(amount, asset_type);

        storage.account.insert(user, account);

        transfer(user, asset, amount);

        log(WithdrawEvent {
            amount,
            asset,
            user,
        });
    }

    #[storage(read, write)]
    fn open_order(
        amount: u64,
        asset: AssetId,
        order_type: OrderType,
        price: u64,
    ) -> b256 {
        require(amount > 0, ValueError::InvalidAmount);

        let asset_type = get_asset_type(asset);
        let user = msg_sender().unwrap();
        let mut account = storage.account.get(user).try_read().unwrap_or(Account::new());

        match order_type {
            OrderType::Sell => {
                account.lock_amount(amount, asset_type);
            }
            OrderType::Buy => {
                account.lock_amount(
                    convert(
                        amount,
                        BASE_ASSET_DECIMALS,
                        price,
                        PRICE_DECIMALS,
                        QUOTE_ASSET_DECIMALS,
                        asset_type == AssetType::Base,
                    ),
                    !asset_type,
                );
            }
        }

        let order = Order::new(amount, asset, asset_type, order_type, user, price);
        let order_id = order.id();

        // Reject identical orders
        require(
            storage
                .orders
                .get(order_id)
                .try_read()
                .is_none(),
            OrderError::DuplicateOrder,
        );

        // Store the new order and update the state of the user's account
        storage.orders.insert(order_id, order);
        storage.account.insert(user, account);

        // Indexing
        storage.user_orders.get(user).push(order_id);
        storage
            .user_order_indexes
            .get(user)
            .insert(order_id, storage.user_orders.get(user).len() - 1);

        log(OpenOrderEvent {
            amount,
            asset,
            asset_type,
            order_type,
            order_id,
            price,
            user,
        });
        order_id
    }

    #[storage(read, write)]
    fn cancel_order(order_id: b256) {
        // Order must exist to be cancelled
        let order = storage.orders.get(order_id).try_read();
        require(order.is_some(), OrderError::NoOrdersFound);

        let order = order.unwrap();
        let user = msg_sender().unwrap();

        // Only the owner of the order may cancel their order
        require(user == order.owner, AuthError::Unauthorized);

        // Safe to read() because user is the owner of the order
        let mut account = storage.account.get(user).read();

        // Order is about to be cancelled, unlock illiquid funds
        match order.order_type {
            OrderType::Sell => {
                account.unlock_amount(order.amount, order.asset_type);
            }
            OrderType::Buy => {
                account.unlock_amount(
                    convert(
                        order.amount,
                        BASE_ASSET_DECIMALS,
                        order.price,
                        PRICE_DECIMALS,
                        QUOTE_ASSET_DECIMALS,
                        order.asset_type == AssetType::Base,
                    ),
                    !order.asset_type,
                );
            }
        }

        remove_order(user, order_id);

        storage.account.insert(user, account);

        log(CancelOrderEvent { order_id });
    }

    #[storage(read, write)]
    fn match_orders(orders: Vec<b256>) {}

    #[storage(write)]
    fn set_fee(amount: u64, user: Option<Identity>) {
        require(
            msg_sender()
                .unwrap()
                .as_address()
                .unwrap() == OWNER,
            AuthError::Unauthorized,
        );

        match user {
            Some(identity) => storage.premium_user.insert(identity, amount),
            None => storage.fee.write(amount),
        };

        log(SetFeeEvent { amount, user });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn account(user: Identity) -> Option<Account> {
        storage.account.get(user).try_read()
    }

    #[storage(read)]
    fn fee(user: Option<Identity>) -> u64 {
        match user {
            Some(identity) => storage.premium_user.get(identity).try_read().unwrap_or(storage.fee.read()),
            None => storage.fee.read(),
        }
    }

    #[storage(read)]
    fn order(order: b256) -> Option<Order> {
        storage.orders.get(order).try_read()
    }

    #[storage(read)]
    fn user_orders(user: Identity) -> Vec<b256> {
        storage.user_orders.get(user).load_vec()
    }

    fn config() -> (Address, AssetId, u32, AssetId, u32, u32) {
        (
            OWNER,
            BASE_ASSET,
            BASE_ASSET_DECIMALS,
            QUOTE_ASSET,
            QUOTE_ASSET_DECIMALS,
            PRICE_DECIMALS,
        )
    }

    fn order_id(
        amount: u64,
        asset: AssetId,
        order_type: OrderType,
        owner: Identity,
        price: u64,
    ) -> b256 {
        require(
            asset == BASE_ASSET || asset == QUOTE_ASSET,
            AssetError::InvalidAsset,
        );
        let asset_type = if asset == BASE_ASSET {
            AssetType::Base
        } else {
            AssetType::Quote
        };
        Order::new(amount, asset, asset_type, order_type, owner, price).id()
    }
}

fn get_asset_type(asset_id: AssetId) -> AssetType {
    if asset_id == BASE_ASSET {
        AssetType::Base
    } else if asset_id == QUOTE_ASSET {
        AssetType::Quote
    } else {
        require(false, AssetError::InvalidAsset);
        AssetType::Quote
    }
}

#[storage(read, write)]
fn remove_order(user: Identity, order_id: b256) {
    require(storage.orders.remove(order_id), OrderError::FailedToRemove);

    let index = storage.user_order_indexes.get(user).get(order_id).read();
    let order_count = storage.user_orders.get(user).len();

    require(
        storage
            .user_order_indexes
            .get(user)
            .remove(order_id),
        OrderError::FailedToRemove,
    ); // TODO: Different error
    if order_count == 1 {
        // There's only 1 element so no swapping. Pop it from the end
        require(
            storage
                .user_orders
                .get(user)
                .pop()
                .unwrap() == order_id,
            OrderError::FailedToRemove,
        );
    } else {
        // The order ID at the end is about to have its index changed via swap_remove()
        let last_element = storage.user_orders.get(user).last().unwrap().read();

        // Remove the current order by replacing it with the order at the end of storage vec
        require(
            storage
                .user_orders
                .get(user)
                .swap_remove(index) == order_id,
            OrderError::FailedToRemove,
        );

        // Last element has been shifted so update its index
        storage
            .user_order_indexes
            .get(user)
            .insert(last_element, index);
    }
}
