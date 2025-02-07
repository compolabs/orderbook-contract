use crate::commands::{
    batch::cli::BatchCommands, core::cli::CoreCommands, info::cli::InfoCommands,
    registry::cli::RegistryCommands, upgrade::cli::UpgradeCommands,
};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "")] // TODO: about
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Clone, Subcommand)]
pub(crate) enum Command {
    ///
    #[clap(short_flag = 'B')]
    Batch(Batch),

    ///
    #[clap(short_flag = 'C')]
    Core(Core),

    ///
    #[clap(short_flag = 'I')]
    Info(Info),

    ///
    #[clap(short_flag = 'R')]
    Registry(Registry),

    ///
    #[clap(short_flag = 'U')]
    Upgrade(Upgrade),
}

#[derive(Args, Clone)]
pub(crate) struct Batch {
    #[clap(subcommand)]
    pub(crate) commands: BatchCommands,
}

#[derive(Args, Clone)]
pub(crate) struct Core {
    #[clap(subcommand)]
    pub(crate) commands: CoreCommands,
}

#[derive(Args, Clone)]
pub(crate) struct Info {
    #[clap(subcommand)]
    pub(crate) commands: InfoCommands,
}

#[derive(Args, Clone)]
pub(crate) struct Registry {
    #[clap(subcommand)]
    pub(crate) commands: RegistryCommands,
}

#[derive(Args, Clone)]
pub(crate) struct Upgrade {
    #[clap(subcommand)]
    pub(crate) commands: UpgradeCommands,
}
