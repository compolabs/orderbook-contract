contract;
mod structs;
mod events;
use i64::*;
use structs::*;
use events::*;
use std::storage::*;
use std::constants::{BASE_ASSET_ID};
use std::hash::*;
use std::storage::storage_vec::*;
use std::call_frames::msg_asset_id;
use std::context::msg_amount;


const DUST: u64 = 10;
const FEE_RATE: u64 = 500;
const HUNDRED_PERCENT: u64 = 1000000;

configurable {
    USDC_ADDRESS: AssetId = BASE_ASSET_ID,
}


storage {
    orders: StorageMap<b256, Order> = StorageMap{},
    markets: StorageMap<AssetId, Market> = StorageMap{},
    orders_by_trader: StorageMap<Address, StorageVec<b256>> = StorageMap{},
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

}

impl OrderBook for Contract {
    
    #[storage(read, write)]
    fn create_market(asset_id: AssetId, decimal: u32){
        //todo verify admin only
        require(storage.markets.get(asset_id).try_read().is_none(), "Market already exists or uninitialized");
        let market = Market {asset_id, decimal};
        storage.markets.insert(asset_id, market);
    }

    #[storage(read, write), payable]
    fn open_order(base_token: AssetId, base_size: I64, order_price: u64) { //price decimal = 9
        require(order_price > 0, "Invalid order price");
        
        let trader = msg_sender_address();

        let market = storage.markets.get(base_token).try_read();
        require(market.is_some(), "Market is not found");
        let market = market.unwrap();

        if base_size < I64::new() {
            let size = base_size.value;
            require(msg_amount() == size, "Incorrect amount sent");
            require(msg_asset_id() == base_token, "Incorrect asset sent");
        } else {
            let scale = 10_u64.pow(market.decimal + 9 - 6);
            let trade_value = (base_size.value * order_price) / scale;
            require(msg_amount() == trade_value, "Incorrect trade value");
            require(msg_asset_id() == USDC_ADDRESS, "Incorrect asset (expected USDC)");
        }

        let id = calc_order_id(trader, base_token, order_price);
        let existing_order = storage.orders.get(id).try_read();

        if existing_order.is_some() {
            let mut order = existing_order.unwrap();
            if order.base_size * base_size < I64::new() {
                // todo Логика возврата токенов аккаунту trader. transfer_to_address(to, asset_id, amount);
            }

            order.base_size += base_size;
            if order.base_size != I64::new() {
                storage.orders.insert(id, order);
            } else {
                remove_order_internal(id);
            }
            // todo Логирование события изменения заказа
        } else {
            let new_order = Order {
                id,
                trader,
                base_token,
                base_size,
                order_price
            };
            storage.orders.insert(id, new_order);
            storage.orders_by_trader.get(trader).push(id);
            // todo Логирование события создания нового заказа
        }
    }
    
    #[storage(read, write)]
    fn remove_order(order_id: b256){
        //todo
    }
    
    #[storage(read, write)]
    fn match_orders(order_sell_id: b256, order_buy_id: b256){
        //todo
    }
        
    #[storage(read)]
    fn orders_by_trader(trader: Address)-> Vec<b256>{
        //todo
        Vec::new()
    }

}
#[storage(read, write)]
fn remove_order_internal(order_id: b256) {
    let order = storage.orders.get(order_id).read();
    let orders = storage.orders_by_trader.get(order.trader);
    let success = storage.orders.remove(order_id);
    assert(success);

    //удаляем заказ из storage.orders
    let mut i = 0;
    while i < orders.len() {
        let id = orders.get(i).unwrap().read();
        if id == order_id{
            let _res = storage.orders_by_trader.get(order.trader).remove(i);
            break;
        }
        i += 1;
    }

}


fn calc_order_id(trader: Address, base_token: AssetId, price: u64) -> b256 {
    sha256((trader, base_token, price))
}


pub fn msg_sender_address() -> Address {
    match std::auth::msg_sender().unwrap() {
        Identity::Address(identity) => identity,
        _ => revert(0),
    }
}