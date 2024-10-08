use crate::commands::batch::deploy_all::DeployAllCommand;
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum BatchCommands {
    /// Batch Deploy a new market contracts and setup them
    #[clap(short_flag = 'A')]
    DeployAll(DeployAllCommand),
}
