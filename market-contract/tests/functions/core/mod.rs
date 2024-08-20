mod cancel_order;
mod deposit;
mod fulfill_order_many;
mod match_order_many;
mod match_order_pair;
mod open_order;
mod set_matcher_fee;
mod set_protocol_fee;
mod withdraw;
mod withdraw_protocol_fees;

/// From market-contract/src/math.sw
/// Converts between base and quote amounts with the appropriate scaling based on decimals.
///
/// # Parameters:
/// - `amount`: The amount to convert.
/// - `base_asset_decimals`: The decimals for the base asset.
/// - `base_price`: The price of the base asset.
/// - `price_decimals`: The decimals for the price.
/// - `quote_asset_decimals`: The decimals for the quote asset.
/// - `base_to_quote`: If `true`, converts from base to quote, otherwise from quote to base.
///
/// # Returns:
/// The converted amount as a `u64`.
pub fn convert(
    amount: u64,
    base_asset_decimals: u32,
    base_price: u64,
    price_decimals: u32,
    quote_asset_decimals: u32,
    base_to_quote: bool,
) -> u64 {
    let op1 = base_price as u128;
    let op2 = 10_u128.pow(base_asset_decimals + price_decimals - quote_asset_decimals);

    if base_to_quote {
        // Convert from base to quote
        (amount as u128)
            .saturating_mul(op1)
            .saturating_div(op2)
            .try_into()
            .unwrap_or(0)
    } else {
        // Convert from quote to base
        (amount as u128)
            .saturating_mul(op2)
            .saturating_div(op1)
            .try_into()
            .unwrap_or(0)
    }
}
