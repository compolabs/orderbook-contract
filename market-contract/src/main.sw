contract;

// TODO: compiler regression, order matters or it won't compile
mod errors;
mod data_structures;
mod events;
mod interface;

use ::data_structures::{
    account::Account,
    asset_type::AssetType,
    balance::Balance,
    limit_type::LimitType,
    match_result::MatchResult,
    math::*,
    order::Order,
    order_change::OrderChangeInfo,
    order_change::OrderChangeType,
    order_type::OrderType,
    protocol_fee::*,
    user_volume::UserVolume,
};
use ::errors::{AccountError, AssetError, AuthError, MatchError, OrderError, ValueError};
use ::events::{
    CancelOrderEvent,
    DepositEvent,
    MatchOrderEvent,
    OpenOrderEvent,
    SetEpochEvent,
    SetMatcherRewardEvent,
    SetProtocolFeeEvent,
    TradeOrderEvent,
    WithdrawEvent,
};
use ::interface::{Market, MarketInfo};

use std::{
    asset::transfer,
    block::height as block_height,
    block::timestamp as block_timestamp,
    call_frames::msg_asset_id,
    context::msg_amount,
    hash::Hash,
    storage::storage_vec::*,
    tx::tx_id,
};

use sway_libs::reentrancy::*;

configurable {
    BASE_ASSET: AssetId = AssetId::zero(),
    BASE_ASSET_DECIMALS: u32 = 9,
    QUOTE_ASSET: AssetId = AssetId::zero(),
    QUOTE_ASSET_DECIMALS: u32 = 9,
    OWNER: Identity = Identity::Address(Address::zero()),
    PRICE_DECIMALS: u32 = 9,
    VERSION: u32 = 0,
}

storage {
    // Balance of each user
    account: StorageMap<Identity, Account> = StorageMap {},
    // All of the currently open orders
    orders: StorageMap<b256, Order> = StorageMap {},
    // Internal handling of indexes for user_orders
    user_order_indexes: StorageMap<Identity, StorageMap<b256, u64>> = StorageMap {},
    // Indexing orders by user
    user_orders: StorageMap<Identity, StorageVec<b256>> = StorageMap {},
    // Temporary order change log structure for indexer debug
    order_change_info: StorageMap<b256, StorageVec<OrderChangeInfo>> = StorageMap {},
    // Protocol fee
    protocol_fee: StorageVec<ProtocolFee> = StorageVec {},
    // The reward to the matcher for single order match
    matcher_fee: u64 = 0,
    // User trade volumes
    user_volumes: StorageMap<Identity, UserVolume> = StorageMap {},
    // Epoch
    epoch: u64 = 0,
    // Epoch duration 1 month (86400 * 365.25 / 12)
    epoch_duration: u64 = 2629800,
    // Order height
    order_height: u64 = 0,
}

impl Market for Contract {
    #[payable]
    #[storage(read, write)]
    fn deposit() {
        reentrancy_guard();

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
    fn withdraw(amount: u64, asset_type: AssetType) {
        reentrancy_guard();

        require(amount > 0, ValueError::InvalidAmount);

        let user = msg_sender().unwrap();
        let mut account = storage.account.get(user).try_read().unwrap_or(Account::new());

        account.liquid.debit(amount, asset_type);
        storage.account.insert(user, account);

        let asset = get_asset_id(asset_type);

        transfer(user, asset, amount);

        log(WithdrawEvent {
            amount,
            asset,
            user,
        });
    }

    #[storage(read, write)]
    fn open_order(amount: u64, order_type: OrderType, price: u64) -> b256 {
        reentrancy_guard();

        let asset_type = AssetType::Base;
        open_order_internal(
            amount,
            asset_type,
            order_type,
            price,
            storage
                .matcher_fee
                .read(),
        )
    }

    #[storage(read, write)]
    fn cancel_order(order_id: b256) {
        reentrancy_guard();

        cancel_order_internal(order_id);
    }

    #[storage(read, write)]
    fn match_order_pair(order0_id: b256, order1_id: b256) {
        reentrancy_guard();

        let order0 = storage.orders.get(order0_id).try_read();
        require(order0.is_some(), OrderError::OrderNotFound(order0_id));
        let order1 = storage.orders.get(order1_id).try_read();
        require(order1.is_some(), OrderError::OrderNotFound(order1_id));
        let (match_result, _) = match_order_internal(
            order0_id,
            order0
                .unwrap(),
            LimitType::GTC,
            order1_id,
            order1
                .unwrap(),
            LimitType::GTC,
        );
        require(
            match_result != MatchResult::ZeroMatch,
            MatchError::CantMatch((order0_id, order1_id)),
        );
    }

    #[storage(read, write)]
    fn match_order_many(orders: Vec<b256>) {
        reentrancy_guard();

        require(orders.len() >= 2, ValueError::InvalidArrayLength);

        let len = orders.len();
        let mut idx0 = 0;
        let mut idx1 = 1;
        let mut full_matched = 0;

        while lts(idx0, idx1, len) {
            if idx0 == idx1 {
                idx1 += 1;
                continue;
            }

            let id0 = orders.get(idx0).unwrap();
            let order0 = storage.orders.get(id0).try_read();
            if order0.is_none() {
                // the order already matched/cancelled or bad id
                idx0 += 1;
                continue;
            }

            let id1 = orders.get(idx1).unwrap();
            let order1 = storage.orders.get(id1).try_read();
            if order1.is_none() {
                // the order already matched/cancelled or bad id
                idx1 += 1;
                continue;
            }

            // try match
            let (match_result, partial_order_id) = match_order_internal(
                id0,
                order0
                    .unwrap(),
                LimitType::GTC,
                id1,
                order1
                    .unwrap(),
                LimitType::GTC,
            );

            match match_result {
                MatchResult::ZeroMatch => {
                    // the case when the 1st & 2nd orders play in same direction
                    if idx0 < idx1 { idx1 += 1; } else { idx0 += 1; }
                }
                MatchResult::PartialMatch => {
                    // the case when the one of the orders is partially completed
                    if partial_order_id == id0 {
                        idx1 += 1;
                    } else {
                        idx0 += 1;
                    }
                    full_matched += 1;
                }
                MatchResult::FullMatch => {
                    // the case when orders are completed
                    idx0 = min(idx0, idx1) + 1;
                    idx1 = idx0 + 1;
                    full_matched += 2;
                }
            }
        }
        require(full_matched > 0, MatchError::CantMatchMany);
    }

    #[payable]
    #[storage(read, write)]
    fn fulfill_order_many(
        amount: u64,
        order_type: OrderType,
        limit_type: LimitType,
        price: u64,
        slippage: u64,
        orders: Vec<b256>,
    ) -> b256 {
        reentrancy_guard();

        require(orders.len() > 0, ValueError::InvalidArrayLength);
        require(slippage <= HUNDRED_PERCENT, ValueError::InvalidSlippage);

        let asset_type = AssetType::Base;
        let id0 = open_order_internal(amount, asset_type, order_type, price, 0);
        let len = orders.len();
        let mut idx1 = 0;
        let mut matched = MatchResult::ZeroMatch;
        let slippage = price * slippage / HUNDRED_PERCENT;

        while idx1 < len {
            let order0 = storage.orders.get(id0).read();
            let id1 = orders.get(idx1).unwrap();
            let order1 = storage.orders.get(id1).try_read();
            if order1.is_some() {
                let order1 = order1.unwrap();
                if asset_type == AssetType::Base
                    && order_type == OrderType::Sell
                        || asset_type == AssetType::Quote
                        && order_type == OrderType::Buy
                    || distance(price, order1.price) <= slippage
                {
                    let (match_result, partial_order_id) = match_order_internal(id0, order0, limit_type, id1, order1, LimitType::GTC);
                    match match_result {
                        MatchResult::ZeroMatch => {}
                        MatchResult::PartialMatch => {
                            matched = if partial_order_id == id1 {
                                MatchResult::FullMatch
                            } else {
                                MatchResult::PartialMatch
                            };
                        }
                        MatchResult::FullMatch => {
                            matched = MatchResult::FullMatch;
                        }
                    }
                    if matched == MatchResult::FullMatch {
                        break;
                    }
                }
            }
            idx1 += 1;
        }
        require(
            !(matched == MatchResult::ZeroMatch),
            MatchError::CantFulfillMany,
        );
        require(
            !(matched == MatchResult::PartialMatch && limit_type == LimitType::FOK),
            MatchError::CantFulfillFOK,
        );

        if matched == MatchResult::PartialMatch {
            cancel_order_internal(id0);
        }

        id0
    }

    #[storage(write)]
    fn set_epoch(epoch: u64, epoch_duration: u64) {
        only_owner();

        let current_epoch = storage.epoch.read();
        let now = block_timestamp();

        require(
            epoch >= current_epoch && (epoch + epoch_duration > now),
            ValueError::InvalidEpoch((current_epoch, epoch, epoch_duration, now)),
        );

        storage.epoch.write(epoch);
        storage.epoch_duration.write(epoch_duration);

        log(SetEpochEvent {
            epoch: epoch,
            epoch_duration,
        });
    }

    #[storage(write)]
    fn set_protocol_fee(protocol_fee: Vec<ProtocolFee>) {
        only_owner();

        if protocol_fee.len() > 0 {
            require(
                protocol_fee
                    .get(0)
                    .unwrap()
                    .volume_threshold == 0,
                ValueError::InvalidFeeZeroBased,
            );
        }
        require(
            protocol_fee
                .is_volume_threshold_sorted(),
            ValueError::InvalidFeeSorting,
        );
        storage.protocol_fee.store_vec(protocol_fee);

        log(SetProtocolFeeEvent { protocol_fee });
    }

    #[storage(read, write)]
    fn set_matcher_fee(amount: u64) {
        only_owner();
        require(
            amount != storage
                .matcher_fee
                .read(),
            ValueError::InvalidValueSame,
        );
        storage.matcher_fee.write(amount);

        log(SetMatcherRewardEvent { amount });
    }
}

impl MarketInfo for Contract {
    #[storage(read)]
    fn account(user: Identity) -> Account {
        storage.account.get(user).try_read().unwrap_or(Account::new())
    }

    #[storage(read)]
    fn get_epoch() -> (u64, u64) {
        (storage.epoch.read(), storage.epoch_duration.read())
    }

    #[storage(read)]
    fn matcher_fee() -> u64 {
        storage.matcher_fee.read()
    }

    #[storage(read)]
    fn protocol_fee() -> Vec<ProtocolFee> {
        storage.protocol_fee.load_vec()
    }

    #[storage(read)]
    fn protocol_fee_user(user: Identity) -> (u64, u64) {
        protocol_fee_user(user)
    }

    #[storage(read)]
    fn protocol_fee_user_amount(amount: u64, user: Identity) -> (u64, u64) {
        protocol_fee_user_amount(amount, user)
    }

    #[storage(read)]
    fn order(order: b256) -> Option<Order> {
        storage.orders.get(order).try_read()
    }

    #[storage(read)]
    fn user_orders(user: Identity) -> Vec<b256> {
        storage.user_orders.get(user).load_vec()
    }

    #[storage(read)]
    fn order_change_info(order_id: b256) -> Vec<OrderChangeInfo> {
        storage.order_change_info.get(order_id).load_vec()
    }

    fn config() -> (AssetId, u32, AssetId, u32, Identity, u32, u32) {
        (
            BASE_ASSET,
            BASE_ASSET_DECIMALS,
            QUOTE_ASSET,
            QUOTE_ASSET_DECIMALS,
            OWNER,
            PRICE_DECIMALS,
            VERSION,
        )
    }

    fn order_id(
        order_type: OrderType,
        owner: Identity,
        price: u64,
        block_height: u32,
        order_height: u64,
    ) -> b256 {
        let asset_type = AssetType::Base;
        require(
            asset_type == AssetType::Base || asset_type == AssetType::Quote,
            AssetError::InvalidAsset,
        );
        Order::new(
            1,
            asset_type,
            order_type,
            owner,
            price,
            PRICE_DECIMALS,
            block_height,
            order_height,
            0,
            0,
            0,
        ).id()
    }
}

#[storage(read, write)]
fn next_order_height() -> u64 {
    let order_height = storage.order_height.read();
    storage.order_height.write(order_height + 1);
    order_height
}

#[storage(read, write)]
fn open_order_internal(
    amount: u64,
    asset_type: AssetType,
    order_type: OrderType,
    price: u64,
    matcher_fee: u64,
) -> b256 {
    require(amount > 0, ValueError::InvalidAmount);

    let user = msg_sender().unwrap();
    let (protocol_maker_fee, protocol_taker_fee) = protocol_fee_user(user);

    let mut order = Order::new(
        amount,
        asset_type,
        order_type,
        user,
        price,
        PRICE_DECIMALS,
        block_height(),
        next_order_height(),
        matcher_fee,
        protocol_maker_fee,
        protocol_taker_fee,
    );

    let order_id = order.id();
    require(
        storage
            .orders
            .get(order_id)
            .try_read()
            .is_none(),
        OrderError::OrderDuplicate(order_id),
    );

    // Indexing
    storage.user_orders.get(user).push(order_id);
    storage
        .user_order_indexes
        .get(user)
        .insert(order_id, storage.user_orders.get(user).len() - 1);

    // Store the new or updated order
    storage.orders.insert(order_id, order);

    // Update user account balance
    let mut account = storage.account.get(user).try_read().unwrap_or(Account::new());
    account.lock_amount(
        lock_order_amount(order),
        match order.order_type {
            OrderType::Sell => order.asset_type,
            OrderType::Buy => !order.asset_type,
        },
    );

    // Update the state of the user's account
    storage.account.insert(user, account);

    let asset = get_asset_id(asset_type);

    log_order_change_info(
        order_id,
        OrderChangeInfo::new(
            OrderChangeType::OrderOpened,
            block_height(),
            user,
            tx_id(),
            0,
            order.amount,
        ),
    );

    log(OpenOrderEvent {
        amount,
        asset,
        order_type,
        order_id,
        price,
        user,
    });
    order_id
}

#[storage(read, write)]
fn cancel_order_internal(order_id: b256) {
    // Order must exist to be cancelled
    let order = storage.orders.get(order_id).try_read();
    require(order.is_some(), OrderError::OrderNotFound(order_id));

    let order = order.unwrap();
    let user = msg_sender().unwrap();

    // Only the owner of the order may cancel their order
    require(user == order.owner, AuthError::Unauthorized);

    // Safe to read() because user is the owner of the order
    let mut account = storage.account.get(user).read();

    // Order is about to be cancelled, unlock liliquid funds
    account.unlock_amount(
        lock_order_amount(order),
        match order.order_type {
            OrderType::Sell => order.asset_type,
            OrderType::Buy => !order.asset_type,
        },
    );

    remove_order(user, order_id);
    storage.account.insert(user, account);

    log_order_change_info(
        order_id,
        OrderChangeInfo::new(
            OrderChangeType::OrderCancelled,
            block_height(),
            user,
            tx_id(),
            order.amount,
            0,
        ),
    );

    log(CancelOrderEvent { order_id });
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

fn get_asset_id(asset_type: AssetType) -> AssetId {
    if asset_type == AssetType::Base {
        BASE_ASSET
    } else if asset_type == AssetType::Quote {
        QUOTE_ASSET
    } else {
        require(false, AssetError::InvalidAsset);
        QUOTE_ASSET
    }
}

#[storage(write)]
fn extend_epoch() {
    let epoch_duration = storage.epoch_duration.read();
    let epoch = storage.epoch.read() + epoch_duration;

    if epoch <= block_timestamp() {
        storage.epoch.write(epoch);
        log(SetEpochEvent {
            epoch,
            epoch_duration,
        });
    }
}

#[storage(read)]
fn protocol_fee_user(user: Identity) -> (u64, u64) {
    let volume = storage.user_volumes.get(user).try_read().unwrap_or(UserVolume::new()).get(storage.epoch.read());
    let protocol_fee = storage.protocol_fee.get_volume_protocol_fee(volume);
    (protocol_fee.maker_fee, protocol_fee.taker_fee)
}

#[storage(read)]
fn protocol_fee_user_amount(amount: u64, user: Identity) -> (u64, u64) {
    let protocol_fee = protocol_fee_user(user);
    (
        amount * protocol_fee.0 / HUNDRED_PERCENT,
        amount * protocol_fee.1 / HUNDRED_PERCENT,
    )
}

#[storage(read, write)]
fn increase_user_volume(user: Identity, volume: u64) {
    extend_epoch();
    storage
        .user_volumes
        .get(user)
        .try_read()
        .unwrap_or(UserVolume::new())
        .update(storage.epoch.read(), volume);
}

#[storage(read, write)]
fn remove_order(user: Identity, order_id: b256) {
    require(
        storage
            .orders
            .remove(order_id),
        OrderError::FailedToRemove(order_id),
    );

    let index = storage.user_order_indexes.get(user).get(order_id).read();
    let order_count = storage.user_orders.get(user).len();

    require(
        storage
            .user_order_indexes
            .get(user)
            .remove(order_id),
        OrderError::FailedToRemove(order_id),
    ); // TODO: Different error
    if order_count == 1 {
        // There's only 1 element so no swapping. Pop it from the end
        require(
            storage
                .user_orders
                .get(user)
                .pop()
                .unwrap() == order_id,
            OrderError::FailedToRemove(order_id),
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
            OrderError::FailedToRemove(order_id),
        );

        // Last element has been shifted so update its index
        storage
            .user_order_indexes
            .get(user)
            .insert(last_element, index);
    }
}

#[storage(read, write)]
fn execute_trade(
    s_order: Order,
    b_order: Order,
    trade_size: u64,
    matcher: Identity,
) -> (u64, u64, u64) {
    let asset_type = s_order.asset_type;
    // it is a trade volume
    let s_trade_volume = quote_of_base_amount(trade_size, s_order.price);
    // it is trade volume reserved by buyer for trade_size
    let b_trade_volume = quote_of_base_amount(trade_size, b_order.price);
    // delta trade volume
    let d_trade_volume = b_trade_volume - s_trade_volume;
    // s_order.matcher_fee part for trade_size <= s_order.amount
    let s_order_matcher_fee = s_order.matcher_fee_of_amount(trade_size);
    // b_order.matcher_fee part for trade_size <= b_order.amount
    let b_order_matcher_fee = b_order.matcher_fee_of_amount(trade_size);
    // s_order.protocol_fee part for trade_size <= b_order.amount with maker/taker condition
    let s_order_protocol_fee = s_order.protocol_fee_of_amount(b_order, s_trade_volume);
    let b_order_protocol_fee = b_order.protocol_fee_of_amount(s_order, s_trade_volume);

    // seller - buyer deal
    if s_order.owner == b_order.owner {
        let mut account = storage.account.get(s_order.owner).read();
        // unlock locked base asset
        account.unlock_amount(trade_size, asset_type);
        // unlock locked quote asset
        // if b_price > s_price unlock extra funds and its extra protocol fee
        account.unlock_amount(
            b_trade_volume + b_order
                .max_protocol_fee_of_amount(d_trade_volume) - s_order_protocol_fee - s_order_matcher_fee,
            !asset_type,
        );
        storage.account.insert(s_order.owner, account);
    } else {
        let mut s_account = storage.account.get(s_order.owner).read();
        let mut b_account = storage.account.get(b_order.owner).read();
        // exchange trade funds
        s_account.transfer_locked_amount(b_account, trade_size, asset_type);
        b_account.transfer_locked_amount(s_account, s_trade_volume, !asset_type);
        // lock protocol and matcher_fee for seller
        let lock_fee = s_order_protocol_fee + s_order_matcher_fee;
        if lock_fee > 0 {
            s_account.lock_amount(s_order_protocol_fee + s_order_matcher_fee, !asset_type);
        }
        let unlock_fee = d_trade_volume + b_order.max_protocol_fee_of_amount(b_trade_volume) - b_order_protocol_fee;
        if unlock_fee > 0 {
            b_account.unlock_amount(unlock_fee, !asset_type);
        }

        // store accounts
        storage.account.insert(s_order.owner, s_account);
        storage.account.insert(b_order.owner, b_account);
    }

    // seller - matcher deal
    if (s_order_matcher_fee > 0) {
        if s_order.owner == matcher {
            let mut account = storage.account.get(s_order.owner).read();
            account.unlock_amount(s_order_matcher_fee, !asset_type);
            storage.account.insert(s_order.owner, account);
        } else {
            let mut s_account = storage.account.get(s_order.owner).read();
            let mut m_account = storage.account.get(matcher).try_read().unwrap_or(Account::new());
            s_account.transfer_locked_amount(m_account, s_order_matcher_fee, !asset_type);
            storage.account.insert(s_order.owner, s_account);
            storage.account.insert(matcher, m_account);
        }
    }

    // buyer - matcher deal
    if (b_order_matcher_fee > 0) {
        if b_order.owner == matcher {
            let mut account = storage.account.get(b_order.owner).read();
            account.unlock_amount(b_order_matcher_fee, !asset_type);
            storage.account.insert(b_order.owner, account);
        } else {
            let mut b_account = storage.account.get(b_order.owner).read();
            let mut m_account = storage.account.get(matcher).try_read().unwrap_or(Account::new());
            b_account.transfer_locked_amount(m_account, b_order_matcher_fee, !asset_type);
            storage.account.insert(b_order.owner, b_account);
            storage.account.insert(matcher, m_account);
        }
    }

    // seller - owner deal
    if (s_order_protocol_fee > 0) {
        if s_order.owner == OWNER {
            let mut account = storage.account.get(s_order.owner).read();
            account.unlock_amount(s_order_protocol_fee, !asset_type);
            storage.account.insert(s_order.owner, account);
        } else {
            let mut s_account = storage.account.get(s_order.owner).read();
            let mut o_account = storage.account.get(OWNER).try_read().unwrap_or(Account::new());
            s_account.transfer_locked_amount(o_account, s_order_protocol_fee, !asset_type);
            storage.account.insert(s_order.owner, s_account);
            storage.account.insert(OWNER, o_account);
        }
    }

    // buyer - owner deal
    if (b_order_protocol_fee > 0) {
        if b_order.owner == OWNER {
            let mut account = storage.account.get(b_order.owner).read();
            account.unlock_amount(b_order_protocol_fee, !asset_type);
            storage.account.insert(b_order.owner, account);
        } else {
            let mut b_account = storage.account.get(b_order.owner).read();
            let mut o_account = storage.account.get(OWNER).try_read().unwrap_or(Account::new());
            b_account.transfer_locked_amount(o_account, b_order_protocol_fee, !asset_type);
            storage.account.insert(b_order.owner, b_account);
            storage.account.insert(OWNER, o_account);
        }
    }
    (s_trade_volume, s_order_matcher_fee, b_order_matcher_fee)
}

#[storage(read, write)]
fn match_order_internal(
    order0_id: b256,
    order0: Order,
    order0_limit: LimitType,
    order1_id: b256,
    order1: Order,
    order1_limit: LimitType,
) -> (MatchResult, b256) {
    let matcher = msg_sender().unwrap();

    require(
        order0
            .asset_type == AssetType::Base && order1
            .asset_type == AssetType::Base,
        AssetError::InvalidAsset,
    );

    // The same order direction
    if order0.order_type == order1.order_type {
        return (MatchResult::ZeroMatch, b256::zero());
    }

    let (mut s_order, s_id, s_limit, mut b_order, b_id, b_limit) = if order0.order_type == OrderType::Sell {
        (order0, order0_id, order0_limit, order1, order1_id, order1_limit)
    } else {
        (order1, order1_id, order1_limit, order0, order0_id, order0_limit)
    };

    // Checking if the prices align for a possible match
    if s_order.price > b_order.price {
        // No match possible due to price mismatch
        return (MatchResult::ZeroMatch, b256::zero());
    }

    let trade_price = s_order.price;
    // Determine trade amounts based on the minimum available
    let trade_size = min(s_order.amount, b_order.amount);

    // Emit events for a matched order scenario
    emit_match_events(
        s_id,
        s_order,
        s_limit,
        trade_size,
        b_id,
        b_order,
        b_limit,
        trade_size,
        matcher,
        trade_price,
    );

    // Execute the trade and update balances
    let (trade_volume, s_order_matcher_fee, b_order_matcher_fee) = execute_trade(s_order, b_order, trade_size, matcher);

    increase_user_volume(s_order.owner, trade_volume);
    increase_user_volume(b_order.owner, trade_volume);

    // Handle partial or full order fulfillment
    update_order_storage(
        trade_size,
        s_order,
        s_id,
        s_order_matcher_fee,
        b_order,
        b_id,
        b_order_matcher_fee,
    )
}

#[storage(read, write)]
fn update_order_storage(
    amount: u64,
    ref mut order0: Order,
    id0: b256,
    order_matcher_fee0: u64,
    ref mut order1: Order,
    id1: b256,
    order_matcher_fee1: u64,
) -> (MatchResult, b256) {
    // Case where the first order is completely filled
    if amount == order0.amount {
        remove_order(order0.owner, id0);
    }
    // Case where the second order is completely filled
    if amount == order1.amount {
        remove_order(order1.owner, id1);
    }
    if amount != order0.amount {
        // Case where the first order is partially filled
        order0.matcher_fee -= order_matcher_fee0;
        order0.amount -= amount;
        storage.orders.insert(id0, order0);
        return (MatchResult::PartialMatch, id0);
    } else if amount != order1.amount {
        // Case where the second order is partially filled
        order1.matcher_fee -= order_matcher_fee1;
        order1.amount -= amount;
        storage.orders.insert(id1, order1);
        return (MatchResult::PartialMatch, id1);
    }
    // Case where both orders are fully matched
    (MatchResult::FullMatch, b256::zero())
}

#[storage(read, write)]
fn emit_match_events(
    id0: b256,
    order0: Order,
    limit0: LimitType,
    amount0: u64,
    id1: b256,
    order1: Order,
    limit1: LimitType,
    amount1: u64,
    matcher: Identity,
    match_price: u64,
) {
    // Emit events for the first order
    log_order_change_info(
        id0,
        OrderChangeInfo::new(
            OrderChangeType::OrderMatched,
            block_height(),
            matcher,
            tx_id(),
            order0
                .amount,
            order0
                .amount - amount0,
        ),
    );
    log(MatchOrderEvent {
        order_id: id0,
        asset: get_asset_id(order0.asset_type),
        order_matcher: matcher,
        owner: order0.owner,
        counterparty: order1.owner,
        match_size: amount0,
        match_price: match_price,
    });

    // Emit events for the second order
    log_order_change_info(
        id1,
        OrderChangeInfo::new(
            OrderChangeType::OrderMatched,
            block_height(),
            matcher,
            tx_id(),
            order1
                .amount,
            order1
                .amount - amount1,
        ),
    );
    log(MatchOrderEvent {
        order_id: id1,
        asset: get_asset_id(order1.asset_type),
        order_matcher: matcher,
        owner: order1.owner,
        counterparty: order0.owner,
        match_size: amount1,
        match_price: match_price,
    });

    // Emit event for the trade execution
    log(TradeOrderEvent {
        base_sell_order_id: id0,
        base_buy_order_id: id1,
        base_sell_order_limit: limit0,
        base_buy_order_limit: limit1,
        order_matcher: matcher,
        trade_size: amount0,
        trade_price: match_price,
        block_height: block_height(),
        tx_id: tx_id(),
        order_seller: order0.owner,
        order_buyer: order1.owner,
    });
}

#[storage(read, write)]
fn log_order_change_info(order_id: b256, change_info: OrderChangeInfo) {
    storage.order_change_info.get(order_id).push(change_info);
}

fn quote_of_base_amount(amount: u64, price: u64) -> u64 {
    convert_asset_amount(amount, price, true)
}

fn base_of_quote_amount(amount: u64, price: u64) -> u64 {
    convert_asset_amount(amount, price, false)
}

fn convert_asset_amount(amount: u64, price: u64, base_to_quote: bool) -> u64 {
    let (op1, op2) = (price, 10_u64.pow(BASE_ASSET_DECIMALS + PRICE_DECIMALS - QUOTE_ASSET_DECIMALS));
    if base_to_quote {
        amount.mul_div(op1, op2)
    } else {
        amount.mul_div(op2, op1)
    }
}

pub fn lock_order_amount(order: Order) -> u64 {
    // For asset_type base only
    if order.order_type == OrderType::Buy {
        let amount = quote_of_base_amount(order.amount, order.price);
        amount + order.max_protocol_fee_of_amount(amount) + order.matcher_fee
    } else {
        order.amount
    }
}

fn only_owner() {
    require(msg_sender().unwrap() == OWNER, AuthError::Unauthorized);
}
