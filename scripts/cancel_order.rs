use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::Bits256,
};
use orderbook::{constants::ORDERBOOK_CONTRACT_ID, orderbook_utils::Orderbook, print_title};
use src20_sdk::constants::RPC;

const ORDER_ID: &str = "0x72a6ade704246cac5699b2016cb8bbcaa91ad254678efb4c4ffc12c3777db2f9";

#[tokio::main]
async fn main() {
    print_title("Cancel order");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    let id = &Bits256::from_hex_str(ORDER_ID).unwrap();

    let order = orderbook.order_by_id(id).await.unwrap().value;

    assert!(order.is_some());

    orderbook
        .cancel_order(id) //fixme TransferZeroCoins
        .await
        .unwrap();
}
