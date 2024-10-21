use crate::setup::{setup, Defaults};
use fuels::types::Identity;

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "CannotReinitialized")]
    async fn reverts_when_already_initialized() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let new_owner: Identity = user.wallet.address().into();
        contract
            .with_account(&user.wallet)
            .initialize_ownership(new_owner)
            .await
            .unwrap();
    }
}
