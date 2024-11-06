use crate::commands::batch::{
    deploy_all::DeployAllCommand, deploy_eth_usdc_proxy::DeployEthUsdcProxyCommand,
    deploy_proxy::DeployProxyCommand, deploy_teth_tusdc_impl::DeployTethTusdcImplCommand,
    deploy_teth_tusdc_proxy::DeployTethTusdcProxyCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum BatchCommands {
    /// Batch Deploy a new market contracts and setup them
    #[clap(short_flag = 'A')]
    DeployAll(DeployAllCommand),

    /// Deploy a new eth/usdc market proxy
    #[clap(short_flag = 'E')]
    DeployEthUsdcProxy(DeployEthUsdcProxyCommand),

    /// Deploy a new market and proxy contracts and setup them
    #[clap(short_flag = 'P')]
    DeployProxy(DeployProxyCommand),

    /// Deploy a new teth/tusdc market implementtion
    #[clap(short_flag = 'I')]
    DeployTethTusdcImpl(DeployTethTusdcImplCommand),

    /// Deploy a new teth/tusdc market and proxy contracts and setup them
    #[clap(short_flag = 'T')]
    DeployTethTusdcProxy(DeployTethTusdcProxyCommand),
}
