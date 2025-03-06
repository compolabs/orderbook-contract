use crate::commands::upgrade::{
    upgrade_eth_usdc_proxy::UpgradeEthUsdcProxyCommand,
    upgrade_ezeth_usdc_proxy::UpgradeEzethUsdcProxyCommand,
    upgrade_fuel_eth_proxy::UpgradeFuelEthProxyCommand,
    upgrade_fuel_usdc_proxy::UpgradeFuelUsdcProxyCommand,
    upgrade_psycho_usdc_proxy::UpgradePsychoUsdcProxyCommand,
    upgrade_pzeth_usdc_proxy::UpgradePzethUsdcProxyCommand,
    upgrade_teth_tusdc_proxy::UpgradeTethTusdcProxyCommand,
    upgrade_usdc_usdt_proxy::UpgradeUsdcUsdtProxyCommand,
    upgrade_usdf_usdc_proxy::UpgradeUsdfUsdcProxyCommand,
    upgrade_usdt_eth_proxy::UpgradeUsdtEthProxyCommand,
    upgrade_usdt_usdc_proxy::UpgradeUsdtUsdcProxyCommand,
    upgrade_weth_usdc_proxy::UpgradeWethUsdcProxyCommand,
};

use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum UpgradeCommands {
    /// Upgrade a eth/usdc market proxy
    #[clap(short_flag = 'E')]
    UpgradeEthUsdcProxy(UpgradeEthUsdcProxyCommand),
    /// Upgrade a ezeth/usdc market proxy
    #[clap(short_flag = 'Z')]
    UpgradeEzethUsdcProxy(UpgradeEzethUsdcProxyCommand),
    /// Upgrade a fuel/eth market proxy
    #[clap(short_flag = 'D')]
    UpgradeFuelEthProxy(UpgradeFuelEthProxyCommand),
    /// Upgrade a fuel/usdc market proxy
    #[clap(short_flag = 'F')]
    UpgradeFuelUsdcProxy(UpgradeFuelUsdcProxyCommand),
    /// Upgrade a psycho/usdc market proxy
    #[clap(short_flag = 'S')]
    UpgradePsychoUsdcProxy(UpgradePsychoUsdcProxyCommand),
    /// Upgrade a pzeth/usdc market proxy
    #[clap(short_flag = 'P')]
    UpgradePzethUsdcProxy(UpgradePzethUsdcProxyCommand),
    /// Upgrade a teth/tusdc market proxy
    #[clap(short_flag = 'T')]
    UpgradeTethTusdcProxy(UpgradeTethTusdcProxyCommand),
    /// Upgrade a usdc/usdt market proxy
    #[clap(short_flag = 'U')]
    UpgradeUsdcUsdtProxy(UpgradeUsdcUsdtProxyCommand),
    /// Upgrade a usdf/usdc market proxy
    #[clap(short_flag = 'H')]
    UpgradeUsdfUsdcProxy(UpgradeUsdfUsdcProxyCommand),
    /// Upgrade a usdt/eth market proxy
    #[clap(short_flag = 'I')]
    UpgradeUsdtEthProxy(UpgradeUsdtEthProxyCommand),
    /// Upgrade a usdt/usdc market proxy
    #[clap(short_flag = 'V')]
    UpgradeUsdtUsdcProxy(UpgradeUsdtUsdcProxyCommand),
    /// Upgrade a usdt/usdc market proxy
    #[clap(short_flag = 'W')]
    UpgradeWethUsdcProxy(UpgradeWethUsdcProxyCommand),
}
