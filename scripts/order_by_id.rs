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

const ORDER_ID: &str = "0x97ff915f6783fdba0ed03d8dc0b60f90186c08646ebbe0cd1e25863ee700478d";

#[tokio::main]
async fn main() {
    print_title("Order by id order");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    let id = &Bits256::from_hex_str(ORDER_ID).unwrap();

    let order = orderbook.order_by_id(id).await.unwrap().value;
    println!("order = {:?}", order);
}
