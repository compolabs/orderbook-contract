contract;

mod structs;
mod events;

use i64::*;
use structs::*;
use events::*;

use std::constants::{BASE_ASSET_ID};
use std::hash::*;
use std::storage::storage_vec::*;
use std::call_frames::msg_asset_id;
use std::context::msg_amount;


const DUST: u64 = 10;
const FEE_RATE: u64 = 500;
const HUNDRED_PERCENT: u64 = 1000000;

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
    fn remove_order(order_id: b256);
    
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
    fn create_market(asset_id: AssetId, decimal: u32){
        require(storage.markets.get(asset_id).try_read().is_none(), "Market already exists");
        let market = Market {asset_id, decimal};
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
        
        if base_size.negative {
            require(msg_amount() == base_size.value, "Bad amount transfered");
            require(msg_asset_id() == base_token, "Bad base token");
        } else {
            let market_scale = 10_u64.pow(market.unwrap().decimal);
            let trade_value = base_size.value * base_price / market_scale / 1000; /* 10**(9 Price - 6 Quote decimals) */
            require(msg_amount() == trade_value, "Bad trade value");
            require(msg_asset_id() == QUOTE_TOKEN, "Bad quote Token");
        }

        let trader_address = msg_sender_address();


        let order_id = gen_order_id(trader_address, base_token, base_price);
        let order = storage.orders.get(order_id).try_read();

        if order.is_some() {
            let mut order = order.unwrap();
            if (order.base_size * base_size).negative {
                // todo Логика возврата токенов аккаунту trader. transfer_to_address(to, asset_id, amount);
            }

            order.base_size += base_size;
            update_remove_order_internal(order);
            // todo Логирование события изменения заказа
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
    fn remove_order(order_id: b256) {
        let order = storage.orders.get(order_id).try_read();
        require(order.is_none(), "Bad order");

        let mut order = order.unwrap();
        require(msg_sender_address() == order.trader, "Not an order owner");

        order.base_size.value = 0;
        update_remove_order_internal(order);
        // transfer funds
    }
    
    #[storage(read, write)]
    fn match_orders(order_sell_id: b256, order_buy_id: b256){
        //todo
    }
        
    #[storage(read)]
    fn orders_by_trader(trader: Address) -> Vec<b256>{
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
fn update_remove_order_internal(order: Order) {
    if order.base_size.value == 0 {
        let pos_id = storage.order_positions_by_trader.get(order.trader).get(order.id).read() - 1; // pos + 1 indexed
        assert(storage.order_positions_by_trader.get(order.trader).remove(order.id));
        storage.orders_by_trader.get(order.trader).swap_remove(pos_id);
        assert(storage.orders.remove(order.id));
    } else {
        storage.orders.insert(order.id, order);
    }
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