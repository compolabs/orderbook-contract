use fuels::accounts::{wallet::WalletUnlocked, ViewOnlyAccount};
pub use src20_sdk::token_utils::Asset;

const TOLERANCE: f64 = 0.0005;

pub async fn check_balance(wallet: &WalletUnlocked, asset: &Asset, expected_balance: u64) {
    let actual_balance = wallet.get_asset_balance(&asset.asset_id).await.unwrap();
    assert!(tolerance_eq(expected_balance, actual_balance));
}
fn tolerance_eq(expected: u64, actual: u64) -> bool {
    let difference = (expected as f64 - actual as f64).abs();
    let relative_difference = difference / expected as f64;

    (expected == actual)
        || (expected == 0 && actual == 0)
        || ((expected > actual) && (relative_difference < TOLERANCE))
}
