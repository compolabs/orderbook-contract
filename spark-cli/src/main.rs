mod commands;
mod utils;

use clap::Parser;
use commands::{
    batch::cli::BatchCommands,
    cli::{Cli, Command},
    core::cli::CoreCommands,
    info::cli::InfoCommands,
    registry::cli::RegistryCommands,
    upgrade::cli::UpgradeCommands,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let version: String = env!("CARGO_PKG_VERSION").into();
    println!("Spark CLI v{}", version);

    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Command::Batch(args) => match args.commands {
            BatchCommands::DeployAll(args) => args.run().await,
            BatchCommands::DeployEthUsdcProxy(args) => args.run().await,
            BatchCommands::DeployEzethUsdcProxy(args) => args.run().await,
            BatchCommands::DeployPzethUsdcProxy(args) => args.run().await,
            BatchCommands::DeployFuelEthProxy(args) => args.run().await,
            BatchCommands::DeployFuelUsdcProxy(args) => args.run().await,
            BatchCommands::DeployProxy(args) => args.run().await,
            BatchCommands::DeployPsychoUsdcProxy(args) => args.run().await,
            BatchCommands::DeployTethTusdcImpl(args) => args.run().await,
            BatchCommands::DeployTethTusdcProxy(args) => args.run().await,
            BatchCommands::DeployTrumpEthProxy(args) => args.run().await,
            BatchCommands::DeployUsdcUsdtProxy(args) => args.run().await,
            BatchCommands::DeployUsdfUsdcProxy(args) => args.run().await,
            BatchCommands::DeployUsdtEthProxy(args) => args.run().await,
            BatchCommands::DeployUsdtUsdcProxy(args) => args.run().await,
            BatchCommands::DeployWethUsdcProxy(args) => args.run().await,
        },
        Command::Core(args) => match args.commands {
            CoreCommands::Cancel(args) => args.run().await,
            CoreCommands::CancelSmall(args) => args.run().await,
            CoreCommands::Deploy(args) => args.run().await,
            CoreCommands::Deposit(args) => args.run().await,
            CoreCommands::DepositFor(args) => args.run().await,
            CoreCommands::FulfillMany(args) => args.run().await,
            CoreCommands::Open(args) => args.run().await,
            CoreCommands::OpenMarket(args) => args.run().await,
            CoreCommands::MatchMany(args) => args.run().await,
            CoreCommands::SetEpoch(args) => args.run().await,
            CoreCommands::SetProtocolFee(args) => args.run().await,
            CoreCommands::SetMatcherFee(args) => args.run().await,
            CoreCommands::SetMinOrderPrice(args) => args.run().await,
            CoreCommands::SetMinOrderSize(args) => args.run().await,
            CoreCommands::SetPaused(args) => args.run().await,
            CoreCommands::SetProxyTarget(args) => args.run().await,
            CoreCommands::Withdraw(args) => args.run().await,
            CoreCommands::WithdrawToMarket(args) => args.run().await,
        },
        Command::Info(args) => match args.commands {
            InfoCommands::Account(args) => args.run().await,
            InfoCommands::Config(args) => args.run().await,
            InfoCommands::Epoch(args) => args.run().await,
            InfoCommands::ProtocolFee(args) => args.run().await,
            InfoCommands::ProtocolFeeUser(args) => args.run().await,
            InfoCommands::ProtocolFeeUserAmount(args) => args.run().await,
            InfoCommands::MatcherFee(args) => args.run().await,
            InfoCommands::MinOrderPrice(args) => args.run().await,
            InfoCommands::MinOrderSize(args) => args.run().await,
            InfoCommands::OrderId(args) => args.run().await,
            InfoCommands::MarketOrder(args) => args.run().await,
            InfoCommands::Order(args) => args.run().await,
            InfoCommands::Paused(args) => args.run().await,
            InfoCommands::ProxyOwner(args) => args.run().await,
            InfoCommands::ProxyTarget(args) => args.run().await,
            InfoCommands::UserOrders(args) => args.run().await,
        },
        Command::Registry(args) => match args.commands {
            RegistryCommands::Config(args) => args.run().await,
            RegistryCommands::Deploy(args) => args.run().await,
            RegistryCommands::Markets(args) => args.run().await,
            RegistryCommands::Register(args) => args.run().await,
            RegistryCommands::Unregister(args) => args.run().await,
        },
        Command::Upgrade(args) => match args.commands {
            UpgradeCommands::UpgradeFuelUsdcProxy(args) => args.run().await,
            UpgradeCommands::UpgradeTethTusdcProxy(args) => args.run().await,
        },
    }
}
