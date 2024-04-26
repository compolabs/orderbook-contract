use std::str::FromStr;

use dotenv::dotenv;
use fuels::{
    prelude::{Provider, WalletUnlocked},
    types::{Bits256, ContractId},
};
use spark_market_sdk::OrderbookContract;
use utils::{
    constants::{ORDERBOOK_CONTRACT_ID, RPC},
    title::print_title,
};

const ORDER_ID: &str = "0x2b04ddb4fe13fd38f0edf10c347ba059f8404bc9063e76857df31a414163db38";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_title("Cancel order");
    dotenv().ok();
    let provider = Provider::connect(RPC).await.unwrap();
    let secret = std::env::var("ADMIN").unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let orderbook =
        OrderbookContract::new(ContractId::from_str(ORDERBOOK_CONTRACT_ID).unwrap(), wallet)
            .await?;

    let id = &Bits256::from_hex_str(ORDER_ID)?;

    let order = orderbook.order_by_id(id).await?.value;

    assert!(order.is_some());

    orderbook
        .cancel_order(id) //fixme TransferZeroCoins
        .await?;

    Ok(())
}
