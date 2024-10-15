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

    #[tokio::test]
    async fn sets_detailed_protocol_fee() -> anyhow::Result<()> {
        let defaults = Defaults::default();
        let (contract, _owner, _user, _, _, _assets) = setup(
            defaults.base_decimals,
            defaults.quote_decimals,
            defaults.price_decimals,
        )
        .await?;

        let protocol_fee = vec![
            ProtocolFee {
                maker_fee: 25,       // 0.25% maker fee
                taker_fee: 40,       // 0.40% taker fee
                volume_threshold: 0, // $0 - $10,000
            },
            ProtocolFee {
                maker_fee: 20,                    // 0.20% maker fee
                taker_fee: 35,                    // 0.35% taker fee
                volume_threshold: 10_000_000_000, // $10,001 - $50,000
            },
            ProtocolFee {
                maker_fee: 14,                    // 0.14% maker fee
                taker_fee: 24,                    // 0.24% taker fee
                volume_threshold: 50_000_000_000, // $50,001 - $100,000
            },
            ProtocolFee {
                maker_fee: 12,                     // 0.12% maker fee
                taker_fee: 22,                     // 0.22% taker fee
                volume_threshold: 100_000_000_000, // $100,001 - $250,000
            },
            ProtocolFee {
                maker_fee: 10,                     // 0.10% maker fee
                taker_fee: 20,                     // 0.20% taker fee
                volume_threshold: 250_000_000_000, // $250,001 - $500,000
            },
            ProtocolFee {
                maker_fee: 8,                      // 0.08% maker fee
                taker_fee: 18,                     // 0.18% taker fee
                volume_threshold: 500_000_000_000, // $500,001 - $1,000,000
            },
            ProtocolFee {
                maker_fee: 6,                        // 0.06% maker fee
                taker_fee: 16,                       // 0.16% taker fee
                volume_threshold: 1_000_000_000_000, // $1,000,001 - $2,500,000
            },
            ProtocolFee {
                maker_fee: 4,                        // 0.04% maker fee
                taker_fee: 14,                       // 0.14% taker fee
                volume_threshold: 2_500_000_000_000, // $2,500,001 - $5,000,000
            },
            ProtocolFee {
                maker_fee: 2,                        // 0.02% maker fee
                taker_fee: 12,                       // 0.12% taker fee
                volume_threshold: 5_000_000_000_000, // $5,000,001 - $10,000,000
            },
            ProtocolFee {
                maker_fee: 0,                         // 0.00% maker fee
                taker_fee: 10,                        // 0.10% taker fee
                volume_threshold: 10_000_000_000_000, // $10,000,001+
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
                protocol_fee: protocol_fee.clone()
            }
        );

        Ok(())
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotOwner")]
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
