use fuels::{
    accounts::wallet::WalletUnlocked,
    test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig},
};

use orderbook::orderbook_utils::Orderbook;
use src20_sdk::token_utils::{deploy_token_contract, Asset};

const PRICE_DECIMALS: u64 = 9;
pub async fn init_orderbook(admin: &WalletUnlocked, usdc: &Asset, token: &Asset) -> Orderbook {
    let orderbook = Orderbook::deploy(admin, usdc.asset_id, usdc.decimals, PRICE_DECIMALS).await;

    orderbook
        ._create_market(token.asset_id, token.decimals as u32)
        .await
        .expect("Failed to create market");

    orderbook
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

 struct TestContext {
    pub admin: WalletUnlocked,
    pub user: WalletUnlocked,
    pub usdc: Asset,
    pub token: Asset,
    pub orderbook: Orderbook,
}
pub(crate) async fn setup() -> TestContext {
    let (admin, user, _user2) = init_wallets().await;
    let (usdc, token) = init_tokens(&admin, "BTC").await;
    let orderbook = init_orderbook(&admin, &usdc, &token).await;

    TestContext {
        admin,
        user,
        usdc,
        token,
        orderbook,
    }
}
