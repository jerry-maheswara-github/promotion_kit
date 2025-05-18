use crate::promotion::{DiscountType, Promotion};

/// Calculates the discount amount based on the promotion and the given transaction amount.
///
/// # Arguments
///
/// * `promo` - A reference to the [`Promotion`] containing the discount details.
/// * `amount` - The original transaction amount before any discount is applied.
///
/// # Returns
///
/// The amount to be discounted, calculated based on the type of discount:
/// - For [`DiscountType::Percentage`], it returns `amount * percentage / 100`.
/// - For [`DiscountType::FixedAmount`], it returns the fixed discount, capped at the transaction amount.
///
/// # Example
///
/// ```
/// use promotion_kit::discount::calculate_discount;
/// use promotion_kit::promotion::{DiscountType, Promotion, TargetScope};
/// let promo = Promotion {
///     code: "SAVE10".into(),
///     description: "10% off".into(),
///     discount: DiscountType::Percentage(10.0),
///     usage_limit: None,
///     used: 0,
///     valid_from: 0,
///     valid_until: u64::MAX,
///     min_transaction: None,
///     target: TargetScope::Global,
/// };
///
/// let discount = calculate_discount(&promo, 200.0);
/// assert_eq!(discount, 20.0);
/// ```
pub fn calculate_discount(promo: &Promotion, amount: f64) -> f64 {
    match &promo.discount {
        DiscountType::Percentage(pct) => amount * pct / 100.0,
        DiscountType::FixedAmount(val) => val.min(amount),
    }
}
