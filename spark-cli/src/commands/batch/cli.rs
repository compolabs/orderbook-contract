use crate::commands::batch::{
    deploy_all::DeployAllCommand, deploy_eth_usdc_proxy::DeployEthUsdcProxyCommand,
    deploy_ezeth_usdc_proxy::DeployEzethUsdcProxyCommand,
    deploy_fuel_eth_proxy::DeployFuelEthProxyCommand,
    deploy_fuel_usdc_proxy::DeployFuelUsdcProxyCommand, deploy_proxy::DeployProxyCommand,
    deploy_psycho_usdc_proxy::DeployPsychoUsdcProxyCommand,
    deploy_pzeth_usdc_proxy::DeployPzethUsdcProxyCommand,
    deploy_teth_tusdc_impl::DeployTethTusdcImplCommand,
    deploy_teth_tusdc_proxy::DeployTethTusdcProxyCommand,
    deploy_trump_eth_proxy::DeployTrumpEthProxyCommand,
    deploy_usdc_usdt_proxy::DeployUsdcUsdtProxyCommand,
    deploy_usdf_usdc_proxy::DeployUsdfUsdcProxyCommand,
    deploy_usdt_eth_proxy::DeployUsdtEthProxyCommand,
    deploy_usdt_usdc_proxy::DeployUsdtUsdcProxyCommand,
    deploy_weth_usdc_proxy::DeployWethUsdcProxyCommand,
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

    /// Deploy a new ezeth/usdc market proxy
    #[clap(short_flag = 'H')]
    DeployEzethUsdcProxy(DeployEzethUsdcProxyCommand),

    /// Deploy a new fuel/usdc market proxy
    #[clap(short_flag = 'G')]
    DeployFuelEthProxy(DeployFuelEthProxyCommand),

    /// Deploy a new fuel/usdc market proxy
    #[clap(short_flag = 'F')]
    DeployFuelUsdcProxy(DeployFuelUsdcProxyCommand),

    /// Deploy a new market and proxy contracts and setup them
    #[clap(short_flag = 'P')]
    DeployProxy(DeployProxyCommand),

    /// Deploy a new market and proxy contracts and setup them
    #[clap(short_flag = 'Y')]
    DeployPsychoUsdcProxy(DeployPsychoUsdcProxyCommand),

    /// Deploy a new ezeth/usdc market proxy
    #[clap(short_flag = 'Q')]
    DeployPzethUsdcProxy(DeployPzethUsdcProxyCommand),

    /// Deploy a new teth/tusdc market implementtion
    #[clap(short_flag = 'I')]
    DeployTethTusdcImpl(DeployTethTusdcImplCommand),

    /// Deploy a new teth/tusdc market and proxy contracts and setup them
    #[clap(short_flag = 'T')]
    DeployTethTusdcProxy(DeployTethTusdcProxyCommand),

    /// Deploy a new trump/eth market and proxy contracts and setup them
    #[clap(short_flag = 'X')]
    DeployTrumpEthProxy(DeployTrumpEthProxyCommand),

    /// Deploy a new usdc/usdt market proxy
    #[clap(short_flag = 'U')]
    DeployUsdcUsdtProxy(DeployUsdcUsdtProxyCommand),

    /// Deploy a new usdf/usdc market proxy
    #[clap(short_flag = 'J')]
    DeployUsdfUsdcProxy(DeployUsdfUsdcProxyCommand),

    /// Deploy a new usdt/usdc market proxy
    #[clap(short_flag = 'Z')]
    DeployUsdtEthProxy(DeployUsdtEthProxyCommand),

    /// Deploy a new usdt/usdc market proxy
    #[clap(short_flag = 'V')]
    DeployUsdtUsdcProxy(DeployUsdtUsdcProxyCommand),

    /// Deploy a new usdt/usdc market proxy
    #[clap(short_flag = 'W')]
    DeployWethUsdcProxy(DeployWethUsdcProxyCommand),
}
