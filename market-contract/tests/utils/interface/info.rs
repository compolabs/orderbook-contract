use crate::utils::setup::{Account, Market, Order, OrderType};
use fuels::{
    accounts::wallet::WalletUnlocked,
    programs::call_response::FuelCallResponse,
    types::{Address, AssetId, Bits256, Identity},
};

pub(crate) async fn account(
    contract: &Market<WalletUnlocked>,
    user: Identity,
) -> FuelCallResponse<Option<Account>> {
    contract.methods().account(user).call().await.unwrap()
}

pub(crate) async fn fee(
    contract: &Market<WalletUnlocked>,
    user: Option<Identity>,
) -> FuelCallResponse<u64> {
    contract.methods().fee(user).call().await.unwrap()
}

pub(crate) async fn order(
    contract: &Market<WalletUnlocked>,
    order: Bits256,
) -> FuelCallResponse<Option<Order>> {
    contract.methods().order(order).call().await.unwrap()
}

pub(crate) async fn user_orders(
    contract: &Market<WalletUnlocked>,
    user: Identity,
) -> FuelCallResponse<Vec<Bits256>> {
    contract.methods().user_orders(user).call().await.unwrap()
}

pub(crate) async fn config(
    contract: &Market<WalletUnlocked>,
) -> FuelCallResponse<(Address, AssetId, u32, AssetId, u32, u32)> {
    contract.methods().config().call().await.unwrap()
}

pub(crate) async fn order_id(
    contract: &Market<WalletUnlocked>,
    amount: u64,
    asset: AssetId,
    order_type: OrderType,
    owner: Identity,
    price: u64,
) -> FuelCallResponse<Bits256> {
    contract
        .methods()
        .order_id(amount, asset, order_type, owner, price)
        .call()
        .await
        .unwrap()
}
