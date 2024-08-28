use crate::commands::info::{
    account::AccountCommand, config::ConfigCommand, epoch::EpochCommand,
    matcher_fee::MatcherFeeCommand, order::OrderCommand, order_id::OrderIdCommand,
    protocol_fee::ProtocolFeeCommand, protocol_fee_user::ProtocolFeeUserCommand,
    protocol_fee_user_amount::ProtocolFeeUserAmountCommand, user_orders::UserOrdersCommand,
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

    /// Query epoch information of the market contract
    #[clap(short_flag = 'W')]
    Epoch(EpochCommand),

    /// Query protocol fee information
    #[clap(short_flag = 'F')]
    ProtocolFee(ProtocolFeeCommand),

    /// Query fee information for a specific user and amount of the market contract
    #[clap(short_flag = 'E')]
    ProtocolFeeUserAmount(ProtocolFeeUserAmountCommand),

    /// Query fee information for a specific user of the market contract
    #[clap(short_flag = 'B')]
    ProtocolFeeUser(ProtocolFeeUserCommand),

    /// Query matcher fee information of the market contract
    #[clap(short_flag = 'M')]
    MatcherFee(MatcherFeeCommand),

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
