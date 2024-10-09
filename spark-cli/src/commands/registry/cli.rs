use crate::commands::registry::{
    config::ConfigCommand, deploy::DeployCommand, markets::MarketsCommand,
    register::RegisterCommand, unregister::UnregisterCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum RegistryCommands {
    /// Deploy a new market registry contract
    #[clap(short_flag = 'C')]
    Config(ConfigCommand),

    /// Deploy a new market registry contract
    #[clap(short_flag = 'D')]
    Deploy(DeployCommand),

    /// Unregister a market in the market registry contract
    #[clap(short_flag = 'M')]
    Markets(MarketsCommand),

    /// Register a new market in the market registry contract
    #[clap(short_flag = 'R')]
    Register(RegisterCommand),

    /// Unregister a market in the market registry contract
    #[clap(short_flag = 'U')]
    Unregister(UnregisterCommand),
}
