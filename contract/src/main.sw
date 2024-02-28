contract;

mod structs;
mod events;

use events::*;
use i64::*;
use structs::*;

use std::asset::*;
use std::call_frames::msg_asset_id;
use std::constants::BASE_ASSET_ID;
use std::context::msg_amount;
use std::hash::*;
use std::storage::storage_vec::*;


//const DUST: u64 = 10;
//const FEE_RATE: u64 = 500;
//const HUNDRED_PERCENT: u64 = 1000000;

configurable {
    QUOTE_TOKEN: AssetId = BASE_ASSET_ID,
}

storage {
    orders: StorageMap<b256, Order> = StorageMap{},
    markets: StorageMap<AssetId, Market> = StorageMap{},
    orders_by_trader: StorageMap<Address, StorageVec<b256>> = StorageMap{},
    order_positions_by_trader: StorageMap<Address, StorageMap<b256, u64>> = StorageMap{},
}

//todo fix reentrancy issues

abi OrderBook {
    
    #[storage(read, write)]
    fn create_market(asset_id: AssetId, decimal: u32);

    #[storage(read, write), payable]
    fn open_order(base_token: AssetId, base_size: I64, order_price: u64);
    
    #[storage(read, write)]
    fn cancel_order(order_id: b256);
    
    #[storage(read, write)]
    fn match_orders(order_sell_id: b256, order_buy_id: b256);
    
    #[storage(read)]
    fn orders_by_trader(trader: Address) -> Vec<b256>;

    #[storage(read)]
    fn order_by_id(order: b256) -> Option<Order>;

    #[storage(read)]
    fn market_exists(asset_id: AssetId) -> bool;
}

impl OrderBook for Contract {
    
    #[storage(read, write)]
    fn create_market(asset_id: AssetId, asset_decimals: u32) {
        require(asset_id != QUOTE_TOKEN, "No quote token market");
        require(storage.markets.get(asset_id).try_read().is_none(), "Market already exists");
        let market = Market {asset_id, asset_decimals};
        storage.markets.insert(asset_id, market);
    }

    #[storage(read)]
    fn market_exists(asset_id: AssetId) -> bool{
        !storage.markets.get(asset_id).try_read().is_none()
    }

    #[storage(read, write), payable]
    fn open_order(base_token: AssetId, base_size: I64, base_price: u64 /* decimal = 9 */) {
        let market = storage.markets.get(base_token).try_read();
        require(market.is_some(), "Market not found");
        require(base_price != 0, "Zero base price");

        let market = market.unwrap();
        if base_size.negative {
            require(msg_amount() == base_size_to_base_amount(base_size.value, market.asset_decimals), "Bad amount transfered");
            require(msg_asset_id() == base_token, "Bad base token");
        } else {
            require(msg_amount() == base_size_to_quote_amount(base_size.value, market.asset_decimals, base_price), "Bad trade value");
            require(msg_asset_id() == QUOTE_TOKEN, "Bad quote Token");
        }

        let trader_address = msg_sender_address();

        let order_id = gen_order_id(trader_address, base_token, base_price);
        let order = storage.orders.get(order_id).try_read();

        if order.is_some() {
            let order = order.unwrap();
            let mut refund = (BASE_ASSET_ID, 1);
            if (order.base_size + base_size).value == 0 {
                refund = cancel_order_internal(order);
            } else {
                let mut order = order;
                order.base_size += base_size;
                update_order_internal(order);
                // todo Логирование события изменения заказа
                if (order.base_size * base_size).negative {
                    // todo Логика возврата токенов аккаунту trader. transfer_to_address(to, asset_id, amount);
                }
            }
        } else {
            let order = Order {
                id: order_id,
                trader: trader_address,
                base_token,
                base_size,
                base_price
            };
            add_order_internal(order);
            // todo Логирование события создания нового заказа
        }
    }
    
    #[storage(read, write)]
    fn cancel_order(order_id: b256) {
        let order = storage.orders.get(order_id).try_read();
        require(order.is_some(), "Bad order");

        let order = order.unwrap();
        let msg_sender = msg_sender_address();
        require(msg_sender == order.trader, "Not an order owner");

        // log event

        let refund = cancel_order_internal(order);
        assert(refund.0 == order.base_token);
        assert(refund.1 == order.base_size.value * 100000000);
        //transfer_to_address(msg_sender, refund.0, refund.1);
    }
    
    #[storage(read, write)]
    fn match_orders(order_sell_id: b256, order_buy_id: b256) {
        // log event
    }
        
    #[storage(read)]
    fn orders_by_trader(trader: Address) -> Vec<b256> {
        storage.orders_by_trader.get(trader).load_vec()
    }

    #[storage(read)]
    fn order_by_id(order: b256) -> Option<Order> {
        storage.orders.get(order).try_read()
    }
}

#[storage(read, write)]
fn add_order_internal(order: Order) {
    storage.orders.insert(order.id, order);
    storage.orders_by_trader.get(order.trader).push(order.id);
    storage.order_positions_by_trader.get(order.trader).insert(
        order.id, storage.orders_by_trader.get(order.trader).len()); // pos + 1 indexed
}

#[storage(read, write)]
fn update_order_internal(order: Order) {
    assert(order.base_size.value != 0);
    storage.orders.insert(order.id, order);
}

#[storage(read, write)]
fn cancel_order_internal(order: Order) -> (AssetId, u64) {
    assert(order.base_size.value != 0);
    let pos_id = storage.order_positions_by_trader.get(order.trader).get(order.id).read() - 1; // pos + 1 indexed
    assert(storage.order_positions_by_trader.get(order.trader).remove(order.id));
    assert(storage.orders_by_trader.get(order.trader).swap_remove(pos_id) == order.id);
    assert(storage.orders.remove(order.id));
    order_return_asset_amount(order)
}

#[storage(read)]
fn order_return_asset_amount(order: Order) -> (AssetId, u64) {
    let market = storage.markets.get(order.base_token).try_read().unwrap();
    return if order.base_size.negative {
        (order.base_token, base_size_to_base_amount(order.base_size.value, market.asset_decimals))
    } else {
        assert(false);
        (QUOTE_TOKEN, base_size_to_quote_amount(order.base_size.value, market.asset_decimals, order.base_price))
    } 
}

fn base_size_to_base_amount(base_size: u64, base_decimals: u32) -> u64 {
    base_size * 10_u64.pow(base_decimals)
}

fn base_size_to_quote_amount(base_size: u64, base_decimals: u32, base_price: u64) -> u64 {
    // Rework Price and USDC decimals
    base_size * base_price / 10_u64.pow(base_decimals) / 1000 /* 10**(9 Price - 6 Quote decimals) */
}

fn gen_order_id(trader_address: Address, base_token: AssetId, base_price: u64) -> b256 {
    sha256((trader_address, base_token, base_price))
}

pub fn msg_sender_address() -> Address {
    match std::auth::msg_sender().unwrap() {
        Identity::Address(identity) => identity,
        _ => revert(0),
    }
}