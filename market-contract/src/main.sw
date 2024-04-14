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
use ::errors::{AccountError, AssetError, AuthError, OrderError, TradeError};
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
    asset::*,
    call_frames::msg_asset_id,
    constants::{
        BASE_ASSET_ID,
        ZERO_B256,
    },
    context::msg_amount,
    hash::{
        Hash,
        sha256,
    },
    storage::storage_vec::*,
};

configurable {
    BASE_ASSET: AssetId = BASE_ASSET_ID,
    BASE_ASSET_DECIMALS: u32 = 9,
    OWNER: Address = Address::from(ZERO_B256),
    PRICE_DECIMALS: u32 = 9,
    QUOTE_ASSET: AssetId = BASE_ASSET_ID,
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
        let asset = msg_asset_id();
        let amount = msg_amount();

        require(asset == BASE_ASSET || asset == QUOTE_ASSET, AssetError::InvalidAsset);
        let asset_type = if asset == BASE_ASSET { AssetType::Base } else { AssetType::Quote };
        
        let user = msg_sender().unwrap();

        let mut account = storage.account.get(user).try_read().unwrap_or(Account::new());

        account.liquid.credit(amount, asset_type);
        storage.account.insert(user, account);

        log(DepositEvent { amount, asset, user });

    }

    #[storage(read, write)]
    fn withdraw(amount: u64, asset: AssetId) {
        require(asset == BASE_ASSET || asset == QUOTE_ASSET, AssetError::InvalidAsset);
        let asset_type = if asset == BASE_ASSET { AssetType::Base } else { AssetType::Quote };

        let user = msg_sender().unwrap();
        let account = storage.account.get(user).try_read();
        require(account.is_some(), AccountError::InvalidUser);
        let mut account = account.unwrap();

        account.liquid.debit(amount, asset_type);
        storage.account.insert(user, account);

        transfer(user, asset, amount);
        
        log(WithdrawEvent { amount, asset, user });
    }

    #[storage(read, write)]
    fn open_order(amount: u64, asset: AssetId, order_type: OrderType, price: u64) -> b256 {
        require(asset == BASE_ASSET || asset == QUOTE_ASSET, AssetError::InvalidAsset);

        let asset_type = if asset == BASE_ASSET { AssetType::Base } else { AssetType::Quote };
        let user = msg_sender().unwrap();
        let account = storage.account.get(user).try_read();

        // An order may be open if an account exists
        require(account.is_some(), AccountError::InvalidUser);

        let mut account = account.unwrap();

        match order_type {
            OrderType::Sell => {
                // If the account has enough liquidity of the asset that you already own then lock 
                // it for the new sell order

                let base_amount = if asset == BASE_ASSET {
                    // example open_order(0.5, btc, SELL, 70k)
                    amount
                } else {
                    // example open_order(35k, usdc, SELL, 70k)
                    quote_to_base_amount(amount, BASE_ASSET_DECIMALS, price, PRICE_DECIMALS, QUOTE_ASSET_DECIMALS)
                };

                account.liquid.debit(base_amount, AssetType::Base);
                account.locked.credit(base_amount, AssetType::Base);
            }
            OrderType::Buy => {
                // Calculate amount to lock of the other asset

                let quote_amount = if asset == BASE_ASSET {
                    // example open_order(0.5, btc, BUY, 70k)
                    base_to_quote_amount(amount, BASE_ASSET_DECIMALS, price, PRICE_DECIMALS, QUOTE_ASSET_DECIMALS)
                } else {
                    // example open_order(35k, usdc, BUY, 70k)
                    amount
                };

                account.liquid.debit(quote_amount, AssetType::Quote);
                account.locked.credit(quote_amount, AssetType::Quote);
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
                // TODO: use amount to credit liquid balance
                let (_amount, asset_type) = if order.asset == BASE_ASSET {
                    (order.amount * BASE_ASSET_DECIMALS.as_u64(), AssetType::Base)
                } else {
                    (order.amount * QUOTE_ASSET_DECIMALS.as_u64(), AssetType::Quote)
                };
                account.locked.debit(order.amount, order.asset_type);
                account.liquid.credit(order.amount, order.asset_type);
            }
            OrderType::Buy => {
                // TODO: these "amounts" do not return expected values
                let amount = match order.asset == BASE_ASSET {
                    true => quote_to_base_amount(
                        order.amount,
                        BASE_ASSET_DECIMALS,
                        order.price,
                        PRICE_DECIMALS,
                        QUOTE_ASSET_DECIMALS,
                    ),
                    false => base_to_quote_amount(
                        order.amount,
                        BASE_ASSET_DECIMALS,
                        order.price,
                        PRICE_DECIMALS,
                        QUOTE_ASSET_DECIMALS,
                    ),
                };

                // Swap the asset types because you've payed with what you have when you were buying the other asset
                let asset_type = if order.asset == BASE_ASSET {
                    AssetType::Quote
                } else {
                    AssetType::Base
                };

                account.locked.credit(amount, asset_type);
                account.liquid.debit(amount, asset_type);
            }
        }

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

        storage.account.insert(user, account);

        log(CancelOrderEvent { order_id });
    }

    // #[storage(read, write)]
    // fn fulfill(order_id: b256);

    #[storage(read, write)]
    fn batch_fulfill(order_sell_id: b256, order_buy_id: b256) {

        let order_buy = storage.orders.get(order_buy_id).try_read();
        let order_sell = storage.orders.get(order_sell_id).try_read();
        require(order_buy.is_some() && order_sell.is_some(), OrderError::NoOrdersFound);

        let order_buy = order_buy.unwrap();
        let order_sell = order_sell.unwrap();

        // let mut buyer_account = storage.account.get(order_buy.owner).read();
        // let mut seller_account = storage.account.get(order_sell.owner).read();

        require(order_buy.order_type == OrderType::Buy, TradeError::CannotTrade);
        require(order_sell.order_type == OrderType::Sell, TradeError::CannotTrade);
        
        require(order_buy.asset == order_sell.asset, TradeError::CannotTrade);
        require(order_sell.price <= order_buy.price, TradeError::CannotTrade);
    

        let mut tmp = order_sell;
        // tmp.amount = tmp.amount.flip();
        let trade_size = min(order_sell.amount, order_buy.amount.mul_div(order_buy.price, order_sell.price));
        tmp.amount = trade_size;
        
        let seller: Identity = order_sell.owner;
        let (sellerDealAssetId, sellerDealRefund) = order_return_asset_amount(tmp);
        remove_update_order_internal(order_sell, tmp.amount);

        // tmp.amount = tmp.amount.flip();

        let buyer: Identity = order_buy.owner;
        let (buyerDealAssetId, buyerDealRefund) = order_return_asset_amount(tmp);
        tmp.amount = tmp.amount.mul_div_rounding_up(order_sell.price, order_buy.price);
        remove_update_order_internal(order_buy, tmp.amount);

        require(
            sellerDealRefund != 0 && buyerDealRefund != 0,
            TradeError::CannotTrade,
        );

        transfer(seller, sellerDealAssetId, sellerDealRefund);
        transfer(buyer, buyerDealAssetId, buyerDealRefund);
    }

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


#[storage(read)]
fn order_return_asset_amount(order: Order) -> (AssetId, u64) {
    match order.order_type {
        OrderType::Sell => {
            (order.asset, order.amount)
        },
        OrderType::Buy => {
            (
                QUOTE_ASSET,
                base_size_to_quote_amount(
                    order.amount,
                    QUOTE_ASSET_DECIMALS,
                    order.price,
                ),
            )
        }
    }
}

fn base_size_to_quote_amount(base_size: u64, base_decimals: u32, base_price: u64) -> u64 {
    base_size.mul_div(
        base_price,
        10_u64
            .pow(base_decimals + PRICE_DECIMALS - QUOTE_ASSET_DECIMALS),
    )
}

#[storage(read, write)]
fn remove_update_order_internal(order: Order, base_size: u64) {
    if (order.amount == base_size && order.order_type == OrderType::Sell) {
        let pos_id = storage.user_order_indexes.get(order.owner).get(order.id()).read() - 1; // pos + 1 indexed
        assert(storage.user_order_indexes.get(order.owner).remove(order.id()));
        assert(storage.user_orders.get(order.owner).swap_remove(pos_id) == order.id());
        assert(storage.orders.remove(order.id()));
    } else {
        let mut order = order;
        order.amount += base_size;
        storage.orders.insert(order.id(), order);
    }
}


pub fn min(a: u64, b: u64) -> u64 {
    if a < b { a } else { b }
}