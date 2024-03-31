mod success {

    use crate::utils::{
        interface::{core::deposit, info::account},
        setup::setup,
    };

    #[tokio::test]
    async fn returns_none() {
        let contract = setup().await;
    }
}
