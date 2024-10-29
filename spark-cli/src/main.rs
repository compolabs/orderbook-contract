mod commands;
mod utils;

use clap::Parser;
use commands::{
    batch::cli::BatchCommands,
    cli::{Cli, Command},
    core::cli::CoreCommands,
    info::cli::InfoCommands,
    registry::cli::RegistryCommands,
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
            BatchCommands::DeployProxy(args) => args.run().await,
        },
        Command::Core(args) => match args.commands {
            CoreCommands::Cancel(args) => args.run().await,
            CoreCommands::Deploy(args) => args.run().await,
            CoreCommands::Deposit(args) => args.run().await,
            CoreCommands::DepositFor(args) => args.run().await,
            CoreCommands::FulfillMany(args) => args.run().await,
            CoreCommands::Open(args) => args.run().await,
            CoreCommands::MatchMany(args) => args.run().await,
            CoreCommands::MatchPair(args) => args.run().await,
            CoreCommands::SetEpoch(args) => args.run().await,
            CoreCommands::SetProtocolFee(args) => args.run().await,
            CoreCommands::SetMatcherFee(args) => args.run().await,
            CoreCommands::SetMinOrderSize(args) => args.run().await,
            CoreCommands::SetStoreOrderChangeInfo(args) => args.run().await,
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
            InfoCommands::MinOrderSize(args) => args.run().await,
            InfoCommands::OrderId(args) => args.run().await,
            InfoCommands::Order(args) => args.run().await,
            InfoCommands::StoreOrderChangeInfo(args) => args.run().await,
            InfoCommands::UserOrders(args) => args.run().await,
        },
        Command::Registry(args) => match args.commands {
            RegistryCommands::Config(args) => args.run().await,
            RegistryCommands::Deploy(args) => args.run().await,
            RegistryCommands::Markets(args) => args.run().await,
            RegistryCommands::Register(args) => args.run().await,
            RegistryCommands::Unregister(args) => args.run().await,
        },
    }
}
