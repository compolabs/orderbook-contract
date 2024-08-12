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
    limit_type::LimitType,
    match_result::MatchResult,
    order::Order,
    order_change::OrderChangeInfo,
    order_change::OrderChangeType,
    order_type::OrderType,
};
use ::errors::{AccountError, AssetError, AuthError, MatchError, OrderError, ValueError};
use ::events::{
    CancelOrderEvent,
    DepositEvent,
    MatchOrderEvent,
    OpenOrderEvent,
    SetMatcherRewardEvent,
    SetProtocolFeeEvent,
    TradeOrderEvent,
    WithdrawEvent,
    WithdrawProtocolFeeEvent,
};
use ::interface::{Market, MarketInfo};
use ::math::*;

use std::{
    asset::transfer,
    block::height as block_height,
    call_frames::msg_asset_id,
    constants::ZERO_B256,
    context::msg_amount,
    contract_id::ContractId,
    hash::{
        Hash,
        sha256,
    },
    storage::storage_vec::*,
    tx::tx_id,
};

configurable {
    BASE_ASSET: AssetId = AssetId::from(ZERO_B256),
    BASE_ASSET_DECIMALS: u32 = 9,
    OWNER: Address = Address::from(ZERO_B256),
    PRICE_DECIMALS: u32 = 9,
    QUOTE_ASSET: AssetId = AssetId::from(ZERO_B256),
    QUOTE_ASSET_DECIMALS: u32 = 9,
    FUEL_ASSET: AssetId = AssetId::from(ZERO_B256),
    ETH_BASE_PRICE: u64 = 189200000000,
    ETH_QUOTE_PRICE: u64 = 292300000,
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
    protocol_fee: u32 = 15, // 0.15%
    // The reward to the matcher for single order match
    matcher_fee: u32 = 0,
    // Total protocol fee to withdraw
    total_protocol_fee: u64 = 0,
}

const HUNDRED_PERCENT = 10_000;

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
    fn withdraw(amount: u64, asset_type: AssetType) {
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

    #[payable]
    #[storage(read, write)]
    fn open_order(
        amount: u64,
        order_type: OrderType,
        price: u64,
    ) -> b256 {
        let asset_type = AssetType::Base;
        open_order_internal(
            amount,
            asset_type,
            order_type,
            price,
            storage
                .matcher_fee
                .read()
                .as_u64(),
        )
    }

    #[storage(read, write)]
    fn cancel_order(order_id: b256) {
        cancel_order_internal(order_id);
    }

    #[storage(read, write)]
    fn match_order_pair(order0_id: b256, order1_id: b256) {
        let order0 = storage.orders.get(order0_id).try_read();
        require(order0.is_some(), OrderError::OrderNotFound(order0_id));
        let order1 = storage.orders.get(order1_id).try_read();
        require(order1.is_some(), OrderError::OrderNotFound(order1_id));
        let (match_result, _, matcher_reward) = match_order_internal(order0_id, order0.unwrap(), order1_id, order1.unwrap());
        require(
            match_result != MatchResult::ZeroMatch,
            MatchError::CantMatch((order0_id, order1_id)),
        );
        let matcher = msg_sender().unwrap();
        // reward order matcher
        if matcher_reward > 0 {
            transfer(matcher, FUEL_ASSET, matcher_reward);
        }
    }

    #[storage(read, write)]
    fn match_order_many(orders: Vec<b256>) {
        require(orders.len() >= 2, ValueError::InvalidArrayLength);

        let len = orders.len();
        let mut idx0 = 0;
        let mut idx1 = 1;
        let mut full_matched = 0;
        let mut matcher_reward = 0;

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
            let (match_result, partial_order_id, matcher_reward_single) = match_order_internal(id0, order0.unwrap(), id1, order1.unwrap());
            matcher_reward += matcher_reward_single;

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
        // reward order matcher
        let matcher = msg_sender().unwrap();
        if matcher_reward > 0 {
            transfer(matcher, FUEL_ASSET, matcher_reward);
        }
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
        require(orders.len() > 0, ValueError::InvalidArrayLength);
        require(slippage <= HUNDRED_PERCENT, ValueError::InvalidSlippage);

        let asset_type = AssetType::Base;
        let id0 = open_order_internal(amount, asset_type, order_type, price, 0);
        let len = orders.len();
        let mut idx1 = 0;
        let mut matched = MatchResult::ZeroMatch;
        let mut matcher_reward = 0;
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
                    let (match_result, partial_order_id, match_reward_single) = match_order_internal(id0, order0, id1, order1);
                    matcher_reward += match_reward_single;
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
            !(matched == MatchResult::ZeroMatch) && !(matched == MatchResult::PartialMatch && limit_type == LimitType::FOK),
            MatchError::CantFulfillMany,
        );
        // reward order matcher
        let matcher = msg_sender().unwrap();
        if matcher_reward > 0 {
            transfer(matcher, FUEL_ASSET, matcher_reward);
        }
        if matched == MatchResult::PartialMatch {
            cancel_order_internal(id0);
        }
        id0
    }

    #[storage(write)]
    fn set_protocol_fee(amount: u32) {
        require(
            msg_sender()
                .unwrap()
                .as_address()
                .unwrap() == OWNER,
            AuthError::Unauthorized,
        );

        storage.protocol_fee.write(amount);

        log(SetProtocolFeeEvent { amount });
    }

    #[storage(write)]
    fn set_matcher_fee(amount: u32) {
        require(
            msg_sender()
                .unwrap()
                .as_address()
                .unwrap() == OWNER,
            AuthError::Unauthorized,
        );

        storage.matcher_fee.write(amount);

        log(SetMatcherRewardEvent { amount });
    }

    #[storage(read, write)]
    fn withdraw_protocol_fee(to: Identity) {
        let owner = msg_sender().unwrap();
        require(
            owner
                .as_address()
                .unwrap() == OWNER,
            AuthError::Unauthorized,
        );

        let amount = storage.total_protocol_fee.read();
        require(amount > 0, AccountError::InsufficientBalance((0, 0)));
        storage.total_protocol_fee.write(0);
        transfer(to, FUEL_ASSET, amount);
        log(WithdrawProtocolFeeEvent {
            amount,
            to,
            owner,
        });
    }
}

impl MarketInfo for Contract {
    #[storage(read)]
    fn account(user: Identity) -> Option<Account> {
        storage.account.get(user).try_read()
    }

    #[storage(read)]
    fn protocol_fee() -> u32 {
        storage.protocol_fee.read()
    }

    #[storage(read)]
    fn protocol_fee_amount(amount: u64) -> u64 {
        protocol_fee_amount(amount, AssetType::Base)
    }

    #[storage(read)]
    fn total_protocol_fee() -> u64 {
        storage.total_protocol_fee.read()
    }

    #[storage(read)]
    fn matcher_fee() -> u32 {
        storage.matcher_fee.read()
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

    fn config() -> (Address, AssetId, u32, AssetId, u32, u32, AssetId) {
        (
            OWNER,
            BASE_ASSET,
            BASE_ASSET_DECIMALS,
            QUOTE_ASSET,
            QUOTE_ASSET_DECIMALS,
            PRICE_DECIMALS,
            FUEL_ASSET,
        )
    }

    fn order_id(
        order_type: OrderType,
        owner: Identity,
        price: u64,
        block_height: u32,
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
            0,
            0,
        ).id()
    }
}

#[storage(read)]
fn protocol_fee_amount(amount: u64, asset_type: AssetType) -> u64 {
    (if asset_type == AssetType::Base {
        ETH_BASE_PRICE
    } else {
        ETH_QUOTE_PRICE
    }) / HUNDRED_PERCENT * amount * storage.protocol_fee.read().as_u64() / 10_u64.pow(PRICE_DECIMALS)
}

#[payable]
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

    require(
        matcher_fee == 0 || msg_asset_id() == FUEL_ASSET,
        AssetError::InvalidFeeAsset,
    );

    let mut fee = msg_amount();
    let protocol_fee = protocol_fee_amount(amount, asset_type);
    // Require income fee
    require(
        fee >= matcher_fee + protocol_fee,
        ValueError::InvalidFeeAmount((fee, matcher_fee + protocol_fee)),
    );
    let mut order = Order::new(
        amount,
        asset_type,
        order_type,
        user,
        price,
        PRICE_DECIMALS,
        block_height(),
        matcher_fee
            .try_as_u32()
            .unwrap(),
        protocol_fee,
    );

    let order_id = order.id();
    let amount_before = match storage.orders.get(order_id).try_read() {
        Some(o) => o.amount,
        _ => 0,
    };
    fee -= matcher_fee + protocol_fee;
    if amount_before > 0 {
        // The order already exists in the same transaction
        order.amount += amount_before;
    } else {
        // Indexing
        storage.user_orders.get(user).push(order_id);
        storage
            .user_order_indexes
            .get(user)
            .insert(order_id, storage.user_orders.get(user).len() - 1);
    }

    // Refund extra income fee if any
    if fee > 0 {
        transfer(user, msg_asset_id(), fee);
    }

    // Store the new or updated order
    storage.orders.insert(order_id, order);

    // Update user account balance
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

    // Update the state of the user's account
    storage.account.insert(user, account);

    log_order_change_info(
        order_id,
        OrderChangeInfo::new(
            OrderChangeType::OrderOpened,
            block_height(),
            user,
            tx_id(),
            amount_before,
            order.amount,
        ),
    );
    // Add protocol fee to total
    storage
        .total_protocol_fee
        .write(storage.total_protocol_fee.read() + protocol_fee);

    let asset = get_asset_id(asset_type);
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

    // Refund matcher_fee
    if order.matcher_fee > 0 {
        transfer(user, FUEL_ASSET, order.matcher_fee.as_u64());
    }

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
fn execute_same_asset_type_trade(
    s_order: Order,
    b_order: Order,
    amount: u64,
    pay_amount: u64,
    price_delta: u64,
) {
    let mut s_account = storage.account.get(s_order.owner).try_read().unwrap_or(Account::new());
    let mut b_account = storage.account.get(b_order.owner).try_read().unwrap_or(Account::new());

    if s_order.owner == b_order.owner {
        b_account.unlock_amount(amount, s_order.asset_type);
        b_account.unlock_amount(pay_amount, !s_order.asset_type);
    } else {
        s_account.transfer_locked_amount(b_account, amount, s_order.asset_type);
        b_account.transfer_locked_amount(s_account, pay_amount, !s_order.asset_type);
    }

    if b_order.asset_type == AssetType::Base {
        // unlock delta if order_type == buy for asset_type == base
        let q_unlock_amount = convert(
            amount,
            BASE_ASSET_DECIMALS,
            price_delta,
            PRICE_DECIMALS,
            QUOTE_ASSET_DECIMALS,
            true,
        );
        if q_unlock_amount > 0 {
            b_account.unlock_amount(q_unlock_amount, AssetType::Quote);
        }
    }
    if s_order.owner == b_order.owner {
        storage.account.insert(b_order.owner, b_account);
    } else {
        storage.account.insert(s_order.owner, s_account);
        storage.account.insert(b_order.owner, b_account);
    }
}

#[storage(read, write)]
fn execute_same_order_type_trade(
    b_order: Order,
    q_order: Order,
    b_amount: u64,
    q_amount: u64,
    price_delta: u64,
) {
    let mut b_account = storage.account.get(b_order.owner).try_read().unwrap_or(Account::new());
    let mut q_account = storage.account.get(q_order.owner).try_read().unwrap_or(Account::new());
    if b_order.owner == q_order.owner {
        b_account.unlock_amount(b_amount, AssetType::Base);
        b_account.unlock_amount(q_amount, AssetType::Quote);
    }
    if b_order.order_type == OrderType::Sell {
        if (b_order.owner != q_order.owner) {
            b_account.transfer_locked_amount(q_account, b_amount, AssetType::Base);
            q_account.transfer_locked_amount(b_account, q_amount, AssetType::Quote);
        }
    } else {
        if b_order.owner != q_order.owner {
            q_account.transfer_locked_amount(b_account, b_amount, AssetType::Base);
            b_account.transfer_locked_amount(q_account, q_amount, AssetType::Quote);
        }
        // unlock delta if order_type == buy for asset_type == base
        if price_delta > 0 {
            let q_unlock_amount = convert(
                b_amount,
                BASE_ASSET_DECIMALS,
                price_delta,
                PRICE_DECIMALS,
                QUOTE_ASSET_DECIMALS,
                true,
            );
            if q_unlock_amount > 0 {
                b_account.unlock_amount(q_unlock_amount, AssetType::Quote);
            }
        }
    }
    if b_order.owner == q_order.owner {
        storage.account.insert(b_order.owner, b_account);
    } else {
        storage.account.insert(b_order.owner, b_account);
        storage.account.insert(q_order.owner, q_account);
    }
}

#[storage(read, write)]
fn match_order_internal(
    order0_id: b256,
    order0: Order,
    order1_id: b256,
    order1: Order,
) -> (MatchResult, b256, u64) {
    let matcher = msg_sender().unwrap();
    let mut matcher_reward = 0_u64;
    // the orders should have different directions
    if order0.order_type == order1.order_type && order0.asset_type != order1.asset_type {
        let (mut b_order, b_id, mut q_order, q_id) = if order0.asset_type == AssetType::Base {
            (order0, order0_id, order1, order1_id)
        } else {
            (order1, order1_id, order0, order0_id)
        };
        // check if base sell price <= base buy price
        if (b_order
                    .price > q_order
                    .price
                && b_order
                    .order_type == OrderType::Sell)
                || (b_order
                        .price < q_order
                        .price
                    && b_order
                        .order_type == OrderType::Buy)
        {
            return (MatchResult::ZeroMatch, ZERO_B256, 0);
        }
        let match_price = min(b_order.price, q_order.price);
        let price_delta = max(b_order.price, q_order.price) - match_price;
        // amount is a minimum of order amounts
        let b_amount = convert(
            q_order
                .amount,
            BASE_ASSET_DECIMALS,
            match_price,
            PRICE_DECIMALS,
            QUOTE_ASSET_DECIMALS,
            false,
        );
        // amount is a minimum of order amounts
        let b_amount = min(b_order.amount, b_amount);
        let q_amount = if b_amount != b_order.amount {
            q_order.amount
        } else {
            convert(
                b_amount,
                BASE_ASSET_DECIMALS,
                match_price,
                PRICE_DECIMALS,
                QUOTE_ASSET_DECIMALS,
                true,
            )
        };
        // emit match events
        log_order_change_info(
            b_id,
            OrderChangeInfo::new(
                OrderChangeType::OrderMatched,
                block_height(),
                matcher,
                tx_id(),
                b_order
                    .amount,
                b_order
                    .amount - b_amount,
            ),
        );
        log(MatchOrderEvent {
            order_id: b_id,
            asset: get_asset_id(AssetType::Base),
            order_matcher: matcher,
            owner: b_order.owner,
            counterparty: q_order.owner,
            match_size: b_amount,
            match_price: match_price,
        });
        log_order_change_info(
            q_id,
            OrderChangeInfo::new(
                OrderChangeType::OrderMatched,
                block_height(),
                matcher,
                tx_id(),
                q_order
                    .amount,
                q_order
                    .amount - q_amount,
            ),
        );
        log(MatchOrderEvent {
            order_id: q_id,
            asset: get_asset_id(AssetType::Quote),
            order_matcher: matcher,
            owner: q_order.owner,
            counterparty: b_order.owner,
            match_size: q_amount,
            match_price: match_price,
        });
        log(TradeOrderEvent {
            base_sell_order_id: b_id,
            base_buy_order_id: q_id,
            order_matcher: matcher,
            trade_size: b_amount,
            trade_price: match_price,
            block_height: block_height(),
            tx_id: tx_id(),
        });
        // update account balances
        execute_same_order_type_trade(b_order, q_order, b_amount, q_amount, price_delta);
        // update order storages
        if b_amount == b_order.amount {
            matcher_reward += b_order.matcher_fee.as_u64();
            remove_order(b_order.owner, b_id);
        }
        if q_amount == q_order.amount {
            matcher_reward += q_order.matcher_fee.as_u64();
            remove_order(q_order.owner, q_id);
        }
        if b_amount != b_order.amount {
            let order_matcher_reward = b_order.matcher_fee.as_u64() * b_amount / b_order.amount;
            matcher_reward += order_matcher_reward;
            b_order.matcher_fee -= order_matcher_reward.try_as_u32().unwrap();
            b_order.amount -= b_amount;
            // update amount
            storage.orders.insert(b_id, b_order);
            return (MatchResult::PartialMatch, b_id, matcher_reward);
        } else if q_amount != q_order.amount {
            let order_matcher_reward = q_order.matcher_fee.as_u64() * q_amount / q_order.amount;
            matcher_reward += order_matcher_reward;
            q_order.matcher_fee -= order_matcher_reward.try_as_u32().unwrap();
            q_order.amount -= q_amount;
            // update amount
            storage.orders.insert(q_id, q_order);
            return (MatchResult::PartialMatch, q_id, matcher_reward);
        }
        (MatchResult::FullMatch, ZERO_B256, matcher_reward)
    } else if order0.order_type != order1.order_type && order0.asset_type == order1.asset_type {
        let (mut s_order, s_id, mut b_order, b_id) = if order0.order_type == OrderType::Sell {
            (order0, order0_id, order1, order1_id)
        } else {
            (order1, order1_id, order0, order0_id)
        };
        // check if sell price <= buy price
        if s_order
                .price > b_order
                .price
            && order0
                .asset_type == AssetType::Base
                || s_order
                    .price < b_order
                    .price
                && order0
                    .asset_type == AssetType::Quote
        {
            return (MatchResult::ZeroMatch, ZERO_B256, 0);
        }
        let match_price = min(s_order.price, b_order.price);
        let price_delta = max(s_order.price, b_order.price) - match_price;
        // amount is a minimum of order amounts
        let amount = min(s_order.amount, b_order.amount);
        let pay_amount = convert(
            amount,
            BASE_ASSET_DECIMALS,
            match_price,
            PRICE_DECIMALS,
            QUOTE_ASSET_DECIMALS,
            s_order
                .asset_type == AssetType::Base,
        );
        // emit match events
        log_order_change_info(
            s_id,
            OrderChangeInfo::new(
                OrderChangeType::OrderMatched,
                block_height(),
                matcher,
                tx_id(),
                s_order
                    .amount,
                s_order
                    .amount - amount,
            ),
        );
        log(MatchOrderEvent {
            order_id: s_id,
            asset: get_asset_id(s_order.asset_type),
            order_matcher: matcher,
            owner: s_order.owner,
            counterparty: b_order.owner,
            match_size: amount,
            match_price: match_price,
        });
        log_order_change_info(
            b_id,
            OrderChangeInfo::new(
                OrderChangeType::OrderMatched,
                block_height(),
                matcher,
                tx_id(),
                b_order
                    .amount,
                b_order
                    .amount - amount,
            ),
        );
        log(MatchOrderEvent {
            order_id: b_id,
            asset: get_asset_id(b_order.asset_type),
            order_matcher: matcher,
            owner: b_order.owner,
            counterparty: s_order.owner,
            match_size: amount,
            match_price: match_price,
        });
        log(TradeOrderEvent {
            base_sell_order_id: if s_order.asset_type == AssetType::Base {
                s_id
            } else {
                b_id
            },
            base_buy_order_id: if s_order.asset_type == AssetType::Quote {
                s_id
            } else {
                b_id
            },
            order_matcher: matcher,
            trade_size: amount,
            trade_price: match_price,
            block_height: block_height(),
            tx_id: tx_id(),
        });
        // update account balances
        execute_same_asset_type_trade(s_order, b_order, amount, pay_amount, price_delta);
        // update order storages
        if amount == s_order.amount {
            matcher_reward += s_order.matcher_fee.as_u64();
            remove_order(s_order.owner, s_id);
        }
        if amount == b_order.amount {
            matcher_reward += b_order.matcher_fee.as_u64();
            remove_order(b_order.owner, b_id);
        }
        if amount < s_order.amount {
            let order_matcher_reward = s_order.matcher_fee.as_u64() * amount / s_order.amount;
            matcher_reward += order_matcher_reward;
            s_order.matcher_fee -= order_matcher_reward.try_as_u32().unwrap();
            s_order.amount -= amount;
            // update amount
            storage.orders.insert(s_id, s_order);
            return (MatchResult::PartialMatch, s_id, matcher_reward);
        }
        if amount < b_order.amount {
            let order_matcher_reward = b_order.matcher_fee.as_u64() * amount / b_order.amount;
            matcher_reward += order_matcher_reward;
            b_order.matcher_fee -= order_matcher_reward.try_as_u32().unwrap();
            b_order.amount -= amount;
            // update amount
            storage.orders.insert(b_id, b_order);
            return (MatchResult::PartialMatch, b_id, matcher_reward);
        }
        (MatchResult::FullMatch, ZERO_B256, matcher_reward)
    } else {
        (MatchResult::ZeroMatch, ZERO_B256, matcher_reward)
    }
}

#[storage(read, write)]
fn log_order_change_info(order_id: b256, change_info: OrderChangeInfo) {
    storage.order_change_info.get(order_id).push(change_info);
}
