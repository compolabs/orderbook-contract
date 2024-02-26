use std::str::FromStr;

use fuels::{prelude::*, types::ContractId};
use orderbook::{constants::TOKEN_CONTRACT_ID, orderbook_utils::Orderbook};
use src20_sdk::token_utils::{Asset, TokenContract};

#[tokio::test]
async fn main_test() {
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));
    let wallets = launch_custom_provider_and_get_wallets(wallets_config, None, None)
        .await
        .unwrap();
    let admin = &wallets[0];
    // let alice = &wallets[1];

    let orderbook = Orderbook::deploy(&admin).await;

    let id = ContractId::from_str(TOKEN_CONTRACT_ID).unwrap().into();
    let token_contarct = TokenContract::new(&id, admin.clone());

    let btc = Asset::new(admin.clone(), token_contarct.contract_id().into(), "BTC");

    orderbook
        ._create_market(btc.asset_id, btc.decimals as u32)
        .await
        .unwrap();
}
