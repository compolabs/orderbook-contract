mod success {

    use crate::utils::{
        interface::info::config,
        setup::{setup, Defaults},
    };

    #[tokio::test]
    async fn returns_config() {
        let defaults = Defaults::default();
        let (contract, owner, _user, assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await;

        assert_eq!(
            config(&contract).await.value,
            (
                owner.address(),
                assets.base.id,
                assets.base.decimals,
                assets.quote.id,
                assets.quote.decimals,
                defaults.price_decimals
            )
        );
    }
}
