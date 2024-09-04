library;

pub struct MarketRegisterEvent {
    pub base: AssetId,
    pub quote: AssetId,
    pub market: ContractId,
}

pub struct MarketUnregisterEvent {
    pub base: AssetId,
    pub quote: AssetId,
    pub market: ContractId,
}
