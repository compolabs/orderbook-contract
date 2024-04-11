use crate::commands::core::{
    batch_fulfill::BatchCommand, close_order::CloseCommand, deploy::DeployCommand,
    deposit::DepositCommand, open_order::OpenCommand, set_fee::SetFeeCommand,
    withdraw::WithdrawCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum CoreCommands {
    /// Attempt to batch solve orders
    #[clap(short_flag = 'B')]
    Batch(BatchCommand),

    /// Close an open order
    #[clap(short_flag = 'C')]
    Close(CloseCommand),

    /// Deploy a new market contract
    #[clap(short_flag = 'D')]
    Deploy(DeployCommand),

    /// Deposit into the market contract
    #[clap(short_flag = 'P')]
    Deposit(DepositCommand),

    /// Open an order
    #[clap(short_flag = 'O')]
    Open(OpenCommand),

    /// Set a fee for a specific user or the market
    #[clap(short_flag = 'S')]
    SetFee(SetFeeCommand),

    /// Withdraw from the market contract
    #[clap(short_flag = 'W')]
    Withdraw(WithdrawCommand),
}
