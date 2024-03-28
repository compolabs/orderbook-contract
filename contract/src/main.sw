contract;

mod data_structures;
mod errors;
mod events;
mod interface;
mod math;
mod utils;

use ::data_structures::{Asset, Order};
use ::errors::{AssetError, Error, MarketError, OrderError};
use ::events::{CancelOrderEvent, CreateMarketEvent, OpenOrderEvent, TradeEvent, UpdateOrderEvent};
use ::interface::{Info, OrderBook};
use ::math::min;
use ::utils::trader;

use i64::I64;
use reentrancy::reentrancy_guard;
use std::{
    asset::transfer_to_address,
    call_frames::msg_asset_id,
    constants::BASE_ASSET_ID,
    context::msg_amount,
    hash::{
        Hash,
        sha256,
    },
    storage::storage_vec::*,
};

configurable {
    QUOTE_TOKEN: AssetId = BASE_ASSET_ID,
    QUOTE_TOKEN_DECIMALS: u32 = 9,
    PRICE_DECIMALS: u32 = 9,
}

storage {
    orders: StorageMap<b256, Order> = StorageMap {},
    markets: StorageMap<AssetId, u32> = StorageMap {},
    trader_orders: StorageMap<Address, StorageVec<b256>> = StorageMap {},
    order_indexes_by_trader: StorageMap<Address, StorageMap<b256, u64>> = StorageMap {},
}

impl OrderBook for Contract {
    #[storage(read, write)]
    fn create_market(asset: AssetId, decimals: u32) {
        require(asset != QUOTE_TOKEN, AssetError::InvalidAsset);
        require(
            storage
                .markets
                .get(asset)
                .try_read()
                .is_none(),
            MarketError::DuplicateMarket,
        );

        storage.markets.insert(asset, decimals);

        log(CreateMarketEvent {
            asset,
            decimals,
        });
    }

    #[payable]
    #[storage(read, write)]
    fn open_order(asset_id: AssetId, size: I64, price: u64) -> b256 {
        let trader = trader();

        // Market must exist to place an order
        let market = storage.markets.get(asset_id).try_read();
        require(market.is_some(), MarketError::NoMarketFound);

        let order = Order::new(trader, asset_id, size, price);
        let order_id = order.id();

        // Reject duplicate orders
        require(
            storage
                .orders
                .get(order_id)
                .try_read()
                .is_none(),
            OrderError::DuplicateOrder,
        );

        // Based on size determine the required deposit to open the order
        let asset = order.calculate_deposit(
            storage
                .markets,
            PRICE_DECIMALS,
            QUOTE_TOKEN_DECIMALS,
            QUOTE_TOKEN,
        );

        require(msg_amount() == asset.amount, AssetError::InvalidAssetAmount);
        require(msg_asset_id() == asset.id, AssetError::InvalidAsset);

        storage.orders.insert(order_id, order);
        storage.trader_orders.get(trader).push(order_id);
        storage
            .order_indexes_by_trader
            .get(trader)
            .insert(order_id, storage.trader_orders.get(trader).len());

        log(OpenOrderEvent {
            order_id,
            trader,
            asset: asset_id,
            size,
            price,
        });

        order_id
    }

    #[payable]
    #[storage(read, write)]
    fn update_order(order_id: b256, size: I64, price: u64) {
        // Order should not be able to exist without a market already being created therefore no check here
        // Prove order exists in order to update
        let order = storage.orders.get(order_id).try_read();
        require(order.is_some(), OrderError::NoOrdersFound);

        let trader = trader();
        let original_order = order.unwrap();

        let mut new_order = original_order;
        let new_order = new_order.set_price(price).set_size(size);

        // Based on size determine the required deposit to update the order
        let asset = original_order.calculate_deposit(
            storage
                .markets,
            PRICE_DECIMALS,
            QUOTE_TOKEN_DECIMALS,
            QUOTE_TOKEN,
        );

        // TODO: shouldn't this be the differece between the open price and current update price?
        require(msg_amount() == asset.amount, AssetError::InvalidAssetAmount);
        require(msg_asset_id() == asset.id, AssetError::InvalidAsset);

        // TODO: clean up
        let (asset_1, asset_2) = match original_order.size == size.flip() {
            true => {
                let mut mock_order = original_order;
                mock_order.flip();

                let asset_1 = original_order.calculate_deposit(
                    storage
                        .markets,
                    PRICE_DECIMALS,
                    QUOTE_TOKEN_DECIMALS,
                    QUOTE_TOKEN,
                );
                remove_order(original_order);

                let asset_2 = mock_order.calculate_deposit(
                    storage
                        .markets,
                    PRICE_DECIMALS,
                    QUOTE_TOKEN_DECIMALS,
                    QUOTE_TOKEN,
                );
                (asset_1, asset_2)
            },
            false => {
                let mut asset_1 = Asset::new(0, BASE_ASSET_ID);
                let mut asset_2 = Asset::new(0, BASE_ASSET_ID);

                if !original_order.size.is_same_sign(size) {
                    let mut mock_order = original_order;
                    mock_order.set_size(I64 {
                        value: min(original_order.size.value, size.value),
                        negative: mock_order.size.negative,
                    });

                    asset_1 = mock_order.calculate_deposit(
                        storage
                            .markets,
                        PRICE_DECIMALS,
                        QUOTE_TOKEN_DECIMALS,
                        QUOTE_TOKEN,
                    );
                    mock_order.flip();
                    asset_2 = mock_order.calculate_deposit(
                        storage
                            .markets,
                        PRICE_DECIMALS,
                        QUOTE_TOKEN_DECIMALS,
                        QUOTE_TOKEN,
                    );
                }
                remove_update_order_internal(original_order, size);
                (asset_1, asset_2)
            }
        };

        // TODO: some type of refund here
        if asset_1.amount > 0 {
            transfer_to_address(trader, asset_1.id, asset_1.amount);
        }
        if asset_2.amount > 0 {
            transfer_to_address(trader, asset_2.id, asset_2.amount);
        }

        log(UpdateOrderEvent {
            order_id,
            size,
            price,
        });
    }

    #[storage(read, write)]
    fn cancel_order(order_id: b256) {
        // Order must exist to be cancelled
        let order = storage.orders.get(order_id).try_read();
        require(order.is_some(), OrderError::NoOrdersFound);
        let order = order.unwrap();

        // Only the owner of the order may cancel their order
        let trader = trader();
        require(trader == order.trader, OrderError::AccessDenied);

        let asset = order.calculate_deposit(
            storage
                .markets,
            PRICE_DECIMALS,
            QUOTE_TOKEN_DECIMALS,
            QUOTE_TOKEN,
        );

        remove_order(order);

        transfer_to_address(trader, asset.id, asset.amount);

        log(CancelOrderEvent { order_id });
    }

    #[storage(read, write)]
    fn match_orders(sell_order: b256, buy_order: b256) {
        let trader = trader();

        let sell_order = storage.orders.get(sell_order).try_read();
        let buy_order = storage.orders.get(buy_order).try_read();
        require(
            sell_order
                .is_some() && buy_order
                .is_some(),
            OrderError::NoOrdersFound,
        );

        let sell_order = sell_order.unwrap();
        let buy_order = buy_order.unwrap();
        require(
            sell_order
                .size
                .negative && !buy_order
                .size
                .negative,
            Error::FirstArgumentShouldBeOrderSellSecondOrderBuy, // TODO: rename
        );
        require(
            sell_order
                .asset == buy_order
                .asset,
            OrderError::AssetMismatch,
        );
        require(
            sell_order
                .price <= buy_order
                .price,
            OrderError::InsufficientBuyPrice,
        );

        let mut mock_order = sell_order;
        mock_order.flip();

        let trade_size = min(
            sell_order
                .size
                .value,
            buy_order
                .size
                .value
                .mul_div(buy_order.price, sell_order.price),
        );
        mock_order.size.value = trade_size;

        let asset_1 = mock_order.calculate_deposit(
            storage
                .markets,
            PRICE_DECIMALS,
            QUOTE_TOKEN_DECIMALS,
            QUOTE_TOKEN,
        );
        remove_update_order_internal(sell_order, mock_order.size);

        mock_order.flip();

        let asset_2 = mock_order.calculate_deposit(
            storage
                .markets,
            PRICE_DECIMALS,
            QUOTE_TOKEN_DECIMALS,
            QUOTE_TOKEN,
        );
        mock_order.size.value = mock_order.size.value.mul_div_rounding_up(sell_order.price, buy_order.price);
        remove_update_order_internal(buy_order, mock_order.size);

        require(
            asset_1
                .amount != 0 && asset_2
                .amount != 0,
            AssetError::InvalidAssetAmount,
        );

        transfer_to_address(sell_order.trader, asset_1.id, asset_1.amount);
        transfer_to_address(buy_order.trader, asset_2.id, asset_2.amount);

        log(TradeEvent {
            asset: sell_order.asset,
            order_matcher: trader,
            buyer: buy_order.trader,
            seller: sell_order.trader,
            trade_size,
            trade_price: sell_order.price,
        });
    }
}

impl Info for Contract {
    #[storage(read)]
    fn trader_orders(trader: Address) -> Vec<b256> {
        storage.trader_orders.get(trader).load_vec()
    }

    #[storage(read)]
    fn order(order: b256) -> Option<Order> {
        storage.orders.get(order).try_read()
    }

    #[storage(read)]
    fn market(asset: AssetId) -> Option<u32> {
        storage.markets.get(asset).try_read()
    }

    fn configurables() -> (AssetId, u32, u32) {
        (QUOTE_TOKEN, QUOTE_TOKEN_DECIMALS, PRICE_DECIMALS)
    }

    fn order_id(trader: Address, asset: AssetId, price: u64) -> b256 {
        // This must match with Order::id()
        sha256((trader, asset, price))
    }
}

#[storage(read, write)]
fn remove_update_order_internal(order: Order, size: I64) {
    let order_id = order.id();

    match order.size == size.flip() {
        true => {
            let pos_id = storage.order_indexes_by_trader.get(order.trader).get(order_id).read() - 1; // pos + 1 indexed
            assert(storage.order_indexes_by_trader.get(order.trader).remove(order_id));
            assert(storage.trader_orders.get(order.trader).swap_remove(pos_id) == order_id);
            assert(storage.orders.remove(order_id));
        }
        false => {
            let mut order = order;
            order.size += size;
            storage.orders.insert(order_id, order);
        }
    }
}

#[storage(read, write)]
fn remove_order(order: Order) {
    let order_id = order.id();
    let pos_id = storage.order_indexes_by_trader.get(order.trader).get(order_id).read() - 1; // pos + 1 indexed
    assert(storage.order_indexes_by_trader.get(order.trader).remove(order_id));
    assert(storage.trader_orders.get(order.trader).swap_remove(pos_id) == order_id);
    assert(storage.orders.remove(order_id));
}

#[storage(write)]
fn increase_order_size(order: Order) {
    let mut order = order;
    order.size += order.size;
    storage.orders.insert(order.id(), order);
}
