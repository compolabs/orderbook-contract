use crate::setup::setup;
use fuels::types::Identity;

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialized")]
    async fn reverts_when_already_initialized() {
        let (contract, _, user) = setup().await.unwrap();

        let new_owner: Identity = user.wallet.address().into();
        contract
            .with_account(&user.wallet)
            .initialize_ownership(new_owner)
            .await
            .unwrap();
    }
}
