use crate::utils::setup::{Market, OrderType};
use fuels::{
    accounts::wallet::WalletUnlocked,
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::{AssetId, Bits256, Identity},
};

pub(crate) async fn deposit(contract: &Market<WalletUnlocked>) -> FuelCallResponse<()> {
    contract.methods().deposit().call().await.unwrap()
}

pub(crate) async fn withdraw(
    contract: &Market<WalletUnlocked>,
    amount: u64,
    asset: AssetId,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .withdraw(amount, asset)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn open_order(
    contract: &Market<WalletUnlocked>,
    amount: u64,
    asset: AssetId,
    order_type: OrderType,
    price: u64,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .open_order(amount, asset, order_type, price)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn update_order(
    contract: &Market<WalletUnlocked>,
    amount: Option<u64>,
    order_id: Bits256,
    price: Option<u64>,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .update_order(amount, order_id, price)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn cancel_order(
    contract: &Market<WalletUnlocked>,
    order_id: Bits256,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .cancel_order(order_id)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn batch_fulfill(
    contract: &Market<WalletUnlocked>,
    order_id: Bits256,
    orders: Vec<Bits256>,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .batch_fulfill(order_id, orders)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn set_fee(
    contract: &Market<WalletUnlocked>,
    amount: u64,
    user: Option<Identity>,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .set_fee(amount, user)
        .call()
        .await
        .unwrap()
}
