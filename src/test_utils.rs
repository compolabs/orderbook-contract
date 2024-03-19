pub use crate::orderbook_utils::Orderbook;
pub use src20_sdk::token_utils::{deploy_token_contract, Asset};
#[cfg(test)] 
pub use pretty_assertions::assert_eq;

pub use fuels::types::Bits256;
pub use fuels::{accounts::wallet::Wallet, prelude::*};
pub use std::result::Result;

const TOLERANCE: f64 = 0.0005;
const PRICE_DECIMALS: u64 = 9;

pub fn tolerance_eq(expected: u64, actual: u64) -> bool {
    let difference = (expected as f64 - actual as f64).abs();
    let relative_difference = difference / expected as f64;

    if (expected == actual)
        || (expected == 0 && actual == 0)
        || ((expected > actual) && (relative_difference < TOLERANCE))
    {
        println!(
            "✅ Баланс в пределах допуска или точное соответствие. Expected: {}, Actual: {}",
            expected, actual
        );
        true
    } else {
        if expected < actual {
            println!(
                "❌ Баланс больше ожидаемого. Expected: {}, Actual: {}",
                expected, actual
            );
        } else {
            println!(
                "❌ Баланс за пределами допуска. Expected: {}, Actual: {}",
                expected, actual
            );
        }
        false
    }
}

pub async fn init() -> (WalletUnlocked, WalletUnlocked, Asset, Asset, Orderbook) {
    //--------------- WALLETS ---------------
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .expect("Failed to initialize wallets");
    let admin = wallets[0].clone();
    let alice = wallets[1].clone();
    let bob = wallets[2].clone();

    let btc_token_contract = deploy_token_contract(&admin).await;
    let btc = Asset::new(
        admin.clone(),
        btc_token_contract.contract_id().into(),
        "BTC",
    );

    let usdc_token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(
        admin.clone(),
        usdc_token_contract.contract_id().into(),
        "USDC",
    );

    let orderbook = Orderbook::deploy(&admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    // Create Market
    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .expect("Failed to create market");

    (alice, bob, btc, usdc, orderbook)
}

pub async fn mint_tokens(
    usdc: &Asset,
    btc: &Asset,
    alice: &Wallet,
    bob: &Wallet,
    usdc_mint_amount: u64,
    btc_mint_amount: u64,
) {
    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();
    btc.mint(bob.address().into(), btc_mint_amount)
        .await
        .unwrap();
}

pub async fn open_orders_match(
    orderbook: &Orderbook,
    alice: &WalletUnlocked,
    bob: &WalletUnlocked,
    btc: &Asset,
    buy_size: f64,
    buy_price: f64,
    sell_size: f64,
    sell_price: f64,
) -> Result<(Bits256, Bits256), fuels::types::errors::Error> {
    let alice_order_id = orderbook
        .with_account(&alice)
        .open_order(
            btc.asset_id,
            (buy_size * 1e8) as i64,
            (buy_price * 1e9) as u64,
        )
        .await
        .unwrap()
        .value;

    let bob_order_id = orderbook
        .with_account(&bob)
        .open_order(
            btc.asset_id,
            (sell_size * 1e8) as i64,
            (sell_price * 1e9) as u64,
        )
        .await
        .unwrap()
        .value;

    let res = orderbook.match_orders(&bob_order_id, &alice_order_id).await;
    if res.is_ok() {
        Ok((alice_order_id, bob_order_id))
    } else {
        Err(res.err().unwrap())
    }
}