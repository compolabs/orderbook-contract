contract;

mod errors;
mod math;
mod data_structures;
mod events;
mod interface;

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
    protocol_fee::*,
    user_volume::UserVolume,
};
use ::errors::{AccountError, AssetError, AuthError, MatchError, MathError, OrderError, ValueError};
use ::events::{
    CancelOrderEvent,
    DepositEvent,
    OpenOrderEvent,
    SetEpochEvent,
    SetMatcherRewardEvent,
    SetMinOrderPriceEvent,
    SetMinOrderSizeEvent,
    SetProtocolFeeEvent,
    SetStoreOrderChangeInfoEvent,
    TradeOrderEvent,
    WithdrawEvent,
    WithdrawToMarketEvent,
};
use ::interface::{SparkMarket, SparkMarketInfo};
use ::math::{distance, HUNDRED_PERCENT, lts, min};

use std::{
    asset::transfer,
    block::height as block_height,
    block::timestamp as block_timestamp,
    call_frames::msg_asset_id,
    context::msg_amount,
    error_signals::FAILED_REQUIRE_SIGNAL,
    hash::Hash,
    storage::storage_vec::*,
    tx::tx_id,
};

use sway_libs::{
    ownership::{
        _owner as ownership_owner,
        initialize_ownership as ownership_initialize_ownership,
        only_owner as ownership_only_owner,
        transfer_ownership as ownership_transfer_ownership,
    },
    pausable::{
        Pausable,
        _pause,
        _unpause,
        _is_paused,
        require_not_paused,
        require_paused,
    },
    reentrancy::reentrancy_guard,
};
use standards::src5::{AccessError, SRC5, State};

const ZERO_VALUE = 0;
const TRUE_VALUE = true;
// 1 month (86400 * 365.25 / 12)
const ONE_MONTH_SECONDS = 2629800;

configurable {
    BASE_ASSET: AssetId = AssetId::zero(),
    BASE_ASSET_DECIMALS: u32 = 9,
    QUOTE_ASSET: AssetId = AssetId::zero(),
    QUOTE_ASSET_DECIMALS: u32 = 9,
    PRICE_DECIMALS: u32 = 9,
    VERSION: u32 = 0,
}

storage {
    /// The reward to the matcher for single order match.
    matcher_fee: u64 = ZERO_VALUE,
    /// Epoch.
    epoch: u64 = ZERO_VALUE,
    /// Epoch duration.
    epoch_duration: u64 = ONE_MONTH_SECONDS,
    /// Minimum  order size in BASE_ASSET units
    min_order_size: u64 = ZERO_VALUE,
    /// Minimum  order size in BASE_ASSET units
    min_order_price: u64 = ZERO_VALUE,
    /// Disable storing an order change info.
    store_order_change_info: bool = TRUE_VALUE,
    /// Balance of each user.
    account: StorageMap<Identity, Account> = StorageMap {},
    /// All of the currently open orders.
    orders: StorageMap<b256, Order> = StorageMap {},
    /// Internal handling of indexes for user_orders.
    user_order_indexes: StorageMap<Identity, StorageMap<b256, u64>> = StorageMap {},
    /// Indexing orders by user.
    user_orders: StorageMap<Identity, StorageVec<b256>> = StorageMap {},
    /// Temporary order change log structure for indexer debug.
    order_change_info: StorageMap<b256, StorageVec<OrderChangeInfo>> = StorageMap {},
    /// Protocol fee.
    protocol_fee: StorageVec<ProtocolFee> = StorageVec {},
    /// User trade volumes.
    user_volumes: StorageMap<Identity, UserVolume> = StorageMap {},
    /// Order height.
    order_heights: StorageMap<Identity, u64> = StorageMap {},
}

impl SRC5 for Contract {
    /// Returns the owner.
    ///
    /// # Returns
    ///
    /// * [State] - Represents the state of ownership for this contract.
    #[storage(read)]
    fn owner() -> State {
        ownership_owner()
    }
}

impl Pausable for Contract {
    #[storage(write)]
    fn pause() {
        ownership_only_owner();
        require_not_paused();
        _pause();
    }

    #[storage(write)]
    fn unpause() {
        ownership_only_owner();
        require_paused();
        _unpause();
    }

    #[storage(read)]
    fn is_paused() -> bool {
        _is_paused()
    }
}

impl SparkMarket for Contract {
    #[storage(read, write)]
    fn initialize_ownership(new_owner: Identity) {
        ownership_initialize_ownership(new_owner);
    }

    #[storage(read, write)]
    fn transfer_ownership(new_owner: Identity) {
        ownership_transfer_ownership(new_owner);
    }

    /// Deposits a specified amount of an asset into the caller's account.
    ///
    /// ### Additional Information
    ///
    /// The function requires that the sender sends a non-zero amount of the specified asset.
    ///
    /// ### Reverts
    ///
    /// * When `msg_amount` == 0.
    /// * When `msg_asset` is neither BASE_ASSET nor QUOTE_ASSET.
    #[payable]
    #[storage(read, write)]
    fn deposit() {
        require_not_paused();
        reentrancy_guard();

        let user = msg_sender().unwrap();

        let (amount, asset, account) = deposit_internal(user);

        log(DepositEvent {
            amount,
            asset,
            user,
            account,
            caller: user,
        });
    }

    /// Deposits a specified amount of an asset into the user specified account.
    ///
    /// ### Additional Information
    ///
    /// The function requires that the sender sends a non-zero amount of the specified asset.
    ///
    /// ### Arguments
    ///
    /// * `user`: [Identity] - The deposit's account.
    ///
    /// ### Reverts
    ///
    /// * When `msg_amount` == 0.
    /// * When `msg_asset` is neither BASE_ASSET nor QUOTE_ASSET.
    #[payable]
    #[storage(read, write)]
    fn deposit_for(user: Identity) {
        require_not_paused();
        reentrancy_guard();

        let caller = msg_sender().unwrap();

        let (amount, asset, account) = deposit_internal(user);

        log(DepositEvent {
            amount,
            asset,
            user,
            account,
            caller,
        });
    }

    /// Withdraws a specified amount of a given asset from the caller's account.
    ///
    /// ### Arguments
    ///
    /// * `amount`: [u64] - The amount of the asset to be withdrawn. Must be greater than zero.
    /// * `asset_type`: [AssetType] - The type of the asset to be withdrawn.
    ///
    /// ### Reverts
    ///
    /// * When `amount` == 0 or `amount` exeeds user unlocked asset amount.
    /// * When `asset_type` is neither BASE_ASSET nor QUOTE_ASSET.
    #[storage(read, write)]
    fn withdraw(amount: u64, asset_type: AssetType) {
        reentrancy_guard();
        let (asset, user, account) = withdraw_internal(amount, asset_type);

        transfer(user, asset, amount);

        log(WithdrawEvent {
            amount,
            asset,
            user,
            account,
        });
    }

    /// Withdraws a specified amount of a given asset from the caller's account.
    ///
    /// ### Additional Information
    ///
    /// Then deposits amount to the another market for caller's account.
    ///
    /// ### Arguments
    ///
    /// * `amount`: [u64] - The amount of the asset to be withdrawn. Must be greater than zero.
    /// * `asset_type`: [AssetType] - The type of the asset to be withdrawn.
    /// * `market`: [ContractId] - The market ContractId.
    ///
    /// ### Reverts
    ///
    /// * When `amount` == 0 or `amount` exeeds user unlocked asset amount.
    /// * When `asset_type` is neither BASE_ASSET nor QUOTE_ASSET.
    /// * When asset_id of `asset_type` is not present in `market` as base or quote asset.
    #[storage(read, write)]
    fn withdraw_to_market(amount: u64, asset_type: AssetType, market: ContractId) {
        reentrancy_guard();

        require(market != ContractId::this(), ValueError::InvalidMarketSame);

        let (asset, user, account) = withdraw_internal(amount, asset_type);
        let (base, _, quote, _, _, _, _) = abi(SparkMarketInfo, market.into()).config();
        require(
            asset == base || asset == quote,
            AssetError::InvalidMarketAsset,
        );

        abi(SparkMarket, market
            .into())
            .deposit_for {
                asset_id: asset.into(),
                coins: amount,
            }(user);

        log(WithdrawToMarketEvent {
            amount,
            asset,
            user,
            account,
            market,
        });
    }

    /// Opens a new order with a specified amount, order type, and price.
    ///
    /// ### Arguments
    ///
    /// * `amount`: [u64] - The amount of the asset to be used in the order.
    /// * `order_type`: [OrderType] - The type of the order being created (e.g., buy or sell).
    /// * `price`: [u64] - The price at which the order should be placed.
    ///
    /// ### Returns
    ///
    /// * [b256] - The unique identifier of the newly opened order.
    ///
    /// ### Reverts
    ///
    /// * When `amount` == 0 or `amount` exeeds user unlocked asset amount.
    /// * When `price` == 0.
    #[storage(read, write)]
    fn open_order(amount: u64, order_type: OrderType, price: u64) -> b256 {
        require_not_paused();
        reentrancy_guard();

        open_order_internal(amount, order_type, price, read_matcher_fee())
    }

    /// Cancels an existing order with the specified order ID.
    ///
    /// ### Arguments
    ///
    /// * `order_id`: [b256] - The unique identifier of the order to be canceled.
    ///
    /// ### Reverts
    ///
    /// * When an order with `order_id` doesn't exist in the storage (not opened/matched/cancelled).
    /// * When a caller is not an owner of the order.
    #[storage(read, write)]
    fn cancel_order(order_id: b256) {
        reentrancy_guard();

        cancel_order_internal(order_id);
    }

    /// Matches two orders identified by their respective order IDs.
    ///
    /// ### Arguments
    ///
    /// * `order0_id`: [b256] - The unique identifier of the first order to be matched.
    /// * `order1_id`: [b256] - The unique identifier of the second order to be matched.
    ///
    /// ### Reverts
    ///
    /// * When orders with `order0_id` or `order1_id` not found.
    /// * When orders are in same direction ([sell, sell] or [buy, buy]).
    /// * When order buy price lower than order sell price.
    #[storage(read, write)]
    fn match_order_pair(order0_id: b256, order1_id: b256) {
        require_not_paused();
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

    /// Attempts to match multiple orders provided in a list.
    ///
    /// ### Arguments
    ///
    /// * `orders`: [Vec<b256>] - A vector containing the unique identifiers of the orders to be matched.
    ///
    /// ### Reverts
    ///
    /// * When order vector length is less than 2.
    /// * When no any orders can be matched.
    #[storage(read, write)]
    fn match_order_many(orders: Vec<b256>) {
        require_not_paused();
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
                // The order is already matched, canceled, or has an invalid ID
                idx0 += 1;
                continue;
            }

            let id1 = orders.get(idx1).unwrap();
            let order1 = storage.orders.get(id1).try_read();
            if order1.is_none() {
                // The order is already matched, canceled, or has an invalid ID
                idx1 += 1;
                continue;
            }

            // Attempt to match the orders
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
                    // This case occurs when both orders move in the same direction
                    if idx0 < idx1 { idx1 += 1; } else { idx0 += 1; }
                }
                MatchResult::PartialMatch => {
                    // This case occurs when one of the orders is partially filled
                    if partial_order_id == id0 {
                        idx1 += 1;
                    } else {
                        idx0 += 1;
                    }
                    full_matched += 1;
                }
                MatchResult::FullMatch => {
                    // This case occurs when both orders are fully filled
                    idx0 = min(idx0, idx1) + 1;
                    idx1 = idx0 + 1;
                    full_matched += 2;
                }
            }
        }
        require(full_matched > 0, MatchError::CantMatchMany);
    }

    /// Attempts to fulfill a single order by matching it against multiple orders from a provided list.
    ///
    /// ### Additional Information
    ///
    /// This function creates a new order with the given parameters and iterates through the list of existing orders,
    /// attempting to match the new order with existing orders. It handles full and partial matches according to the specified limit type:
    ///      - 'GTC' (Good-Til-Canceled): The order remains active until it is either fully filled or canceled.
    ///      - 'IOC' (Immediate-Or-Cancel): The order can be partially filled immediately, and any unfilled portion is canceled.
    ///      - 'FOK' (Fill-Or-Kill): The order must be fully filled immediately, or the entire transaction fails.
    ///
    /// ### Arguments
    ///
    /// * `amount`: [u64] - The amount of the asset to be fulfilled in the new order.
    /// * `order_type`: [OrderType] - The type of the order being fulfilled (e.g., buy or sell).
    /// * `limit_type`: [LimitType] - The limit type for the new order: 'GTC', 'IOC', or 'FOK'.
    /// * `price`: [u64] - The price at which the new order is to be fulfilled.
    /// * `slippage`: [u64] - The maximum allowable slippage (as a percentage) for the price during the matching process.
    /// * `orders`: [Vec<b256>] - A vector of order IDs representing the existing orders to match against the new order.
    ///
    /// ### Returns
    ///
    /// * [b256] - The unique identifier of the newly created order. If the order is partially matched and canceled (in the case of 'IOC'), the ID corresponds to the canceled order.
    ///
    /// ### Reverts
    ///
    /// * When order vector length is less than 1.
    /// * When no any orders can be fulfilled.
    #[storage(read, write)]
    fn fulfill_order_many(
        amount: u64,
        order_type: OrderType,
        limit_type: LimitType,
        price: u64,
        slippage: u64,
        orders: Vec<b256>,
    ) -> b256 {
        require_not_paused();
        reentrancy_guard();

        require(orders.len() > 0, ValueError::InvalidArrayLength);
        require(slippage <= HUNDRED_PERCENT, ValueError::InvalidSlippage);

        let id0 = open_order_internal(amount, order_type, price, 0);
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
                if (order_type == OrderType::Sell
                        && distance(price, order1.price) <= slippage)
                        || (order_type == OrderType::Buy
                            && distance(price, order1.price) <= slippage)
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

        if matched == MatchResult::PartialMatch
            && limit_type == LimitType::IOC
        {
            cancel_order_internal(id0);
        }

        id0
    }

    /// Sets the current epoch and its duration.
    ///
    /// ### Additional Information
    ///
    /// This function allows the contract owner to set a new epoch and its duration.
    /// It ensures that the new epoch is not in the past and that the epoch plus its duration extends beyond the current time.
    /// The function is restricted to the contract owner and logs an event after the epoch is set.
    ///
    /// ### Arguments
    ///
    /// * `epoch`: [u64] - The new epoch value to be set. Must be greater than or equal to the current epoch.
    /// * `epoch_duration`: [u64] - The duration of the epoch in seconds. The epoch plus its duration must extend beyond the current time.
    ///
    /// ### Reverts
    ///
    /// * When called by non-owner.
    /// * When epoch start less than current epoch start.
    /// * When epoch end less than current time.
    #[storage(write)]
    fn set_epoch(epoch: u64, epoch_duration: u64) {
        ownership_only_owner();

        let current_epoch = read_epoch();
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

    /// Sets the protocol fees based on volume thresholds.
    ///
    /// ### Additional Information
    ///
    /// This function allows the contract owner to set a list of protocol fees.
    /// It ensures that the first fee in the list has a volume threshold of zero and that the fees are sorted by volume threshold.
    /// The function is restricted to the contract owner and logs an event after the protocol fees are set.
    ///
    /// ### Arguments
    ///
    /// * `protocol_fee`: [Vec<ProtocolFee>] - A vector of 'ProtocolFee' structures that define the fee rates and their corresponding volume thresholds.
    ///    The first element must have a volume threshold of zero, and the list must be sorted by volume threshold.
    ///
    /// ### Reverts
    ///
    /// * When called by non-owner.
    /// * When `protocol_fee` vector length is zero.
    /// * When `protocol_fee` vector contains non-sorted volumes or volume duplicates.
    #[storage(write)]
    fn set_protocol_fee(protocol_fee: Vec<ProtocolFee>) {
        ownership_only_owner();

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
                .is_volume_threshold_valid(),
            ValueError::InvalidFeeSorting,
        );
        storage.protocol_fee.store_vec(protocol_fee);

        log(SetProtocolFeeEvent { protocol_fee });
    }

    /// Sets the matcher fee to a specified amount.
    ///
    /// ### Additional Information
    ///
    /// This function allows the contract owner to update the matcher fee.
    /// It checks that the new fee amount is different from the current one to avoid redundant updates.
    /// The function is restricted to the contract owner and logs an event after the matcher fee is set.
    ///
    /// ### Arguments
    ///
    /// * `amount`: [u64] The new matcher fee amount to be set. It must be different from the current matcher fee.
    ///
    /// ### Reverts
    ///
    /// * When called by non-owner.
    /// * When `matcher_fee` is same as set before.
    #[storage(read, write)]
    fn set_matcher_fee(amount: u64) {
        ownership_only_owner();
        require(amount != read_matcher_fee(), ValueError::InvalidValueSame);
        storage.matcher_fee.write(amount);

        log(SetMatcherRewardEvent { amount });
    }

    /// Sets storing change info flag.
    ///
    /// ### Additional Information
    ///
    /// This function allows the contract owner to enable or disable storing of order change info.
    ///
    /// ### Arguments
    ///
    /// * `store`: [bool] The new store boolean value.
    ///
    /// ### Reverts
    ///
    /// * When called by non-owner.
    /// * When `store` is same as set before.
    #[storage(read, write)]
    fn set_store_order_change_info(store: bool) {
        ownership_only_owner();
        require(
            store != read_store_order_change_info(),
            ValueError::InvalidValueSame,
        );
        storage.store_order_change_info.write(store);

        log(SetStoreOrderChangeInfoEvent { store });
    }

    /// Sets the minimum of order size.
    ///
    /// ### Additional Information
    ///
    /// This function allows the contract owner to update the minimum of an order amount.
    /// It checks that the new size is different from the current one to avoid redundant updates.
    /// The function is restricted to the contract owner.
    ///
    /// ### Arguments
    ///
    /// * `amount`: [u64] The new the minimum of an order amount to be set. It must be different from the current the minimum of an order amount.
    ///
    /// ### Reverts
    ///
    /// * When called by non-owner.
    /// * When `min_order_size` is same as set before.
    #[storage(read, write)]
    fn set_min_order_size(size: u64) {
        ownership_only_owner();
        require(size != read_min_order_size(), ValueError::InvalidValueSame);
        storage.min_order_size.write(size);

        log(SetMinOrderSizeEvent { size });
    }

    /// Sets the minimum of order price.
    ///
    /// ### Additional Information
    ///
    /// This function allows the contract owner to update the minimum of an order price.
    /// It checks that the new price is different from the current one to avoid redundant updates.
    /// The function is restricted to the contract owner.
    ///
    /// ### Arguments
    ///
    /// * `amount`: [u64] The new the minimum of an order amount to be set. It must be different from the current the minimum of an order amount.
    ///
    /// ### Reverts
    ///
    /// * When called by non-owner.
    /// * When `min_order_price` is same as set before.
    #[storage(read, write)]
    fn set_min_order_price(price: u64) {
        ownership_only_owner();
        require(
            price != read_min_order_price(),
            ValueError::InvalidValueSame,
        );
        storage.min_order_price.write(price);

        log(SetMinOrderPriceEvent { price });
    }
}

impl SparkMarketInfo for Contract {
    /// Get the user account information.
    ///
    /// ### Arguments
    ///
    /// * `user`: [Identity] The user id to retrive info.
    ///
    /// ### Returns
    ///
    /// * [Account] - An user account information.
    #[storage(read)]
    fn account(user: Identity) -> Account {
        storage.account.get(user).try_read().unwrap_or(Account::new())
    }

    /// Get the epoch start time and its duration.
    ///
    /// ### Returns
    ///
    /// * [u64, u64] - An epoch and duration.
    #[storage(read)]
    fn get_epoch() -> (u64, u64) {
        (read_epoch(), read_epoch_duration())
    }

    /// Get the matcher fee in `QUOTE_ASSET` units.
    ///
    /// ### Returns
    ///
    /// * [u64] - A matcher fee.
    #[storage(read)]
    fn matcher_fee() -> u64 {
        read_matcher_fee()
    }

    /// Get the protocol fee array.
    ///
    /// ### Returns
    ///
    /// * [Vec<ProtocolFee>] - A protocol fee vector.
    #[storage(read)]
    fn protocol_fee() -> Vec<ProtocolFee> {
        storage.protocol_fee.load_vec()
    }

    /// Get the user protocol fee of its current volume.
    ///
    /// ### Arguments
    ///
    /// * `user`: [Identity] The user id to retrive info.
    ///
    /// ### Returns
    ///
    /// * [(u64, u64)] - A maker and taker user fee percent (10_000 == 100%).
    #[storage(read)]
    fn protocol_fee_user(user: Identity) -> (u64, u64) {
        protocol_fee_user(user)
    }

    /// Get the user protocol fee of its current volume and of amount.
    ///
    /// ### Arguments
    ///
    /// * `amount`: [u64] The amount of the order in `QUOTE_ASSET` units.
    /// * `user`: [Identity] The user id to retrive info.
    ///
    /// ### Returns
    ///
    /// * [(u64, u64)] - A maker and taket user fee amount of `amount`.
    #[storage(read)]
    fn protocol_fee_user_amount(amount: u64, user: Identity) -> (u64, u64) {
        protocol_fee_user_amount(amount, user)
    }

    /// Get the order info.
    ///
    /// ### Arguments
    ///
    /// * `order`: [b256] The order_id.
    ///
    /// ### Returns
    ///
    /// * [Option<Order>] - The Some<Order> struct of found by id otherwise None.
    #[storage(read)]
    fn order(order: b256) -> Option<Order> {
        storage.orders.get(order).try_read()
    }

    /// Get user order list.
    ///
    /// ### Arguments
    ///
    /// * `user`: [Identity] The user identity.
    ///
    /// ### Returns
    ///
    /// * [Vec<b256>] - The vector of user order ids.
    #[storage(read)]
    fn user_orders(user: Identity) -> Vec<b256> {
        storage.user_orders.get(user).load_vec()
    }

    /// Get user order height.
    ///
    /// ### Arguments
    ///
    /// * `user`: [Identity] The user identity.
    ///
    /// ### Returns
    ///
    /// * [u64] - The user order height.
    #[storage(read)]
    fn user_order_height(user: Identity) -> u64 {
        storage.order_heights.get(user).try_read().unwrap_or(0)
    }

    /// Get order change list.
    ///
    /// ### Arguments
    ///
    /// * `order_id`: [b256] The order id.
    ///
    /// ### Returns
    ///
    /// * [Vec<OrderChangeInfo>] - The vector of order change info.
    #[storage(read)]
    fn order_change_info(order_id: b256) -> Vec<OrderChangeInfo> {
        storage.order_change_info.get(order_id).load_vec()
    }

    /// Get the minimum order size in BASE_ASSET units.
    ///
    /// ### Returns
    ///
    /// * [u64] - A minimum order size.
    #[storage(read)]
    fn min_order_size() -> u64 {
        read_min_order_size()
    }

    /// Get the minimum order price in QUOTE_ASSET whole coin * 10 ^ price_decimals.
    ///
    /// ### Returns
    ///
    /// * [u64] - A minimum order price.
    #[storage(read)]
    fn min_order_price() -> u64 {
        read_min_order_price()
    }

    /// Get contract configurables.
    ///
    /// ### Returns
    ///
    /// * [AssetId, u32, AssetId, u32, Option<Identity>, u32, u32)] - The BASE_ASSET, BASE_ASSET_DECIMALS,
    ///     QUOTE_ASSET, QUOTE_ASSET_DECIMALS, OWNER.owner(), PRICE_DECIMALS, VERSION.
    #[storage(read)]
    fn config() -> (AssetId, u32, AssetId, u32, Option<Identity>, u32, u32) {
        (
            BASE_ASSET,
            BASE_ASSET_DECIMALS,
            QUOTE_ASSET,
            QUOTE_ASSET_DECIMALS,
            ownership_owner().owner(),
            PRICE_DECIMALS,
            VERSION,
        )
    }

    /// Generate order id.
    ///
    /// ### Arguments
    ///
    /// * `order_type`: [OrderType] The order type.
    /// * `owner`: [Identity] The order owner.
    /// * `price`: [u64] The order price.
    /// * `block_height`: [u32] The order submission block number.
    /// * `order_height`: [u64] The order height (auto-incremented number).
    ///
    /// ### Returns
    ///
    /// * [b256] - The order id.
    fn order_id(
        order_type: OrderType,
        owner: Identity,
        price: u64,
        block_height: u32,
        order_height: u64,
    ) -> b256 {
        let asset_type = AssetType::Base;
        Order::new(
            1,
            asset_type,
            order_type,
            owner,
            price,
            block_height,
            order_height,
            0,
            0,
            0,
            0,
        ).id()
    }

    /// Get order change info flag.
    ///
    /// ### Returns
    ///
    /// * [bool] - The True if order change info stores otherwise false.
    #[storage(read)]
    fn store_order_change_info() -> bool {
        read_store_order_change_info()
    }
}

#[storage(read)]
fn read_order(order_id: b256) -> Order {
    let order = storage.orders.get(order_id).try_read();
    require(order.is_some(), OrderError::OrderNotFound(order_id));
    order.unwrap()
}

#[storage(read)]
fn read_matcher_fee() -> u64 {
    storage.matcher_fee.try_read().unwrap_or(ZERO_VALUE)
}

#[storage(read)]
fn read_epoch() -> u64 {
    storage.epoch.try_read().unwrap_or(ZERO_VALUE)
}

#[storage(read)]
fn read_epoch_duration() -> u64 {
    storage.epoch_duration.try_read().unwrap_or(ONE_MONTH_SECONDS)
}

#[storage(read)]
fn read_store_order_change_info() -> bool {
    storage.store_order_change_info.try_read().unwrap_or(TRUE_VALUE)
}

#[storage(read)]
fn read_min_order_size() -> u64 {
    storage.min_order_size.try_read().unwrap_or(ZERO_VALUE)
}

#[storage(read)]
fn read_min_order_price() -> u64 {
    storage.min_order_price.try_read().unwrap_or(ZERO_VALUE)
}

#[storage(read)]
fn owner_identity() -> Identity {
    match ownership_owner() {
        State::Initialized(identity) => identity,
        _ => Identity::Address(Address::zero()),
    }
}

fn get_asset_type(asset_id: AssetId) -> AssetType {
    if asset_id == BASE_ASSET {
        AssetType::Base
    } else if asset_id == QUOTE_ASSET {
        AssetType::Quote
    } else {
        log(AssetError::InvalidAsset);
        revert(FAILED_REQUIRE_SIGNAL);
    }
}
fn get_asset_id(asset_type: AssetType) -> AssetId {
    match asset_type {
        AssetType::Base => BASE_ASSET,
        AssetType::Quote => QUOTE_ASSET,
    }
}

fn quote_of_base_amount(amount: u64, price: u64) -> u64 {
    convert_asset_amount(amount, price, true)
}

fn convert_asset_amount(amount: u64, price: u64, base_to_quote: bool) -> u64 {
    let (op1, op2) = (price, 10_u64.pow(BASE_ASSET_DECIMALS + PRICE_DECIMALS - QUOTE_ASSET_DECIMALS));
    let mul_div = if base_to_quote {
        amount.mul_div(op1, op2)
    } else {
        amount.mul_div(op2, op1)
    };
    match mul_div {
        Ok(result) => result,
        Err(_) => {
            log(MathError::Overflow);
            revert(FAILED_REQUIRE_SIGNAL);
        }
    }
}

fn lock_order_amount(order: Order) -> u64 {
    // For asset_type base only
    if order.order_type == OrderType::Buy {
        let amount = quote_of_base_amount(order.amount, order.price);
        amount + order.max_protocol_fee_of_amount(amount) + order.matcher_fee
    } else {
        order.amount
    }
}

#[storage(read)]
fn protocol_fee_user(user: Identity) -> (u64, u64) {
    let volume = storage.user_volumes.get(user).try_read().unwrap_or(UserVolume::new()).get(read_epoch());
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

#[storage(write)]
fn extend_epoch_if_finished() {
    let epoch_duration = read_epoch_duration();
    let epoch = read_epoch() + epoch_duration;
    let timestamp = block_timestamp();

    if epoch <= timestamp {
        storage.epoch.write(timestamp);
        log(SetEpochEvent {
            epoch: timestamp,
            epoch_duration,
        });
    }
}

#[payable]
#[storage(read, write)]
fn deposit_internal(user: Identity) -> (u64, AssetId, Account) {
    let amount = msg_amount();
    require(amount > 0, ValueError::InvalidAmount);

    let asset = msg_asset_id();
    let asset_type = get_asset_type(asset);

    let mut account = storage.account.get(user).try_read().unwrap_or(Account::new());
    account.liquid.credit(amount, asset_type);
    storage.account.insert(user, account);
    (amount, asset, account)
}

#[storage(read, write)]
fn withdraw_internal(amount: u64, asset_type: AssetType) -> (AssetId, Identity, Account) {
    require(amount > 0, ValueError::InvalidAmount);

    let user = msg_sender().unwrap();
    let mut account = storage.account.get(user).try_read().unwrap_or(Account::new());

    account.liquid.debit(amount, asset_type);
    storage.account.insert(user, account);

    let asset = get_asset_id(asset_type);
    (asset, user, account)
}

#[storage(read, write)]
fn next_order_height(user: Identity) -> u64 {
    let order_height = storage.order_heights.get(user).try_read().unwrap_or(0);
    storage.order_heights.insert(user, order_height + 1);
    order_height
}

#[storage(read, write)]
fn open_order_internal(
    amount: u64,
    order_type: OrderType,
    price: u64,
    matcher_fee: u64,
) -> b256 {
    let user = msg_sender().unwrap();
    let (protocol_maker_fee, protocol_taker_fee) = protocol_fee_user(user);

    require(
        valid_order_amount(amount),
        OrderError::OrderSizeTooSmall(amount),
    );

    let asset_type = AssetType::Base;
    let mut order = Order::new(
        amount,
        asset_type,
        order_type,
        user,
        price,
        block_height(),
        next_order_height(user),
        matcher_fee,
        protocol_maker_fee,
        protocol_taker_fee,
        read_min_order_price(),
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

    store_order_change_info(
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
        balance: account,
    });
    order_id
}

#[storage(read, write)]
fn cancel_order_internal(order_id: b256) {
    // Order must exist to be cancelled
    let order = read_order(order_id);
    let user = msg_sender().unwrap();

    // Only the owner of the order may cancel their order
    require(user == order.owner, AuthError::Unauthorized);

    cancel_read_order(order_id, order);
}

#[storage(read, write)]
fn cancel_read_order(order_id: b256, order: Order) {
    let user = order.owner;
    // Safe to read() because user is the owner of the order
    let mut account = storage.account.get(user).read();

    // Order is about to be cancelled, unlock illiquid funds
    account.unlock_amount(
        lock_order_amount(order),
        match order.order_type {
            OrderType::Sell => order.asset_type,
            OrderType::Buy => !order.asset_type,
        },
    );

    remove_order(user, order_id);
    storage.account.insert(user, account);

    store_order_change_info(
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

    log(CancelOrderEvent {
        order_id,
        user,
        balance: account,
    });
}

#[storage(read, write)]
fn cancel_if_small_order(order_id: b256) -> bool {
    let order = read_order(order_id);
    let to_cancel = !valid_order_amount(order.amount);
    if to_cancel {
        cancel_read_order(order_id, order);
    }
    to_cancel
}

#[storage(read)]
fn valid_order_amount(amount: u64) -> bool {
    amount >= read_min_order_size()
}

#[storage(read, write)]
fn increase_user_volume(user: Identity, volume: u64) {
    extend_epoch_if_finished();
    storage
        .user_volumes
        .insert(
            user,
            storage
                .user_volumes
                .get(user)
                .try_read()
                .unwrap_or(UserVolume::new())
                .update(read_epoch(), volume),
        );
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
    );
    if order_count == 1 {
        // There is only one element, so no need to swap. Pop it from the end.
        require(
            storage
                .user_orders
                .get(user)
                .pop()
                .unwrap() == order_id,
            OrderError::FailedToRemove(order_id),
        );
    } else {
        // The order ID at the end will have its index updated via swap_remove().
        let last_element = storage.user_orders.get(user).last().unwrap().read();

        // Remove the current order by replacing it with the order at the end of the storage vector.
        require(
            storage
                .user_orders
                .get(user)
                .swap_remove(index) == order_id,
            OrderError::FailedToRemove(order_id),
        );

        // The last element has been moved, so update its index.
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
    // The volume of the trade for the seller
    let s_trade_volume = quote_of_base_amount(trade_size, s_order.price);
    // The volume of the trade reserved by the buyer for the trade size
    let b_trade_volume = quote_of_base_amount(trade_size, b_order.price);
    // The difference in trade volumes between the buyer and seller
    let d_trade_volume = b_trade_volume - s_trade_volume;
    // The matcher's fee for the seller's order based on the trade size (<= s_order.amount)
    let s_order_matcher_fee = s_order.matcher_fee_of_amount(trade_size);
    // The matcher's fee for the buyer's order based on the trade size (<= b_order.amount)
    let b_order_matcher_fee = b_order.matcher_fee_of_amount(trade_size);
    // The protocol fee for the seller's order based on the trade size and maker/taker conditions
    let s_order_protocol_fee = s_order.protocol_fee_of_amount(b_order, s_trade_volume);
    let b_order_protocol_fee = b_order.protocol_fee_of_amount(s_order, s_trade_volume);

    // The seller and buyer are the same entity (same owner)
    if s_order.owner == b_order.owner {
        let mut account = storage.account.get(s_order.owner).read();
        // Unlock the locked base asset
        account.unlock_amount(trade_size, asset_type);
        // Unlock the locked quote asset
        // If the buyer's price is greater than the seller's price, unlock extra funds and their protocol fees
        account.unlock_amount(
            b_trade_volume + b_order
                .max_protocol_fee_of_amount(d_trade_volume) - s_order_protocol_fee - s_order_matcher_fee,
            !asset_type,
        );
        storage.account.insert(s_order.owner, account);
    } else {
        // The seller and buyer are different entities (different owners)
        let mut s_account = storage.account.get(s_order.owner).read();
        let mut b_account = storage.account.get(b_order.owner).read();
        // Exchange trade funds between the seller and buyer
        s_account.transfer_locked_amount(b_account, trade_size, asset_type);
        b_account.transfer_locked_amount(s_account, s_trade_volume, !asset_type);
        // Lock the protocol and matcher fees for the seller
        let lock_fee = s_order_protocol_fee + s_order_matcher_fee;
        if lock_fee > 0 {
            s_account.lock_amount(lock_fee, !asset_type);
        }
        // Unlock excess funds for the buyer
        let unlock_delta = d_trade_volume + b_order.max_protocol_fee_of_amount(b_trade_volume) - b_order_protocol_fee;
        if unlock_delta > 0 {
            b_account.unlock_amount(unlock_delta, !asset_type);
        }

        // Store the updated accounts
        storage.account.insert(s_order.owner, s_account);
        storage.account.insert(b_order.owner, b_account);
    }

    // Handle the matcher's fee related to the seller
    if s_order_matcher_fee > 0 {
        // If the seller is the matcher
        if s_order.owner == matcher {
            let mut account = storage.account.get(s_order.owner).read();
            account.unlock_amount(s_order_matcher_fee, !asset_type);
            storage.account.insert(s_order.owner, account);
        } else {
            // If the matcher is a different entity, transfer the matcher's fee from seller to matcher
            let mut s_account = storage.account.get(s_order.owner).read();
            let mut m_account = storage.account.get(matcher).try_read().unwrap_or(Account::new());
            s_account.transfer_locked_amount(m_account, s_order_matcher_fee, !asset_type);
            storage.account.insert(s_order.owner, s_account);
            storage.account.insert(matcher, m_account);
        }
    }

    // Handle the matcher's fee related to the buyer
    if b_order_matcher_fee > 0 {
        // If the buyer is the matcher
        if b_order.owner == matcher {
            let mut account = storage.account.get(b_order.owner).read();
            account.unlock_amount(b_order_matcher_fee, !asset_type);
            storage.account.insert(b_order.owner, account);
        } else {
            // If the matcher is a different entity, transfer the matcher's fee from buyer to matcher
            let mut b_account = storage.account.get(b_order.owner).read();
            let mut m_account = storage.account.get(matcher).try_read().unwrap_or(Account::new());
            b_account.transfer_locked_amount(m_account, b_order_matcher_fee, !asset_type);
            storage.account.insert(b_order.owner, b_account);
            storage.account.insert(matcher, m_account);
        }
    }

    let owner = owner_identity();

    // Handle the protocol fee related to the seller
    if s_order_protocol_fee > 0 {
        // If the seller is the protocol owner
        if s_order.owner == owner {
            let mut account = storage.account.get(s_order.owner).read();
            account.unlock_amount(s_order_protocol_fee, !asset_type);
            storage.account.insert(s_order.owner, account);
        } else {
            // If the protocol owner is a different entity, transfer the protocol fee from seller to protocol owner
            let mut s_account = storage.account.get(s_order.owner).read();
            let mut o_account = storage.account.get(owner).try_read().unwrap_or(Account::new());
            s_account.transfer_locked_amount(o_account, s_order_protocol_fee, !asset_type);
            storage.account.insert(s_order.owner, s_account);
            storage.account.insert(owner, o_account);
        }
    }

    // Handle the protocol fee related to the buyer
    if b_order_protocol_fee > 0 {
        // If the buyer is the protocol owner
        if b_order.owner == owner {
            let mut account = storage.account.get(b_order.owner).read();
            account.unlock_amount(b_order_protocol_fee, !asset_type);
            storage.account.insert(b_order.owner, account);
        } else {
            // If the protocol owner is a different entity, transfer the protocol fee from buyer to protocol owner
            let mut b_account = storage.account.get(b_order.owner).read();
            let mut o_account = storage.account.get(owner).try_read().unwrap_or(Account::new());
            b_account.transfer_locked_amount(o_account, b_order_protocol_fee, !asset_type);
            storage.account.insert(b_order.owner, b_account);
            storage.account.insert(owner, o_account);
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

    // Execute the trade and update balances
    let (trade_volume, s_order_matcher_fee, b_order_matcher_fee) = execute_trade(s_order, b_order, trade_size, matcher);

    increase_user_volume(s_order.owner, trade_volume);
    increase_user_volume(b_order.owner, trade_volume);

    let s_account = storage.account.get(s_order.owner).read();
    let b_account = storage.account.get(b_order.owner).read();

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
        s_account,
        b_account,
    );

    // Handle partial or full order fulfillment
    let (match_result, partial_order_id) = update_order_storage(
        trade_size,
        s_order,
        s_id,
        s_order_matcher_fee,
        b_order,
        b_id,
        b_order_matcher_fee,
    );
    if match_result == MatchResult::PartialMatch
        && cancel_if_small_order(partial_order_id)
    {
        return (MatchResult::FullMatch, b256::zero());
    }
    (match_result, partial_order_id)
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
    s_id: b256,
    s_order: Order,
    s_limit: LimitType,
    s_amount: u64,
    b_id: b256,
    b_order: Order,
    b_limit: LimitType,
    b_amount: u64,
    matcher: Identity,
    match_price: u64,
    s_account: Account,
    b_account: Account,
) {
    // Emit events for the first order
    store_order_change_info(
        s_id,
        OrderChangeInfo::new(
            OrderChangeType::OrderMatched,
            block_height(),
            matcher,
            tx_id(),
            s_order
                .amount,
            s_order
                .amount - s_amount,
        ),
    );

    // Emit events for the second order
    store_order_change_info(
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

    // Emit event for the trade execution
    log(TradeOrderEvent {
        base_sell_order_id: s_id,
        base_buy_order_id: b_id,
        base_sell_order_limit: s_limit,
        base_buy_order_limit: b_limit,
        order_matcher: matcher,
        trade_size: s_amount,
        trade_price: match_price,
        block_height: block_height(),
        tx_id: tx_id(),
        order_seller: s_order.owner,
        order_buyer: b_order.owner,
        s_balance: s_account,
        b_balance: b_account,
        seller_is_maker: s_order.is_maker(b_order),
    });
}

#[storage(read, write)]
fn store_order_change_info(order_id: b256, change_info: OrderChangeInfo) {
    if read_store_order_change_info() {
        storage.order_change_info.get(order_id).push(change_info);
    }
}
