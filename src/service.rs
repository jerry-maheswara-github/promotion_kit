use crate::discount::calculate_discount;
use crate::error::PromoError;
use crate::promotion::Promotion;
use crate::rule_engine::PromoContext;

/// Applies a promotion to a given transaction, if all conditions are satisfied.
///
/// This function performs validation on:
/// - Logical time range (`valid_from` < `valid_until`)
/// - Current time within validity period
/// - Minimum transaction threshold
/// - Usage limits
///
/// If valid, it returns the discount amount to apply.
///
/// # Arguments
///
/// * `promo` - A reference to the [`Promotion`] to apply.
/// * `transaction_amount` - The original transaction value before any discounts.
/// * `now` - The current timestamp (in seconds) used to validate the promotion period.
///
/// # Returns
///
/// A [`Result`] containing the discount value on success, or a [`PromoError`] if validation fails.
///
/// # Errors
///
/// - [`PromoError::InvalidPeriod`]: `valid_from >= valid_until`
/// - [`PromoError::InvalidTime`]: Current time is outside the promo validity window
/// - [`PromoError::BelowMinimumTransaction`]: Transaction is below `min_transaction`
/// - [`PromoError::UsageLimitReached`]: Promo has reached its maximum allowed usage
pub fn apply_promotion(
    promo: &Promotion,
    transaction_amount: f64,
    now: u64,
) -> Result<f64, PromoError> {
    let ctx = PromoContext {
        current_time: now,
        transaction_amount,
        promo,
    };

    // Logical time range check
    if ctx.promo.valid_from >= ctx.promo.valid_until {
        return Err(PromoError::InvalidPeriod);
    }

    // Actual time window check
    if ctx.current_time < ctx.promo.valid_from || ctx.current_time > ctx.promo.valid_until {
        return Err(PromoError::InvalidTime);
    }

    // Minimum transaction amount
    if let Some(min_tx) = ctx.promo.min_transaction {
        if ctx.transaction_amount < min_tx {
            return Err(PromoError::BelowMinimumTransaction);
        }
    }

    // Usage limit check
    if let Some(limit) = ctx.promo.usage_limit {
        if ctx.promo.used >= limit {
            return Err(PromoError::UsageLimitReached);
        }
    }

    // If all validations pass, calculate and return discount
    Ok(calculate_discount(ctx.promo, ctx.transaction_amount))
}

/// Increments the usage count of a promotion by 1.
///
/// This function is used to track how many times a promotion has been used. 
/// It ensures that the `used` field of the `Promotion` struct is incremented
/// correctly each time a promotion is applied.
pub fn increment_usage(promo: &mut Promotion) {
    promo.used += 1;
}

/// Resets the usage count of a promotion to 0.
///
/// This function is useful when you need to reset the usage count for a promotion,
/// either for testing purposes or when the promotion's usage limit is reset.
pub fn reset_usage(promo: &mut Promotion) {
    promo.used = 0;
}