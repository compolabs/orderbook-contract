pub use crate::orderbook_utils::Orderbook;
pub use fuels::types::Bits256;
pub use fuels::{accounts::wallet::Wallet, prelude::*};
pub use src20_sdk::token_utils::{deploy_token_contract, Asset};
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

pub async fn init_wallets() -> (WalletUnlocked, WalletUnlocked, WalletUnlocked) {
    let wallets_config = WalletsConfig::new(Some(3), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .expect("Failed to initialize wallets");

    let admin = wallets[0].clone();
    let alice = wallets[1].clone();
    let bob = wallets[2].clone();

    (admin, alice, bob)
}

pub async fn init_tokens(admin: &WalletUnlocked, symbol: &str) -> (Asset, Asset) {
    let usdc_token_contract = deploy_token_contract(&admin).await;
    let usdc = Asset::new(
        admin.clone(),
        usdc_token_contract.contract_id().into(),
        "USDC",
    );

    let token_contract = deploy_token_contract(&admin).await;
    let token = Asset::new(admin.clone(), token_contract.contract_id().into(), symbol);
    (usdc, token)
}

pub async fn init_orderbook(admin: &WalletUnlocked, usdc: &Asset, token: &Asset) -> Orderbook {
    let orderbook = Orderbook::deploy(admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    orderbook
        ._create_market(token.asset_id, token.decimals as u32)
        .await
        .expect("Failed to create market");

    orderbook
}

pub async fn mint_tokens(
    usdc: &Asset,
    token: &Asset,
    alice: &Wallet,
    bob: &Wallet,
    usdc_mint_amount: u64,
    token_mint_amount: u64,
) {
    usdc.mint(alice.address().into(), usdc_mint_amount)
        .await
        .unwrap();
    token
        .mint(bob.address().into(), token_mint_amount)
        .await
        .unwrap();
}

pub async fn open_orders_match(
    orderbook: &Orderbook,
    alice: &WalletUnlocked,
    bob: &WalletUnlocked,
    token: &Asset,
    buy_size: f64,
    buy_price: f64,
    sell_size: f64,
    sell_price: f64,
) -> Result<(Bits256, Bits256), fuels::types::errors::Error> {
    let alice_order_id = orderbook
        .with_account(&alice)
        .open_order(
            token.asset_id,
            (buy_size * 1e8) as i64,
            (buy_price * 1e9) as u64,
        )
        .await
        .unwrap()
        .value;

    let bob_order_id = orderbook
        .with_account(&bob)
        .open_order(
            token.asset_id,
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
