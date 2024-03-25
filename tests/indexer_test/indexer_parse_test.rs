use std::path::PathBuf;

use fuels::test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig};
use orderbook::orderbook_utils::Orderbook;
use src20_sdk::token_utils::{deploy_token_contract, Asset};

#[tokio::test]
async fn indexer_parse_test() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/indexer_test/indexer_data.json");
    let addresses_json = std::fs::read_to_string(path).unwrap();
    let data: serde_json::Value = serde_json::from_str(&addresses_json).unwrap();
    let _receipts_str = data["data"][0]["receipts"].as_str().unwrap();

    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];

    let token_contract = deploy_token_contract(&admin).await;
    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, 9).await;

    let _res = orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .unwrap();

    //fixme https://github.com/FuelLabs/fuels-rs/issues/1308
    // let receipts: &[Receipt] = serde_json::from_str(&receipts_str).unwrap();
    // let res = res
    //     .log_decoder
    // .decode_logs_with_type::<OrderChangeEvent>(receipts);
}
