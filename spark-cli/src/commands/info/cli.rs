use crate::commands::info::{
    account::AccountCommand, config::ConfigCommand, matcher_fee::MatcherFeeCommand,
    order::OrderCommand, order_id::OrderIdCommand, protocol_fee::ProtocolFeeCommand,
    protocol_fee_amount::ProtocolFeeAmountCommand, total_protocol_fee::TotalProtocolFeeCommand,
    user_orders::UserOrdersCommand,
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

    /// Query protocol fee information
    #[clap(short_flag = 'F')]
    ProtocolFee(ProtocolFeeCommand),

    /// Query total collected protocol fee information
    #[clap(short_flag = 'T')]
    TotalProtocolFee(TotalProtocolFeeCommand),

    /// Query fee information for a specific user or the market contract
    #[clap(short_flag = 'E')]
    ProtocolFeeAmount(ProtocolFeeAmountCommand),

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
