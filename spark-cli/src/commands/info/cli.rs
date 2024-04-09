use crate::commands::info::{
    account::AccountCommand, config::ConfigCommand, fee::FeeCommand, order::OrderCommand,
    order_id::OrderIdCommand, user_orders::UserOrdersCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum InfoCommands {
    ///
    #[clap(short_flag = 'A')]
    Account(AccountCommand),

    ///
    #[clap(short_flag = 'C')]
    Config(ConfigCommand),

    ///
    #[clap(short_flag = 'F')]
    Fee(FeeCommand),

    ///
    #[clap(short_flag = 'I')]
    OrderId(OrderIdCommand),

    ///
    #[clap(short_flag = 'O')]
    Order(OrderCommand),

    ///
    #[clap(short_flag = 'U')]
    UserOrders(UserOrdersCommand),
}
