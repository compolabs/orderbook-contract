mod success {

    use crate::utils::{interface::info::config, setup::setup};
    use fuels::types::Identity;

    #[tokio::test]
    async fn returns_config() {
        let price_decimals = 9;
        let (contract, owner, _user, assets) = setup(9, 9, price_decimals).await;

        let owner = match owner {
            Identity::Address(address) => address,
            _ => panic!("Invalid setup in returns_config"),
        };

        assert_eq!(
            config(&contract).await.value,
            (
                owner,
                assets.base.id,
                assets.base.decimals,
                assets.quote.id,
                assets.quote.decimals,
                price_decimals
            )
        );
    }
}
