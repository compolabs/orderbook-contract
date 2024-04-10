use crate::commands::info::{
    account::AccountCommand, config::ConfigCommand, fee::FeeCommand, order::OrderCommand,
    order_id::OrderIdCommand, user_orders::UserOrdersCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum InfoCommands {
    /// Query account information
    #[clap(short_flag = 'A')]
    Account(AccountCommand),

    /// Query configuration information for a market contract
    #[clap(short_flag = 'C')]
    Config(ConfigCommand),

    /// Query fee information for a specific user or the market contract
    #[clap(short_flag = 'F')]
    Fee(FeeCommand),

    /// Calculate the order id given the provided arguments
    #[clap(short_flag = 'I')]
    OrderId(OrderIdCommand),

    /// Query order information
    #[clap(short_flag = 'O')]
    Order(OrderCommand),

    /// Query orders associated with an
    #[clap(short_flag = 'U')]
    UserOrders(UserOrdersCommand),
}
