use crate::commands::info::{
    account::AccountCommand, config::ConfigCommand, epoch::EpochCommand,
    market_order::MarketOrderCommand, matcher_fee::MatcherFeeCommand,
    min_order_price::MinOrderPriceCommand, min_order_size::MinOrderSizeCommand,
    order::OrderCommand, order_id::OrderIdCommand, paused::PausedCommand,
    protocol_fee::ProtocolFeeCommand, protocol_fee_user::ProtocolFeeUserCommand,
    protocol_fee_user_amount::ProtocolFeeUserAmountCommand, proxy_owner::ProxyOwnerCommand,
    proxy_target::ProxyTargetCommand, user_orders::UserOrdersCommand,
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

    /// Query minimum order size of the market contract
    #[clap(short_flag = 'N')]
    MinOrderSize(MinOrderSizeCommand),

    /// Query minimum order price of the market contract
    #[clap(short_flag = 'P')]
    MinOrderPrice(MinOrderPriceCommand),

    /// Calculate the order id given the provided arguments
    #[clap(short_flag = 'I')]
    OrderId(OrderIdCommand),

    /// Query market order information
    #[clap(short_flag = 'M')]
    MarketOrder(MarketOrderCommand),

    /// Query order information
    #[clap(short_flag = 'O')]
    Order(OrderCommand),

    /// Query paused market state
    #[clap(short_flag = 'P')]
    Paused(PausedCommand),

    /// Query information for a proxy target
    #[clap(short_flag = 'T')]
    ProxyTarget(ProxyTargetCommand),

    /// Query orders associated with an
    #[clap(short_flag = 'U')]
    UserOrders(UserOrdersCommand),

    /// Query information for a proxy owner
    #[clap(short_flag = 'V')]
    ProxyOwner(ProxyOwnerCommand),
}
