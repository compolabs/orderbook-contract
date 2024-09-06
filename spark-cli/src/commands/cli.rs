use crate::commands::{
    core::cli::CoreCommands, info::cli::InfoCommands, registry::cli::RegistryCommands,
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
    #[clap(short_flag = 'R')]
    Registry(Registry),

    ///
    #[clap(short_flag = 'C')]
    Core(Core),

    ///
    #[clap(short_flag = 'I')]
    Info(Info),
}

#[derive(Args, Clone)]
pub(crate) struct Registry {
    #[clap(subcommand)]
    pub(crate) commands: RegistryCommands,
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
