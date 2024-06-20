use crate::commands::core::{
    cancel_order::CancelCommand, deploy::DeployCommand, deposit::DepositCommand,
    match_many::MatchManyCommand, match_pair::MatchPairCommand, open_order::OpenCommand,
    set_fee::SetFeeCommand, withdraw::WithdrawCommand,
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

    /// Open an order
    #[clap(short_flag = 'O')]
    Open(OpenCommand),

    /// Match a pair of order
    #[clap(short_flag = 'A')]
    MatchPair(MatchPairCommand),

    /// Match multiple order
    #[clap(short_flag = 'M')]
    MatchMany(MatchManyCommand),

    /// Set a fee for a specific user or the market
    #[clap(short_flag = 'S')]
    SetFee(SetFeeCommand),

    /// Withdraw from the market contract
    #[clap(short_flag = 'W')]
    Withdraw(WithdrawCommand),
}
