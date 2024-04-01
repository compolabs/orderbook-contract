use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::Bits256,
};
use orderbook::{
    constants::{ORDERBOOK_CONTRACT_ID, RPC},
    orderbook_utils::Orderbook,
    print_title,
};

const ORDER_ID: &str = "0x2b04ddb4fe13fd38f0edf10c347ba059f8404bc9063e76857df31a414163db38";

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
