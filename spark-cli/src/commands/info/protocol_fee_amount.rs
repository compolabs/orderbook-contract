use crate::utils::{setup, validate_contract_id, AssetType};
use clap::Args;
use spark_market_sdk::{AssetType as ContractAssetType, MarketContract};

#[derive(Args, Clone)]
#[command(about = "Query the protocol fee amount")]
pub(crate) struct ProtocolFeeAmountCommand {
    /// The amount of asset
    #[clap(long)]
    pub(crate) amount: u64,

    /// The asset type of the market
    #[clap(long)]
    pub(crate) asset_type: AssetType,

    /// The contract id of the market
    #[clap(long)]
    pub(crate) contract_id: String,

    /// The URL to query
    /// Ex. testnet.fuel.network
    #[clap(long)]
    pub(crate) rpc: String,
}

impl ProtocolFeeAmountCommand {
    pub(crate) async fn run(&self) -> anyhow::Result<()> {
        let wallet = setup(&self.rpc).await?;
        let contract_id = validate_contract_id(&self.contract_id)?;

        // Connect to the deployed contract via the rpc
        let contract = MarketContract::new(contract_id, wallet).await;
        let asset_type = match self.asset_type {
            AssetType::Base => ContractAssetType::Base,
            AssetType::Quote => ContractAssetType::Quote,
        };

        let protocol_fee_amount = contract
            .protocol_fee_amount(self.amount, asset_type)
            .await?
            .value;

        println!("Protocol Fee Amount: {}", protocol_fee_amount);

        Ok(())
    }
}
