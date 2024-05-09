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

const ORDER_ID: &str = "0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367";

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
order_id: 0x{:?},
sender: {:?},
timestamp: {:?},
tx_id: {:?},
base_token: {:?},
base_size: {:?},
base_price: {:?},
    ",
        Address::from(event.order_id.0),
        event.sender,
        event.timestamp,
        event.identifier,
        tx_id_hex,
        order.map_or_else(|| "-".to_owned(), |o| o.base_token.to_string()),
        order.map_or_else(
            || "-".to_owned(),
            |o| o.base_size.clone().as_i64().to_string()
        ),
        order.map_or_else(|| "-".to_owned(), |o| o.base_price.to_string()),
    );
}


/* 
1
order_id: 0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367,
sender: Address(e845e2607087c64d0bd1a7fdcc2f3d9b8e9880675b1c9bc46044863d18c828d5),
timestamp: 4611686020142525856,
identifier: OrderOpenEvent,
tx_id: "0x7a2904700b44ec18fe690200901d9ae4904baf5b6de7de6dc25b3b025c9bebe6",
base_token: "593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746",
base_size: "4865628",
base_price: "65629999999999",

2
order_id: 0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367,
sender: Address(831c401a7ee97c41d76292fcebc31b7c9f70667c5914eeab277043e1c2c84ed0),
timestamp: 4611686020142525862,
identifier: OrderMatchEvent,
tx_id: "0x8016fc8e3313689d23aec6a395cf1fd9d245b1be60155127a4bc77f3271f9710",
base_token: "593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746",
base_size: "4816092",
base_price: "65629999999999",

3
order_id: 0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367,
sender: Address(194c4d5d321ea3bc2e87109f4a86520ad60f924998f67007d487d3cc0acc45d2),
timestamp: 4611686020142525863,
identifier: OrderMatchEvent,
tx_id: "0x366b59882b3ebc2c15fbceee5801522a84f210e32abfd0249e7edd2fd8830411",
base_token: "593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746",
base_size: "3939246",
base_price: "65629999999999",

4
order_id: 0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367,
sender: Address(194c4d5d321ea3bc2e87109f4a86520ad60f924998f67007d487d3cc0acc45d2),
timestamp: 4611686020142525865,
identifier: OrderMatchEvent,
tx_id: "0xb477e2171e6df5f7dd83932668aa1968a47684afec34de46e58c681337abbcf8",
base_token: "593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746",
base_size: "3632829",
base_price: "65629999999999",

6
order_id: 0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367,
sender: Address(8e3bcc4316900e48929b4826cca9af280292f72f8665351ecfaa9ffdadb7b637),
timestamp: 4611686020142525867,
identifier: OrderMatchEvent,
tx_id: "0x7ec49087c8a834da29d5ce7c1c53291230442166ba62a06fbd85a9e461c73276",
base_token: "593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746",
base_size: "3446785",
base_price: "65629999999999",

5
order_id: 0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367,
sender: Address(194c4d5d321ea3bc2e87109f4a86520ad60f924998f67007d487d3cc0acc45d2),
timestamp: 4611686020142525867,
identifier: OrderMatchEvent,
tx_id: "0x3fcbc994b6819e957adef3c2969e11d10a64534115bbd9b7f465a322765a3de0",
base_token: "593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746",
base_size: "3258118",
base_price: "65629999999999",

8
order_id: 0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367,
sender: Address(8e3bcc4316900e48929b4826cca9af280292f72f8665351ecfaa9ffdadb7b637),
timestamp: 4611686020142525873,
identifier: OrderMatchEvent,
tx_id: "0xa6bcdb4fef6a030cb430118b5a85d3cf13c3b9d2494e0bca0df7d835d22ffe41",
base_token: "593b117a05f5ea64b39ba1f9bc3fb7e7a791c9be130e28376ad552eacdb3b746",
base_size: "3082263",
base_price: "65629999999999",

7
order_id: 0x84a8ef15146b2551dc97fa8f70ec99eae9f9e14826779fca4078c4325ed0e367,
sender: Address(194c4d5d321ea3bc2e87109f4a86520ad60f924998f67007d487d3cc0acc45d2),
timestamp: 4611686020142525873,
identifier: OrderMatchEvent,
tx_id: "0x4547dcca9f6d7572eb5d9908d9db2a4ab791e999f4da190bd51f4413534422c5",
base_token: "-",
base_size: "-",
base_price: "-",
*/