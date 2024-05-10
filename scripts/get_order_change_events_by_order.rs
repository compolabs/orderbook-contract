use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Address, Bits256},
};
use orderbook::{
    constants::{ORDERBOOK_CONTRACT_ID, RPC},
    orderbook_utils::{OrderChangeEvent, Orderbook},
    print_title,
};
const ORDER_ID: &str = "0xf23b5ac54f9a7ea1e47e141a44717e227d88e1c185591bbde083ca37df0abc38";

#[tokio::main]
async fn main() {
    print_title("Get order change events by order id");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;

    match orderbook
        .get_order_change_events_by_order(Bits256::from_hex_str(ORDER_ID).unwrap())
        .await
    {
        Ok(result) => {
            let res = result.value;
            for event in res {
                print_order_change_event(event);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn print_order_change_event(event: OrderChangeEvent) {
    let order = event.order.as_ref();
    let tx_id_hex = format!("0x{}", hex::encode(&event.tx_id.0));

    println!(
        "
        identifier: {:?},
        tx_id: {:?},
        timestamp: {:?},
        sender: {:?},
        base_token: {:?},
        base_size: {:?},
        base_price: {:?},
        order_id: 0x{:?},
    ",
        event.identifier,
        tx_id_hex,
        event.timestamp,
        event.sender,
        format!(
            "0x{}",
            order.map_or_else(|| "-".to_owned(), |o| o.base_token.to_string())
        ),
        order.map_or_else(
            || "-".to_owned(),
            |o| o.base_size.clone().as_i64().to_string()
        ),
        order.map_or_else(|| "-".to_owned(), |o| o.base_price.to_string()),
        Address::from(event.order_id.0),
    );
}
