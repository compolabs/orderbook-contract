use fuels::{prelude::*, types::Bits256};
use orderbook::orderbook_utils::Orderbook;
use src20_sdk::token_utils::{deploy_token_contract, Asset};

const PRICE_DECIMALS: u64 = 9;
const BASE_SIZE: u64 = 1; //units
const BASE_PRICE: u64 = 70000; //units

abigen!(Script(
    name = "MatchScript",
    abi = "match-script/out/debug/match-script-abi.json"
));

#[tokio::test]
async fn match_script_test() {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];
    let alice = &wallets[1];

    let token_contract = deploy_token_contract(&admin).await;
    let base_asset = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");
    let token_contract = deploy_token_contract(&admin).await;
    let quote_asset = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(
        &admin,
        quote_asset.asset_id,
        quote_asset.decimals,
        PRICE_DECIMALS,
    )
    .await;

    // Create Market
    orderbook
        ._create_market(base_asset.asset_id, base_asset.decimals as u32)
        .await
        .unwrap();

    let price = BASE_PRICE * 10u64.pow(orderbook.price_decimals as u32);

    //mint base asset to sell
    let base_size = base_asset.parse_units(BASE_SIZE as f64) as u64;
    base_asset
        .mint(alice.address().into(), base_size)
        .await
        .unwrap();

    // sell
    let sell_order_id = orderbook
        .with_account(&alice)
        .open_order(base_asset.asset_id, -1 * (base_size / 2) as i64, price - 1)
        .await
        .unwrap()
        .value;
    // let sell_order1_id = orderbook
    //     .with_account(&alice)
    //     .open_order(base_asset.asset_id, -1 * (base_size / 2) as i64, price - 1)
    //     .await
    //     .unwrap()
    //     .value;

    //mint quote asset to buy
    let quote_size = quote_asset.parse_units(BASE_SIZE as f64 * BASE_PRICE as f64);
    quote_asset
        .mint(alice.address().into(), quote_size as u64)
        .await
        .unwrap();

    //buy
    let buy_order_id = orderbook
        .with_account(&alice)
        .open_order(base_asset.asset_id, base_size as i64, price)
        .await
        .unwrap()
        .value;

    let match_script = MatchScript::new(admin.clone(), "match-script/out/debug/match-script.bin")
        .with_configurables(
            MatchScriptConfigurables::default().with_ORDER_BOOK_CONTRACT_ID(
                Bits256::from_hex_str(&orderbook.instance.contract_id().hash().to_string()).unwrap(),
            ).unwrap(),
        );

    match_script
        .main(sell_order_id, vec![buy_order_id])
        .with_contracts(&[&orderbook.instance])
        .with_tx_policies(TxPolicies::default().with_tip(1))
        .append_variable_outputs(2)
        .call()
        .await
        .unwrap();
}
