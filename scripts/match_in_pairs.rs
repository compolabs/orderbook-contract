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

#[tokio::main]
async fn main() {
    print_title("Match Orders");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;
    let pairs: Vec<(Bits256, Bits256)> = vec![(
        "0xca8fba1367edfc9aa6083aa0da46180bdaf2394d831a4d9690092016ccf5e0db",
        "0x5054fcd3a8d21aa23b264a622bc67c74df2d37f5b42a7f8c3d34862c74d1b442",
    )]
    .iter()
    .map(|(s, b)| {
        (
            Bits256::from_hex_str(s).unwrap(),
            Bits256::from_hex_str(b).unwrap(),
        )
    })
    .collect();
    orderbook.match_in_pairs(pairs).await.unwrap();
    // for pair in pairs {
    //     let res = orderbook.match_orders(&pair.0, &pair.1).await;
    //     if res.is_ok() {
    //         println!("orders matched = {:?}", pair);
    //     } else {
    //         println!("err = {:?}", res.err().unwrap());
    //     }
    // }
}
