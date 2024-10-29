use crate::commands::batch::{deploy_all::DeployAllCommand, deploy_proxy::DeployProxyCommand};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum BatchCommands {
    /// Batch Deploy a new market contracts and setup them
    #[clap(short_flag = 'A')]
    DeployAll(DeployAllCommand),

    /// Deploy a new market and proxy contracts and setup them
    #[clap(short_flag = 'P')]
    DeployProxy(DeployProxyCommand),
}
