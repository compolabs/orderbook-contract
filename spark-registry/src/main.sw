contract;

mod errors;
mod events;

use errors::*;
use events::*;

use std::hash::{Hash, sha256};
use standards::src5::{AccessError, SRC5, State};

configurable {
    OWNER: State = State::Uninitialized,
    VERSION: u32 = 0,
}

storage {
    markets: StorageMap<b256, ContractId> = StorageMap {},
}

abi SparkMarketInfoConfig {
    fn config() -> (AssetId, u32, AssetId, u32, Identity, u32, u32);
}

abi SparkRegistry {
    #[storage(read, write)]
    fn register_market(market: ContractId);

    #[storage(write)]
    fn unregister_market(market: ContractId);

    #[storage(read)]
    fn markets(market_assets: Vec<(AssetId, AssetId)>) -> Vec<(AssetId, AssetId, Option<ContractId>)>;

    fn config() -> (Option<Identity>, u32);
}

impl SRC5 for Contract {
    /// Returns the owner.
    ///
    /// # Returns
    ///
    /// * [State] - Represents the state of ownership for this contract.
    #[storage(read)]
    fn owner() -> State {
        OWNER
    }
}

impl SparkRegistry for Contract {
    /// Registers a new market with the given contract ID.
    ///
    /// ### Additional Information
    ///
    /// This function allows the contract owner to register a new market. It retrieves the base and quote assets associated with the market,
    /// generates a unique market ID, and checks if the market is already registered. If the market is not registered, it is stored in the
    /// contract's storage and a 'MarketRegisterEvent' is logged. The function enforces that only the contract owner can call it.
    ///
    /// ### Arguments
    ///
    /// * `market`: [ContractId] - The 'ContractId' of the market to be registered.
    ///
    /// ### Reverts
    ///
    /// * When called by non-owner.
    /// * When a token pair market already registered
    #[storage(read, write)]
    fn register_market(market: ContractId) {
        only_owner();

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

    /// Unregisters an existing market identified by the given contract ID.
    ///
    /// ### Additional Information
    ///
    /// This function allows the contract owner to unregister a market. It retrieves the base and quote assets associated with the market,
    /// generates the market ID, and checks if the market is currently registered. If the market is registered, it is removed from the
    /// contract's storage and a 'MarketUnregisterEvent' is logged. The function enforces that only the contract owner can call it.
    ///
    /// ### Arguments
    ///
    /// * `market`: [ContractId] - The 'ContractId' of the market to be unregistered.
    ///
    /// ### Reverts
    ///
    /// * When called by non-owner.
    /// * When a market is not registered
    #[storage(write)]
    fn unregister_market(market: ContractId) {
        only_owner();

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

    /// Retrieves the contract IDs of markets for a given list of base and quote asset pairs.
    ///
    /// ### Additional Information
    ///
    /// This function takes a list of asset pairs and returns a vector containing each pair along with the corresponding market's contract ID if it is registered.
    /// If a market is not registered, 'None' is returned for the contract ID.
    ///
    /// ### Arguments
    ///
    /// * `market_assets`: [Vec<(AssetId, AssetId)>] - A vector of tuples, where each tuple contains a base 'AssetId' and a quote 'AssetId'.
    ///
    /// ### Returns
    ///
    /// * [Vec<(AssetId, AssetId, Option<ContractId>)>] - A vector of tuples, where each tuple contains the base 'AssetId', the quote 'AssetId', and an 'Option<ContractId>' representing the market's contract ID.
    ///         The 'Option<ContractId>' is 'None' if the market is not registered.
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

    /// Retrieves the contract's configuration details, including the owner address and version number.
    ///
    /// ### Additional Information
    ///
    /// This function returns the owner address and the version number of the contract.
    ///
    /// ### Returns
    ///
    /// * [(Option<Identity>, u32)] - A tuple containing the Otion of owner's Identity and the contract's version number as a 'u32'.
    fn config() -> (Option<Identity>, u32) {
        (OWNER.owner(), VERSION)
    }
}

fn market_assets(market: ContractId) -> (AssetId, AssetId) {
    let (base, _, quote, _, _, _, _) = abi(SparkMarketInfoConfig, market.into()).config();
    (base, quote)
}

fn market_id(base: AssetId, quote: AssetId) -> b256 {
    sha256((base, quote))
}

fn only_owner() {
    require(
        OWNER
            .is_initialized() && msg_sender()
            .unwrap() == OWNER
            .owner()
            .unwrap(),
        AccessError::NotOwner,
    );
}
