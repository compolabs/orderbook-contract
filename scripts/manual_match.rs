use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Bits256, ContractId},
};
use orderbook::{
    constants::{ORDERBOOK_CONTRACT_ID, RPC, TOKEN_CONTRACT_ID},
    orderbook_utils::Orderbook,
    print_title,
};
use src20_sdk::token_utils::TokenContract;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    print_title("Match Orders");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let orderbook = Orderbook::new(&wallet, ORDERBOOK_CONTRACT_ID).await;
    let pairs: Vec<(Bits256, Bits256)> = vec![
    (
        "0x0423084cdcc863989b5e424a28b54f2b2a0e6661afba6533a20af87c9bbb2a0f",
        "0x5054fcd3a8d21aa23b264a622bc67c74df2d37f5b42a7f8c3d34862c74d1b442",
    ),
    (
        "0x78cb6eb606f78884d637837f9da05259a75fce93d667de05cb46ad800a2d6986",
        "0xcb996d3b84ea222267cabcc8f2c33cd9f482b3f808f73570e466fb51f401aba4",
    ),
    (
        "0x9d94f94a8a331b1b10775bb10371a4727a3c8d50590d7e5e1830a05d140ea0d4",
        "0x9ef1719af2a10562f616a44ae024f003b57e56c3eabaee574f028f8c9181854c",
    ),
    (
        "0x525d616b1411ea876c40932a96d738db3fd28bc600d0442fb1c4e552dc05d7ec",
        "0x3fad1ff1cde09a9fe82d8d17c58abdd058f19451e6df5b932c6c9be210982c76",
    ),
    (
        "0x41e5d8e0f6820cdecbeaf56a5a95583438e64e4af41308ed377f814b57c6b40a",
        "0x33484f04f72f15cca5af3433bd9f5964daf970d560db7dc32d869ff054f7377d",
    ),
    (
        "0x46ed7a086e7cee739a08bd97dbdd787817485592762fe60351de403babb6539f",
        "0x838a48ab53452d15374b4d9a59e103ad5cca07a5e10817d01c99f9b436c6e6ab",
    ),
    (
        "0x687c42d7f4b8365e871571ec3cb21445dacbef45b6a11a4da725a8f6bf4de2ef",
        "0x92c428048afcc7512d45dd0e796c80133d3f096ffb2d6be1d0e693a0848edeb7",
    ),
    (
        "0x24f9b16f86970b34f4e1817fc1bc1d695dc771aaa8231e8be2c773ecfad7418b",
        "0x251841c7df70ed2a34505bc32a7932753cdef0127e7c91f0e8afb873c77c4dd9",
    ),
    (
        "0xc38d672dba29c9b6ff9e760fe5b16319019e400e7df457488866a7e929c2beb0",
        "0xcc9c2c9b8f1c09598ac8c224118120a7623896ff7289ce1c4f12ad881814dfc5",
    ),
    (
        "0x0381311251e61a9c62558a0f5f58007829680a407555481ed140783ff6e21ff1",
        "0xe75dc394d78a6d2e8aa625f55d9062af6c41497b198f6c62e3bf74fc5e07545d",
    ),




    ]
    .iter()
    .map(|(s, b)| {
        (
            Bits256::from_hex_str(s).unwrap(),
            Bits256::from_hex_str(b).unwrap(),
        )
    })
    .collect();

    for pair in pairs {
        let res = orderbook.match_orders(&pair.0, &pair.1).await;
        if res.is_ok() {
            println!("orders matched = {:?}", pair);
        } else {
            println!("err = {:?}", res.err().unwrap());
        }
    }
}