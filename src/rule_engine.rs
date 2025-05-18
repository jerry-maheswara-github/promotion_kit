use crate::promotion::Promotion;

/// Context used to evaluate whether a promotion is currently valid
pub struct PromoContext<'a> {
    pub current_time: u64,
    pub transaction_amount: f64,
    pub promo: &'a Promotion,
}

/// Simple boolean validation for use in filters or lightweight checks
pub fn is_promo_valid(ctx: &PromoContext) -> bool {
    let promo = ctx.promo;

    // Validate logical time period
    if promo.valid_from >= promo.valid_until {
        return false;
    }

    // Validate current time is within the promo period
    if ctx.current_time < promo.valid_from || ctx.current_time > promo.valid_until {
        return false;
    }

    // Validate transaction amount is sufficient
    if let Some(min_tx) = promo.min_transaction {
        if ctx.transaction_amount < min_tx {
            return false;
        }
    }

    // Validate usage limit
    if let Some(limit) = promo.usage_limit {
        if promo.used >= limit {
            return false;
        }
    }

    true
}
