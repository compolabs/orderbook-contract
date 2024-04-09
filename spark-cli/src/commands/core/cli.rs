use crate::commands::core::{
    batch_fulfill::BatchCommand, close_order::CloseCommand, deploy::DeployCommand,
    deposit::DepositCommand, open_order::OpenCommand, set_fee::SetFeeCommand,
    withdraw::WithdrawCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum CoreCommands {
    ///
    #[clap(short_flag = 'B')]
    Batch(BatchCommand),

    ///
    #[clap(short_flag = 'C')]
    Close(CloseCommand),

    ///
    #[clap(short_flag = 'D')]
    Deploy(DeployCommand),

    ///
    #[clap(short_flag = 'P')]
    Deposit(DepositCommand),

    ///
    #[clap(short_flag = 'O')]
    Open(OpenCommand),

    ///
    #[clap(short_flag = 'S')]
    SetFee(SetFeeCommand),

    ///
    #[clap(short_flag = 'W')]
    Withdraw(WithdrawCommand),
}
