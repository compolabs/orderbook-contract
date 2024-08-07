mod commands;
mod utils;

use clap::Parser;
use commands::{
    book::cli::BookCommands,
    cli::{Cli, Command},
    core::cli::CoreCommands,
    info::cli::InfoCommands,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // TODO: check this works against std::env

    // TODO: document local provider rpc in each command
    // Provider::connect(format!("127.0.0.1:{port}")).await?;

    let cli = Cli::parse();

    match cli.command {
        Command::Book(args) => match args.commands {
            BookCommands::Config(args) => args.run().await,
            BookCommands::Deploy(args) => args.run().await,
            BookCommands::Markets(args) => args.run().await,
            BookCommands::Register(args) => args.run().await,
            BookCommands::Unregister(args) => args.run().await,
        },
        Command::Core(args) => match args.commands {
            CoreCommands::Cancel(args) => args.run().await,
            CoreCommands::Deploy(args) => args.run().await,
            CoreCommands::Deposit(args) => args.run().await,
            CoreCommands::FulfillMany(args) => args.run().await,
            CoreCommands::Open(args) => args.run().await,
            CoreCommands::MatchMany(args) => args.run().await,
            CoreCommands::MatchPair(args) => args.run().await,
            CoreCommands::SetProtocolFee(args) => args.run().await,
            CoreCommands::SetMatcherFee(args) => args.run().await,
            CoreCommands::WithdrawProtocolFee(args) => args.run().await,
            CoreCommands::Withdraw(args) => args.run().await,
        },
        Command::Info(args) => match args.commands {
            InfoCommands::Account(args) => args.run().await,
            InfoCommands::Config(args) => args.run().await,
            InfoCommands::ProtocolFee(args) => args.run().await,
            InfoCommands::TotalProtocolFee(args) => args.run().await,
            InfoCommands::ProtocolFeeAmount(args) => args.run().await,
            InfoCommands::MatcherFee(args) => args.run().await,
            InfoCommands::OrderId(args) => args.run().await,
            InfoCommands::Order(args) => args.run().await,
            InfoCommands::UserOrders(args) => args.run().await,
        },
    }
}
