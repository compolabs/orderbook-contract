library;

pub struct MarketRegisterEvent {
    pub asset_id: AssetId,
    pub market: ContractId,
}

pub struct MarketUnregisterEvent {
    pub asset_id: AssetId,
}
