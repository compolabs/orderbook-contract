contract;

mod errors;
mod events;

use errors::*;
use events::*;

use std::{constants::ZERO_B256, hash::{Hash, sha256},};

configurable {
    OWNER: Address = Address::from(ZERO_B256),
}

storage {
    markets: StorageMap<b256, ContractId> = StorageMap {},
}

abi MarketInfo {
    fn config() -> (Address, AssetId, u32, AssetId, u32, u32, AssetId);
}

abi Orderbook {
    #[storage(read, write)]
    fn register_market(market: ContractId);

    #[storage(write)]
    fn unregister_market(market: ContractId);

    #[storage(read)]
    fn markets(market_assets: Vec<(AssetId, AssetId)>) -> Vec<(AssetId, AssetId, Option<ContractId>)>;

    fn config() -> Address;
}

impl Orderbook for Contract {
    #[storage(read, write)]
    fn register_market(market: ContractId) {
        require(
            msg_sender()
                .unwrap()
                .as_address()
                .unwrap() == OWNER,
            AuthError::Unauthorized,
        );
        let (base, quote) = market_assets(market);
        let id = market_id(base, quote);
        require(
            storage
                .markets
                .get(id)
                .try_read()
                .is_none(),
            OrderbookError::MarketAlreadyRegistered,
        );
        storage.markets.insert(id, market);
        log(MarketRegisterEvent {
            base: base,
            quote: quote,
            market: market,
        });
    }

    #[storage(write)]
    fn unregister_market(market: ContractId) {
        require(
            msg_sender()
                .unwrap()
                .as_address()
                .unwrap() == OWNER,
            AuthError::Unauthorized,
        );
        let (base, quote) = market_assets(market);
        let id = market_id(base, quote);
        require(
            storage
                .markets
                .remove(id),
            OrderbookError::MarketNotRegistered,
        );
        log(MarketUnregisterEvent {
            base: base,
            quote: quote,
            market: market,
        });
    }

    #[storage(read)]
    fn markets(market_assets: Vec<(AssetId, AssetId)>) -> Vec<(AssetId, AssetId, Option<ContractId>)> {
        let mut markets = Vec::new();
        let mut idx = 0;
        while idx < market_assets.len() {
            let (base, quote) = market_assets.get(idx).unwrap();
            markets.push((base, quote, storage.markets.get(market_id(base, quote)).try_read()));
            idx += 1;
        }
        markets
    }

    fn config() -> Address {
        OWNER
    }
}

fn market_assets(market: ContractId) -> (AssetId, AssetId) {
    let (_, base, _, quote, _, _, _) = abi(MarketInfo, market.into()).config();
    (base, quote)
}

fn market_id(base: AssetId, quote: AssetId) -> b256 {
    sha256((base, quote))
}
