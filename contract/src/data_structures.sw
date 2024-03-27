library;

use i64::I64;
use std::hash::{Hash, sha256};
use ::errors::MarketError;
use ::math::size_to_quote;

pub struct Asset {
    amount: u64,
    id: AssetId,
}

impl Asset {
    pub fn new(amount: u64, id: AssetId) -> Self {
        Self { amount, id }
    }
}

pub struct Order {
    trader: Address,
    asset: AssetId,
    size: I64,
    price: u64,
}

impl Order {
    pub fn new(trader: Address, asset: AssetId, size: I64, price: u64) -> Self {
        Self {
            trader,
            asset,
            size,
            price,
        }
    }

    pub fn flip(ref mut self) {
        self.size = self.size.flip();
    }

    pub fn id(self) -> b256 {
        sha256((self.trader, self.asset, self.price))
    }

    #[storage(read)]
    pub fn calculate_deposit(
        self,
        markets: StorageKey<StorageMap<AssetId, u32>>,
        size: I64,
        price: u64,
        PRICE_DECIMALS: u32,
        QUOTE_TOKEN_DECIMALS: u32,
        QUOTE_TOKEN: AssetId,
) -> (u64, AssetId) {
        match size.negative {
            true => (size.value, self.asset),
            false => {
                // Assumes check for the market has been made in caller
                let market = markets.get(self.asset).read();
                (
                    size_to_quote(
                        size.value,
                        market,
                        price,
                        PRICE_DECIMALS,
                        QUOTE_TOKEN_DECIMALS,
                    ),
                    QUOTE_TOKEN,
                )
            },
        }
    }

    #[storage(read)]
    pub fn calculate_refund(
        self,
        markets: StorageKey<StorageMap<AssetId, u32>>,
        PRICE_DECIMALS: u32,
        QUOTE_TOKEN_DECIMALS: u32,
        QUOTE_TOKEN: AssetId,
) -> Asset {
        match self.size.negative {
            true => Asset::new(self.size.value, self.asset),
            false => {
                // Assumes check for the market has been made in caller
                let market = markets.get(self.asset).read();
                let amount = size_to_quote(
                    self.size
                        .value,
                    market,
                    self.price,
                    PRICE_DECIMALS,
                    QUOTE_TOKEN_DECIMALS,
                );
                Asset::new(amount, QUOTE_TOKEN)
            }
        }
    }
}
