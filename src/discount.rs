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
pub fn calculate_discount(promo: &Promotion, amount: f64) -> f64 {
    match &promo.discount {
        DiscountType::Percentage(pct) => amount * pct / 100.0,
        DiscountType::FixedAmount(val) => val.min(amount),
    }
}
