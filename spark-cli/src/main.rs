mod commands;
mod utils;

use clap::Parser;
use commands::{
    cli::{Cli, Command},
    core::cli::CoreCommands,
    info::cli::InfoCommands,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // TODO: check this works against std::env

    let cli = Cli::parse();

    match cli.command {
        Command::Core(args) => match args.commands {
            CoreCommands::Batch(args) => args.run(),
            CoreCommands::Close(args) => args.run(),
            CoreCommands::Deploy(args) => args.run().await,
            CoreCommands::Deposit(args) => args.run().await,
            CoreCommands::Open(args) => args.run(),
            CoreCommands::SetFee(args) => args.run().await,
            CoreCommands::Withdraw(args) => args.run().await,
        },
        Command::Info(args) => match args.commands {
            InfoCommands::Account(args) => args.run().await,
            InfoCommands::Config(args) => args.run().await,
            InfoCommands::Fee(args) => args.run().await,
            InfoCommands::OrderId(args) => args.run().await,
            InfoCommands::Order(args) => args.run().await,
            InfoCommands::UserOrders(args) => args.run().await,
        },
    }
}
