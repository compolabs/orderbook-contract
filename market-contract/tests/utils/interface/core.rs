use crate::utils::setup::{Market, OrderType};
use fuels::{
    accounts::wallet::WalletUnlocked,
    prelude::{CallParameters, TxPolicies},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension},
    types::{AssetId, Bits256, Identity},
};

pub(crate) async fn deposit(
    contract: &Market<WalletUnlocked>,
    amount: u64,
    asset: AssetId,
) -> FuelCallResponse<()> {
    let tx_params = TxPolicies::new(Some(1), Some(2_000_000), None, None, None);
    let call_params = CallParameters::new(amount, asset, 1_000_000);

    contract
        .methods()
        .deposit()
        .with_tx_policies(tx_params)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
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

#[allow(dead_code)]
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
