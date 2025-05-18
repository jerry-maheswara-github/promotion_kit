#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};
    use promotion_kit::error::PromoError;
    use promotion_kit::promotion::{DiscountType, Promotion, TargetScope};
    use promotion_kit::service::apply_promotion;

    fn get_current_time() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    #[test]
    fn test_valid_promotion_percentage() {
        // Prepare a valid promotion
        let promo = Promotion {
            code: "SAVE10".to_string(),
            description: "10% off on orders".to_string(),
            discount: DiscountType::Percentage(10.0),
            usage_limit: Some(100),
            used: 0,
            valid_from: 1_700_000_000,
            valid_until: 1_800_000_000,
            min_transaction: Some(50.0),
            target: TargetScope::Global,
            currency: Some("USD".to_string()),
        };

        let transaction_amount = 100.0;
        let now = get_current_time();

        let result = apply_promotion(&promo, transaction_amount, now);

        // Assert the discount applied is correct
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10.0); // 10% of 100 is 10
    }

    #[test]
    fn test_invalid_time() {
        // Prepare a promotion that is no longer valid
        let promo = Promotion {
            code: "SAVE10".to_string(),
            description: "10% off on orders".to_string(),
            discount: DiscountType::Percentage(10.0),
            usage_limit: Some(100),
            used: 0,
            valid_from: 1_700_000_000,
            valid_until: 1_800_000_000,
            min_transaction: Some(50.0),
            target: TargetScope::Global,
            currency: Some("USD".to_string()),
        };

        let transaction_amount = 100.0;
        let now = 1_900_000_000; // Current time is after `valid_until`

        let result = apply_promotion(&promo, transaction_amount, now);

        // Assert that an error is returned due to invalid time
        assert_eq!(result, Err(PromoError::InvalidTime));
    }

    #[test]
    fn test_below_minimum_transaction() {
        // Prepare a promotion with a minimum transaction amount
        let promo = Promotion {
            code: "SAVE10".to_string(),
            description: "10% off on orders".to_string(),
            discount: DiscountType::Percentage(10.0),
            usage_limit: Some(100),
            used: 0,
            valid_from: 1_700_000_000,
            valid_until: 1_800_000_000,
            min_transaction: Some(150.0),
            target: TargetScope::Global,
            currency: Some("USD".to_string()),
        };

        let transaction_amount = 100.0; // Below the minimum required amount
        let now = get_current_time();

        let result = apply_promotion(&promo, transaction_amount, now);

        // Assert that the error is returned because the transaction is too low
        assert_eq!(result, Err(PromoError::BelowMinimumTransaction));
    }

    #[test]
    fn test_usage_limit_reached() {
        // Prepare a promotion with a usage limit
        let promo = Promotion {
            code: "SAVE10".to_string(),
            description: "10% off on orders".to_string(),
            discount: DiscountType::Percentage(10.0),
            usage_limit: Some(2), // Limit is 2
            used: 2, // Already used the maximum
            valid_from: 1_700_000_000,
            valid_until: 1_800_000_000,
            min_transaction: Some(50.0),
            target: TargetScope::Global,
            currency: Some("USD".to_string()),
        };

        let transaction_amount = 100.0;
        let now = get_current_time();

        let result = apply_promotion(&promo, transaction_amount, now);

        // Assert that the usage limit has been reached
        assert_eq!(result, Err(PromoError::UsageLimitReached));
    }

    #[test]
    fn test_invalid_period() {
        // Prepare a promotion with an invalid period
        let promo = Promotion {
            code: "SAVE10".to_string(),
            description: "10% off on orders".to_string(),
            discount: DiscountType::Percentage(10.0),
            usage_limit: Some(100),
            used: 0,
            valid_from: 1_800_000_000,
            valid_until: 1_700_000_000, // Invalid period (valid_from > valid_until)
            min_transaction: Some(50.0),
            target: TargetScope::Global,
            currency: Some("USD".to_string()),
        };

        let transaction_amount = 100.0;
        let now = get_current_time();

        let result = apply_promotion(&promo, transaction_amount, now);

        // Assert that the period is invalid
        assert_eq!(result, Err(PromoError::InvalidPeriod));
    }

    #[test]
    fn test_valid_fixed_amount_promotion() {
        // Prepare a fixed amount promotion
        let promo = Promotion {
            code: "FLAT50".to_string(),
            description: "Flat 50 off".to_string(),
            discount: DiscountType::FixedAmount(50.0),
            usage_limit: Some(100),
            used: 0,
            valid_from: 1_700_000_000,
            valid_until: 1_800_000_000,
            min_transaction: Some(50.0),
            target: TargetScope::Global,
            currency: Some("USD".to_string()),
        };

        let transaction_amount = 200.0;
        let now = get_current_time();

        let result = apply_promotion(&promo, transaction_amount, now);

        // Assert the discount applied is 50 (flat amount)
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 50.0);
    }
}
