contract;

mod errors;
mod events;

use errors::*;
use events::*;

use std::{constants::ZERO_B256, hash::{Hash, sha256},};

configurable {
    OWNER: Address = Address::from(ZERO_B256),
    VERSION: u32 = 0,
}

storage {
    markets: StorageMap<b256, ContractId> = StorageMap {},
}

abi SparkMarketInfo {
    fn config() -> (AssetId, u32, AssetId, u32, Identity, u32, u32);
}

abi SparkRegistry {
    #[storage(read, write)]
    fn register_market(market: ContractId);

    #[storage(write)]
    fn unregister_market(market: ContractId);

    #[storage(read)]
    fn markets(market_assets: Vec<(AssetId, AssetId)>) -> Vec<(AssetId, AssetId, Option<ContractId>)>;

    fn config() -> (Address, u32);
}

impl SparkRegistry for Contract {
    /// @notice Registers a new market with the given contract ID.
    /// @dev This function allows the contract owner to register a new market. It retrieves the base and quote assets associated with the market,
    ///      generates a unique market ID, and checks if the market is already registered. If the market is not registered, it is stored in the
    ///      contract's storage and a MarketRegisterEvent is logged. The function enforces that only the contract owner can call it.
    /// @param market The ContractId of the market to be registered.
    /// @return None - The function does not return a value.
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
            MarketRegistryError::MarketAlreadyRegistered,
        );
        storage.markets.insert(id, market);
        log(MarketRegisterEvent {
            base: base,
            quote: quote,
            market: market,
        });
    }

    /// @notice Unregisters an existing market identified by the given contract ID.
    /// @dev This function allows the contract owner to unregister a market. It retrieves the base and quote assets associated with the market,
    ///      generates the market ID, and checks if the market is currently registered. If the market is registered, it is removed from the
    ///      contract's storage and a MarketUnregisterEvent is logged. The function enforces that only the contract owner can call it.
    /// @param market The ContractId of the market to be unregistered.
    /// @return None - The function does not return a value.
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
            MarketRegistryError::MarketNotRegistered,
        );
        log(MarketUnregisterEvent {
            base: base,
            quote: quote,
            market: market,
        });
    }

    /// @notice Retrieves the contract IDs of markets for a given list of base and quote asset pairs.
    /// @dev This function takes a list of asset pairs and returns a vector containing each pair along with the corresponding market's contract ID if it is registered.
    ///      If a market is not registered, None is returned for the contract ID.
    /// @param market_assets A vector of tuples, where each tuple contains a base AssetId and a quote AssetId.
    /// @return A vector of tuples, where each tuple contains the base AssetId, the quote AssetId, and an Option<ContractId> representing the market's contract ID.
    ///         The Option<ContractId> is None if the market is not registered.
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

    /// @notice Retrieves the contract's configuration details, including the owner address and version number.
    /// @dev This function returns the owner address and the version number of the contract.
    /// @return A tuple containing the owner's Address and the contract's version number as a u32.
    fn config() -> (Address, u32) {
        (OWNER, VERSION)
    }
}

fn market_assets(market: ContractId) -> (AssetId, AssetId) {
    let (base, _, quote, _, _, _, _) = abi(SparkMarketInfo, market.into()).config();
    (base, quote)
}

fn market_id(base: AssetId, quote: AssetId) -> b256 {
    sha256((base, quote))
}
