use fuels::prelude::*;
use orderbook::orderbook_utils::Orderbook;
use src20_sdk::token_utils::{deploy_token_contract, Asset};

const PRICE_DECIMALS: u64 = 9;

#[tokio::test]
async fn deploy_test() {
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];

    let token_contract = deploy_token_contract(&admin).await;
    let quote_asset = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(
        &admin,
        quote_asset.asset_id,
        quote_asset.decimals,
        PRICE_DECIMALS,
    )
    .await;

    let configurables = orderbook.get_configurables().await.unwrap().value;
    assert_eq!(configurables.0, quote_asset.asset_id);
    assert_eq!(configurables.1, quote_asset.decimals as u32);
    assert_eq!(configurables.2, PRICE_DECIMALS as u32);
}
