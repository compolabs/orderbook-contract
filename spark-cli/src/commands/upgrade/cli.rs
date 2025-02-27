use crate::commands::upgrade::{
    upgrade_fuel_usdc_proxy::UpgradeFuelUsdcProxyCommand,
    upgrade_teth_tusdc_proxy::UpgradeTethTusdcProxyCommand,
};

use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum UpgradeCommands {
    /// Upgrade a fuel/usdc market proxy
    #[clap(short_flag = 'F')]
    UpgradeFuelUsdcProxy(UpgradeFuelUsdcProxyCommand),
    /// Upgrade a fuel/usdc market proxy
    #[clap(short_flag = 'T')]
    UpgradeTethTusdcProxy(UpgradeTethTusdcProxyCommand),
}
