use crate::commands::core::{
    cancel_order::CancelCommand, cancel_small_order::CancelSmallCommand, deploy::DeployCommand,
    deposit::DepositCommand, deposit_for::DepositForCommand, fulfill_many::FulfillManyCommand,
    match_many::MatchManyCommand, open_market_order::OpenMarketCommand, open_order::OpenCommand,
    set_epoch::SetEpochCommand, set_matcher_fee::SetMatcherFeeCommand,
    set_min_order_price::SetMinOrderPriceCommand, set_min_order_size::SetMinOrderSizeCommand,
    set_paused::SetPausedCommand, set_protocol_fee::SetProtocolFeeCommand,
    set_proxy_target::SetProxyTargetCommand, withdraw::WithdrawCommand,
    withdraw_to_market::WithdrawToMarketCommand,
};
use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub(crate) enum CoreCommands {
    /// Cancel an open order
    #[clap(short_flag = 'C')]
    Cancel(CancelCommand),

    /// Cancel an open order
    #[clap(short_flag = 'F')]
    CancelSmall(CancelSmallCommand),

    /// Deploy a new market contract
    #[clap(short_flag = 'D')]
    Deploy(DeployCommand),

    /// Deposit into the market contract for user
    #[clap(short_flag = 'E')]
    DepositFor(DepositForCommand),

    /// Deposit into the market contract
    #[clap(short_flag = 'P')]
    Deposit(DepositCommand),

    /// Fulfill multiple orders
    #[clap(short_flag = 'F')]
    FulfillMany(FulfillManyCommand),

    /// Match multiple orders
    #[clap(short_flag = 'M')]
    MatchMany(MatchManyCommand),

    /// Open an order
    #[clap(short_flag = 'N')]
    OpenMarket(OpenMarketCommand),

    /// Open an order
    #[clap(short_flag = 'O')]
    Open(OpenCommand),

    /// Set a protocol fee
    #[clap(short_flag = 'E')]
    SetEpoch(SetEpochCommand),

    /// Set a minimum order price for the market
    #[clap(short_flag = 'P')]
    SetMinOrderPrice(SetMinOrderPriceCommand),

    /// Set a minimum order price for the market
    #[clap(short_flag = 'Q')]
    SetPaused(SetPausedCommand),

    /// Set a protocol fee
    #[clap(short_flag = 'S')]
    SetProtocolFee(SetProtocolFeeCommand),

    /// Set a matcher fee for the market
    #[clap(short_flag = 'T')]
    SetMatcherFee(SetMatcherFeeCommand),

    /// Set a minimum order size for the market
    #[clap(short_flag = 'V')]
    SetMinOrderSize(SetMinOrderSizeCommand),

    /// Withdraw from the market contract
    #[clap(short_flag = 'W')]
    Withdraw(WithdrawCommand),

    /// Withdraw from the market contract
    #[clap(short_flag = 'X')]
    WithdrawToMarket(WithdrawToMarketCommand),

    /// Set a proxy target market contract
    #[clap(short_flag = 'Z')]
    SetProxyTarget(SetProxyTargetCommand),
}
