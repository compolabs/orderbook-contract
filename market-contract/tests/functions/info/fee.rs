mod success {

    use crate::utils::{interface::info::fee, setup::setup};
    use fuels::types::Identity;

    #[tokio::test]
    async fn returns_global_fee() {
        let (contract, owner, _user, assets) = setup(9, 9, 9).await;
    }

    #[tokio::test]
    async fn returns_premium_user_fee() {
        let (contract, owner, _user, assets) = setup(9, 9, 9).await;
    }

    #[tokio::test]
    async fn returns_global_fee_for_non_premium_user() {
        let (contract, owner, _user, assets) = setup(9, 9, 9).await;
    }
}
