contract;

mod errors;
mod events;

use errors::*;
use events::*;

use std::{constants::ZERO_B256, hash::Hash,};

configurable {
    OWNER: Address = Address::from(ZERO_B256),
}

storage {
    markets: StorageMap<AssetId, ContractId> = StorageMap {},
}

abi Orderbook {
    #[storage(read, write)]
    fn register_market(asset_id: AssetId, market: ContractId);

    #[storage(read, write)]
    fn unregister_market(asset_id: AssetId);

    #[storage(read)]
    fn registered_markets(asset_ids: Vec<AssetId>) -> Vec<(AssetId, Option<ContractId>)>;

    fn config() -> Address;
}

impl Orderbook for Contract {
    #[storage(read, write)]
    fn register_market(asset_id: AssetId, market: ContractId) {
        require(
            msg_sender()
                .unwrap()
                .as_address()
                .unwrap() == OWNER,
            AuthError::Unauthorized,
        );
        require(
            storage
                .markets
                .get(asset_id)
                .try_read()
                .is_none(),
            OrderbookError::MarketAlreadyRegistered,
        );
        storage.markets.insert(asset_id, market);
        log(MarketRegisterEvent {
            asset_id: asset_id,
            market: market,
        });
    }

    #[storage(read, write)]
    fn unregister_market(asset_id: AssetId) {
        require(
            msg_sender()
                .unwrap()
                .as_address()
                .unwrap() == OWNER,
            AuthError::Unauthorized,
        );
        require(
            storage
                .markets
                .remove(asset_id),
            OrderbookError::MarketNotRegistered,
        );
        log(MarketUnregisterEvent {
            asset_id: asset_id,
        });
    }

    #[storage(read)]
    fn registered_markets(asset_ids: Vec<AssetId>) -> Vec<(AssetId, Option<ContractId>)> {
        let mut markets = Vec::new();
        let mut idx = 0;
        while idx < asset_ids.len() {
            let asset_id = asset_ids.get(idx).unwrap();
            markets.push((asset_id, storage.markets.get(asset_id).try_read()));
            idx += 1;
        }
        markets
    }

    fn config() -> Address {
        (OWNER)
    }
}
