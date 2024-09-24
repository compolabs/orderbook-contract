use crate::setup::{setup, Defaults};
use spark_market_sdk::ProtocolFee;

mod success {

    use super::*;
    use spark_market_sdk::SetProtocolFeeEvent;

    #[tokio::test]
    async fn sets_protocol_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 10,
                taker_fee: 15,
                volume_threshold: 0,
            },
            ProtocolFee {
                maker_fee: 8,
                taker_fee: 13,
                volume_threshold: 10000,
            },
            ProtocolFee {
                maker_fee: 6,
                taker_fee: 10,
                volume_threshold: 20000,
            },
        ];

        let response = contract.set_protocol_fee(protocol_fee.clone()).await?;

        // Log should be emitted when fee is changed
        let log = response
            .decode_logs_with_type::<SetProtocolFeeEvent>()
            .unwrap();
        let event = log.first().unwrap();
        assert_eq!(
            *event,
            SetProtocolFeeEvent {
                protocol_fee: protocol_fee
            }
        );

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "Unauthorized")]
    async fn reverts_when_non_owner() {
        let defaults = Defaults::default();
        let (contract, _owner, user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        let protocol_fee = vec![ProtocolFee {
            maker_fee: 10,
            taker_fee: 15,
            volume_threshold: 0,
        }];

        contract
            .with_account(&user.wallet)
            .set_protocol_fee(protocol_fee.clone())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidFeeZeroBased")]
    async fn protocol_fee_starts_with_non_zero_volume() {
        let defaults = Defaults::default();
        let (contract, _owner, _, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        // Change fee to be non-zero for testing purposes
        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 10,
                taker_fee: 15,
                volume_threshold: 2000,
            },
            ProtocolFee {
                maker_fee: 8,
                taker_fee: 13,
                volume_threshold: 10000,
            },
            ProtocolFee {
                maker_fee: 6,
                taker_fee: 10,
                volume_threshold: 20000,
            },
        ];

        let _ = contract
            .set_protocol_fee(protocol_fee.clone())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidFeeSorting")]
    async fn protocol_fee_is_not_sorted_by_volume() {
        let defaults = Defaults::default();
        let (contract, _owner, _, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await
        .unwrap();

        // Change fee to be non-zero for testing purposes
        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 10,
                taker_fee: 15,
                volume_threshold: 0,
            },
            ProtocolFee {
                maker_fee: 8,
                taker_fee: 13,
                volume_threshold: 100000,
            },
            ProtocolFee {
                maker_fee: 6,
                taker_fee: 10,
                volume_threshold: 20000,
            },
        ];

        let _ = contract
            .set_protocol_fee(protocol_fee.clone())
            .await
            .unwrap();
    }
}
