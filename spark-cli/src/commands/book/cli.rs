use crate::commands::book::{
    config::ConfigCommand, deploy::DeployCommand, markets::MarketsCommand,
    register::RegisterCommand, unregister::UnregisterCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum BookCommands {
    /// Deploy a new orderbook contract
    #[clap(short_flag = 'C')]
    Config(ConfigCommand),

    /// Deploy a new orderbook contract
    #[clap(short_flag = 'D')]
    Deploy(DeployCommand),

    /// Unegister a market in orderbook contract
    #[clap(short_flag = 'M')]
    Markets(MarketsCommand),

    /// Register a new market in orderbook contract
    #[clap(short_flag = 'R')]
    Register(RegisterCommand),

    /// Unegister a market in orderbook contract
    #[clap(short_flag = 'U')]
    Unregister(UnregisterCommand),
}
