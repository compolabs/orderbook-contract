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

const ORDER_ID: &str = "0x8fef66690f7266698f61f77ed151c79614a8daa80525a6f72559acef0829ed90";

#[tokio::main]
async fn main() {
    print_title("Get order change events by order id");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    let res = orderbook
        .get_order_change_events_by_order(Bits256::from_hex_str(ORDER_ID).unwrap())
        .await
        .unwrap()
        .value;

    println!("res = {:?}", res);
}
