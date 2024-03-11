use fuels::prelude::*;
use orderbook::orderbook_utils::{Orderbook, I64};
use src20_sdk::token_utils::{deploy_token_contract, Asset};

const PRICE_DECIMALS: u64 = 9;

//noinspection RsVariableNaming
#[tokio::test]
async fn open_base_token_order_cancel_test() {
    // ✅ buyOrder.orderPrice > sellOrder.orderPrice & buyOrder.baseSize > sellOrder.baseSize
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];
    let alice = &wallets[1];
    let bob = &wallets[2];

    let token_contract = deploy_token_contract(&admin).await;
    let btc = Asset::new(admin.clone(), token_contract.contract_id().into(), "BTC");
    let token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(admin.clone(), token_contract.contract_id().into(), "USDC");

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    // Create Market
    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .unwrap();

    // Mint BTC & USDC

    let buy_price = usdc.parse_units(46000_f64) as u64; // Higher buy price
    let sell_price = usdc.parse_units(45000_f64) as u64; // Lower sell price

    let buy_size = btc.parse_units(2.0) as i64; // Larger buy size
    let sell_size = -1 * btc.parse_units(1.0) as i64; // Smaller sell size

    let amount_usdc = usdc.parse_units(92000_f64) as u64;
    let amount_btc = btc.parse_units(1_f64) as u64;

    // alice mints 92000 usdc
    usdc.mint(alice.address().into(), amount_usdc)
        .await
        .unwrap();

    assert_eq!(
        alice.get_asset_balance(&usdc.asset_id).await.unwrap(),
        amount_usdc
    );

    // bob mints 1 btc
    btc.mint(bob.address().into(), amount_btc).await.unwrap();

    assert_eq!(
        bob.get_asset_balance(&btc.asset_id).await.unwrap(),
        amount_btc
    );

    let call_params = CallParameters::default()
        .with_asset_id(usdc.asset_id)
        .with_amount(amount_usdc);

    orderbook
        .with_account(alice)
        .instance
        .methods()
        .open_order(btc.asset_id, I64::from(buy_size).clone(), buy_price)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    assert_eq!(alice.get_asset_balance(&usdc.asset_id).await.unwrap(), 0);

    let call_params = CallParameters::default()
        .with_asset_id(btc.asset_id)
        .with_amount(amount_btc);

    orderbook
        .with_account(bob)
        .instance
        .methods()
        .open_order(btc.asset_id, I64::from(sell_size), sell_price)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap();

    assert_eq!(bob.get_asset_balance(&btc.asset_id).await.unwrap(), 0);


    // let response = orderbook
    //     .instance
    //     .methods()
    //     .orders_by_trader(alice.address())
    //     .call()
    //     .await
    //     .unwrap();

    // let alice_order_id = response
    //     .value
    //     .get(0)
    //     .unwrap();
    //
    //
    //
    // let response = orderbook
    //     .instance
    //     .methods()
    //     .orders_by_trader(bob.address())
    //     .call()
    //     .await
    //     .unwrap();
    // let bob_order_id = response
    //     .value
    //     .get(0)
    //     .unwrap();
    //
    // orderbook
    //     .instance
    //     .methods()
    //     .match_orders(*alice_order_id, *bob_order_id)
    //     .append_variable_outputs(2)
    //     .call()
    //     .await
    //     .unwrap();
}
