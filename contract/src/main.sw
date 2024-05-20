contract;

mod errors;
mod events;
mod math;
mod structs;

use errors::*;
use events::*;
use i64::*;
use math::*;
use structs::*;

use sway_libs::reentrancy::reentrancy_guard;

use std::asset::transfer;
use std::block::timestamp;
use std::call_frames::msg_asset_id;
use std::constants::ZERO_B256;
use std::context::msg_amount;
use std::hash::*;
use std::storage::storage_vec::*;
use std::tx::tx_id;

configurable {
    QUOTE_TOKEN: AssetId = AssetId::from(ZERO_B256),
    QUOTE_TOKEN_DECIMALS: u32 = 6,
    PRICE_DECIMALS: u32 = 9,
}

storage {
    orders: StorageMap<b256, Order> = StorageMap {},
    markets: StorageMap<AssetId, Market> = StorageMap {},
    orders_by_trader: StorageMap<Address, StorageVec<b256>> = StorageMap {},
    order_indexes_by_trader: StorageMap<Address, StorageMap<b256, u64>> = StorageMap {},
    order_change_events: StorageMap<b256, StorageVec<OrderChangeEvent>> = StorageMap {},
}

abi OrderBook {
    #[storage(read, write)]
    fn create_market(asset_id: AssetId, decimal: u32);

    #[storage(read, write), payable]
    fn open_order(base_token: AssetId, base_size: I64, order_price: u64) -> b256;

    #[storage(read, write)]
    fn cancel_order(order_id: b256);

    #[storage(read, write)]
    fn match_orders(order_sell_id: b256, order_buy_id: b256);

    #[storage(read, write)]
    fn match_orders_many(order_sell_ids: Vec<b256>, order_buy_ids: Vec<b256>);

    #[storage(read)]
    fn orders_by_trader(trader: Address) -> Vec<b256>;

    #[storage(read)]
    fn order_by_id(order: b256) -> Option<Order>;

    #[storage(read)]
    fn market_exists(asset_id: AssetId) -> bool;

    #[storage(read)]
    fn get_market_by_id(asset_id: AssetId) -> Market;

    #[storage(read)]
    fn get_order_change_events_by_order(order: b256) -> Vec<OrderChangeEvent>;

    fn get_configurables() -> (AssetId, u32, u32);
}

impl OrderBook for Contract {
    #[storage(read, write)]
    fn create_market(asset_id: AssetId, asset_decimals: u32) {
        require(asset_id != QUOTE_TOKEN, Error::BadAsset);
        require(
            storage
                .markets
                .get(asset_id)
                .try_read()
                .is_none(),
            Error::MarketAlreadyExists,
        );
        let market = Market {
            asset_id,
            asset_decimals,
        };
        storage.markets.insert(asset_id, market);
        log(MarketCreateEvent {
            asset_id: asset_id,
            asset_decimals: asset_decimals,
            timestamp: timestamp(),
            tx_id: tx_id(),
        });
    }

    #[storage(read)]
    fn get_market_by_id(asset_id: AssetId) -> Market {
        storage.markets.get(asset_id).read()
    }

    #[storage(read)]
    fn market_exists(asset_id: AssetId) -> bool {
        !storage.markets.get(asset_id).try_read().is_none()
    }

    #[storage(read, write), payable]
    fn open_order(base_token: AssetId, base_size: I64, base_price: u64 /* decimal = 9 */ ) -> b256 {
        reentrancy_guard();

        let market = storage.markets.get(base_token).try_read();
        require(base_size.value != 0, Error::BaseSizeIsZero);
        require(market.is_some(), Error::NoMarketFound);
        require(base_price != 0, Error::BadPrice);

        let market = market.unwrap();
        if base_size.negative {
            require(msg_amount() == base_size.value, Error::BadValue);
            require(msg_asset_id() == base_token, Error::BadAsset);
        } else {
            require(
                msg_amount() == base_size_to_quote_amount(base_size.value, market.asset_decimals, base_price),
                Error::BadValue,
            );
            require(msg_asset_id() == QUOTE_TOKEN, Error::BadAsset);
        }

        let msg_sender = msg_sender_address();

        let order_id = gen_order_id(msg_sender, base_token, base_price);
        let order = storage.orders.get(order_id).try_read();

        if order.is_some() {
            let order = order.unwrap();
            let ((asset_id_0, refund_0), (asset_id_1, refund_1)) = update_order_internal(order, base_size);

            if refund_0 > 0 {
                transfer(Identity::Address(msg_sender), asset_id_0, refund_0);
            }
            if refund_1 > 0 {
                transfer(Identity::Address(msg_sender), asset_id_1, refund_1);
            }
        } else {
            let order = Order {
                id: order_id,
                trader: msg_sender,
                base_token,
                base_size,
                base_price,
            };
            add_order_internal(order);
        }

        let event = OrderChangeEvent::open(order_id, storage.orders.get(order_id).try_read());
        storage.order_change_events.get(order_id).push(event);
        log(event);

        order_id
    }

    #[storage(read, write)]
    fn cancel_order(order_id: b256) {
        reentrancy_guard();

        let order = storage.orders.get(order_id).try_read();
        require(order.is_some(), Error::NoOrdersFound);

        let order = order.unwrap();
        let msg_sender = msg_sender_address();
        require(msg_sender == order.trader, Error::AccessDenied);

        let (asset_id, refund) = cancel_order_internal(order);
        transfer(Identity::Address(msg_sender), asset_id, refund);

        let event = OrderChangeEvent::cancel(order_id, storage.orders.get(order_id).try_read());
        storage.order_change_events.get(order_id).push(event);
        log(event);
    }

    #[storage(read, write)]
    fn match_orders(order_sell_id: b256, order_buy_id: b256) {
        reentrancy_guard();
        match_orders(order_sell_id, order_buy_id);
    }

    #[storage(read, write)]
    fn match_orders_many(order_sell_ids: Vec<b256>, order_buy_ids: Vec<b256>) {
        let s_len = order_sell_ids.len();
        let b_len = order_buy_ids.len();
        require(s_len > 0 && b_len > 0, Error::OrdersCantBeMatched);
        reentrancy_guard();

        let mut s = 0;
        let mut b = 0;
        while true {
            let sid = order_sell_ids.get(s).unwrap();
            let bid = order_buy_ids.get(b).unwrap();
            match_orders(sid, bid);
            if storage.orders.get(sid).try_read().is_none() {
                s += 1;
                if s == s_len {
                    break;
                }
            }
            if storage.orders.get(bid).try_read().is_none() {
                b += 1;
                if b == b_len {
                    break;
                }
            }
        }
    }

    #[storage(read)]
    fn orders_by_trader(trader: Address) -> Vec<b256> {
        storage.orders_by_trader.get(trader).load_vec()
    }

    #[storage(read)]
    fn order_by_id(order: b256) -> Option<Order> {
        storage.orders.get(order).try_read()
    }

    #[storage(read)]
    fn get_order_change_events_by_order(order: b256) -> Vec<OrderChangeEvent> {
        storage.order_change_events.get(order).load_vec()
    }

    fn get_configurables() -> (AssetId, u32, u32) {
        (QUOTE_TOKEN, QUOTE_TOKEN_DECIMALS, PRICE_DECIMALS)
    }
}

#[storage(read, write)]
fn add_order_internal(order: Order) {
    storage.orders.insert(order.id, order);
    storage.orders_by_trader.get(order.trader).push(order.id);
    storage
        .order_indexes_by_trader
        .get(order.trader)
        .insert(order.id, storage.orders_by_trader.get(order.trader).len()); // pos + 1 indexed
}

#[storage(read, write)]
fn update_order_internal(order: Order, base_size: I64) -> ((AssetId, u64), (AssetId, u64)) {
    require(order.base_size.value != 0, Error::BaseSizeIsZero);
    let mut refund = ((AssetId::from(ZERO_B256), 0), (AssetId::from(ZERO_B256), 0));
    if order.base_size == base_size.flip() {
        let mut tmp = order;
        refund.0 = cancel_order_internal(order);
        tmp.base_size = tmp.base_size.flip();
        refund.1 = order_return_asset_amount(tmp);
    } else {
        if !order.base_size.is_same_sign(base_size) {
            let mut tmp = order;
            tmp.base_size.value = min(order.base_size.value, base_size.value);
            refund.0 = order_return_asset_amount(tmp);
            tmp.base_size = tmp.base_size.flip();
            refund.1 = order_return_asset_amount(tmp);
        }
        remove_update_order_internal(order, base_size);
    }
    refund
}

#[storage(read, write)]
fn cancel_order_internal(order: Order) -> (AssetId, u64) {
    require(order.base_size.value != 0, Error::BaseSizeIsZero);
    let refund = order_return_asset_amount(order);
    remove_update_order_internal(order, order.base_size.flip());
    refund
}

#[storage(read, write)]
fn remove_update_order_internal(order: Order, base_size: I64) {
    if (order.base_size == base_size.flip()) {
        let pos_id = storage.order_indexes_by_trader.get(order.trader).get(order.id).read() - 1; // pos + 1 indexed
        require(
            storage
                .order_indexes_by_trader
                .get(order.trader)
                .remove(order.id),
            Error::CannotRemoveOrderIndex,
        );
        let last_pos = storage.orders_by_trader.get(order.trader).len() - 1;
        if last_pos != pos_id {
            let last_id = storage.orders_by_trader.get(order.trader).get(last_pos).unwrap().read();
            require(
                storage
                    .orders_by_trader
                    .get(order.trader)
                    .swap_remove(pos_id) == order.id,
                Error::CannotRemoveOrderByTrader,
            );
            storage
                .order_indexes_by_trader
                .get(order.trader)
                .insert(last_id, pos_id + 1);
        } else {
            require(
                storage
                    .orders_by_trader
                    .get(order.trader)
                    .pop()
                    .unwrap() == order.id,
                Error::CannotRemoveOrderByTrader,
            );
        }
        require(storage.orders.remove(order.id), Error::CannotRemoveOrder);
    } else {
        let mut order = order;
        order.base_size += base_size;
        storage.orders.insert(order.id, order);
    }
}

#[storage(read, write)]
fn match_orders(order_sell_id: b256, order_buy_id: b256) {
    reentrancy_guard();

    let order_sell = storage.orders.get(order_sell_id).try_read();
    let order_buy = storage.orders.get(order_buy_id).try_read();
    require(
        order_sell
            .is_some() && order_buy
            .is_some(),
        Error::NoOrdersFound,
    );

    let order_sell = order_sell.unwrap();
    let order_buy = order_buy.unwrap();
    require(
        order_sell
            .base_size
            .negative && !order_buy
            .base_size
            .negative,
        Error::FirstArgumentShouldBeOrderSellSecondOrderBuy,
    );
    require(
        order_sell
            .base_token == order_buy
            .base_token && order_sell
            .base_price <= order_buy
            .base_price,
        Error::OrdersCantBeMatched,
    );

    let mut tmp = order_sell;
    tmp.base_size = tmp.base_size.flip();
    let trade_size = min(
        order_sell
            .base_size
            .value,
        order_buy
            .base_size
            .value
            .mul_div(order_buy.base_price, order_sell.base_price),
    );
    tmp.base_size.value = trade_size;

    let seller: Address = order_sell.trader;
    let (sellerDealAssetId, sellerDealRefund) = order_return_asset_amount(tmp);
    remove_update_order_internal(order_sell, tmp.base_size);

    tmp.base_size = tmp.base_size.flip();

    let buyer: Address = order_buy.trader;
    let (buyerDealAssetId, buyerDealRefund) = order_return_asset_amount(tmp);
    tmp.base_size.value = tmp.base_size.value.mul_div_rounding_up(order_sell.base_price, order_buy.base_price);
    remove_update_order_internal(order_buy, tmp.base_size);

    require(
        sellerDealRefund != 0 && buyerDealRefund != 0,
        Error::ZeroAssetAmountToSend,
    );

    transfer(
        Identity::Address(seller),
        sellerDealAssetId,
        sellerDealRefund,
    );
    transfer(Identity::Address(buyer), buyerDealAssetId, buyerDealRefund);

    let msg_sender = msg_sender_address();

    let event = OrderChangeEvent::match_orders(order_sell.id, storage.orders.get(order_sell.id).try_read());
    storage.order_change_events.get(order_sell.id).push(event);
    log(event);

    let event = OrderChangeEvent::match_orders(order_buy.id, storage.orders.get(order_buy.id).try_read());
    storage.order_change_events.get(order_buy.id).push(event);
    log(event);

    log(TradeEvent {
        base_token: order_sell.base_token,
        order_matcher: msg_sender,
        buyer: order_buy.trader,
        seller: order_sell.trader,
        trade_size: trade_size,
        trade_price: order_sell.base_price,
        sell_order_id: order_sell.id,
        buy_order_id: order_buy.id,
        timestamp: timestamp(),
        tx_id: tx_id(),
    });
}

#[storage(read)]
fn order_return_asset_amount(order: Order) -> (AssetId, u64) {
    return if order.base_size.negative {
        (order.base_token, order.base_size.value)
    } else {
        let market = storage.markets.get(order.base_token).try_read().unwrap();
        (
            QUOTE_TOKEN,
            base_size_to_quote_amount(
                order.base_size
                    .value,
                market
                    .asset_decimals,
                order.base_price,
            ),
        )
    }
}

fn base_size_to_quote_amount(base_size: u64, base_decimals: u32, base_price: u64) -> u64 {
    base_size.mul_div(
        base_price,
        10_u64
            .pow(base_decimals + PRICE_DECIMALS - QUOTE_TOKEN_DECIMALS),
    )
}

fn gen_order_id(
    trader_address: Address,
    base_token: AssetId,
    base_price: u64,
) -> b256 {
    sha256((trader_address, base_token, base_price))
}

pub fn msg_sender_address() -> Address {
    match std::auth::msg_sender().unwrap() {
        Identity::Address(identity) => identity,
        _ => revert(0),
    }
}
