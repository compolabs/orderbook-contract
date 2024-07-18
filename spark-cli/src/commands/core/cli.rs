use crate::commands::core::{
    cancel_order::CancelCommand, deploy::DeployCommand, deposit::DepositCommand,
    fulfill_many::FulfillManyCommand, match_many::MatchManyCommand, match_pair::MatchPairCommand,
    open_order::OpenCommand, set_matcher_fee::SetMatcherFeeCommand,
    set_protocol_fee::SetProtocolFeeCommand, withdraw::WithdrawCommand,
    withdraw_protocol_fee::WithdrawProtocolFeeCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum CoreCommands {
    /// Cancel an open order
    #[clap(short_flag = 'C')]
    Cancel(CancelCommand),

    /// Deploy a new market contract
    #[clap(short_flag = 'D')]
    Deploy(DeployCommand),

    /// Deposit into the market contract
    #[clap(short_flag = 'P')]
    Deposit(DepositCommand),

    /// Fulfill multiple orders
    #[clap(short_flag = 'F')]
    FulfillMany(FulfillManyCommand),

    /// Match multiple orders
    #[clap(short_flag = 'M')]
    MatchMany(MatchManyCommand),

    /// Match a pair of orders
    #[clap(short_flag = 'A')]
    MatchPair(MatchPairCommand),

    /// Open an order
    #[clap(short_flag = 'O')]
    Open(OpenCommand),

    /// Set a protocol fee
    #[clap(short_flag = 'S')]
    SetProtocolFee(SetProtocolFeeCommand),

    /// Set a matcher fee for the market
    #[clap(short_flag = 'T')]
    SetMatcherFee(SetMatcherFeeCommand),

    /// Withdraw from the market contract
    #[clap(short_flag = 'U')]
    WithdrawProtocolFee(WithdrawProtocolFeeCommand),

    /// Withdraw from the market contract
    #[clap(short_flag = 'W')]
    Withdraw(WithdrawCommand),
}
