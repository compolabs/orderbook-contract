use crate::setup::{now_tai64, setup, Defaults};
use spark_market_sdk::SetEpochEvent;
use tokio;

mod success {

    use super::*;

    #[tokio::test]
    async fn set_epoch_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let one_month = (86400.0 * 365.25 / 12.0) as u64;

        // Fetch the initial epoch values
        let (uninitialized_epoch, uninitialized_epoch_duration) = contract.get_epoch().await?.value;

        assert_eq!(uninitialized_epoch, 0);
        assert_eq!(uninitialized_epoch_duration, one_month);

        let tai64_epoch = now_tai64();

        // Define the new epoch duration (e.g., 1 day)
        let epoch_duration = 60 * 60 * 24;

        // Increase the epoch and duration
        let response = contract.set_epoch(tai64_epoch, epoch_duration).await?;

        // Log should be emitted when epoch is changed
        let log = response.decode_logs_with_type::<SetEpochEvent>().unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            SetEpochEvent {
                epoch: tai64_epoch,
                epoch_duration
            }
        );

        // Check if the epoch values have been updated correctly
        let (new_epoch, new_epoch_duration) = contract.get_epoch().await?.value;

        assert_eq!(tai64_epoch, new_epoch);
        assert_eq!(epoch_duration, new_epoch_duration);

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn reverts_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _, user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let new_epoch = 0;
        let epoch_duration = 60 * 60 * 24;

        // Attempt to set the epoch with a non-owner user
        contract
            .with_account(&user.wallet)
            .set_epoch(new_epoch, epoch_duration)
            .await
            .unwrap();
    }
}
