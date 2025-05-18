#[cfg(test)]
mod tests {
    use promotion_kit::promotion::{DiscountType, Promotion, TargetScope};
    use promotion_kit::service::{increment_usage, reset_usage};

    /// Helper function to create a basic promotion for testing
    fn create_promo() -> Promotion {
        Promotion {
            code: "PROMO10".to_string(),
            description: "10% discount".to_string(),
            discount: DiscountType::Percentage(10.0),
            usage_limit: Some(5),
            used: 0,
            valid_from: 1_700_000_000,
            valid_until: 1_800_000_000,
            min_transaction: Some(50.0),
            target: TargetScope::Global,
            currency: Some("USD".to_string()),
        }
    }

    #[test]
    fn test_increment_usage() {
        // Prepare a promo with initial usage count of 0
        let mut promo = create_promo();

        // Increment the usage
        increment_usage(&mut promo);

        // Assert that the usage count is now 1
        assert_eq!(promo.used, 1);

        // Increment the usage again
        increment_usage(&mut promo);

        // Assert that the usage count is now 2
        assert_eq!(promo.used, 2);
    }

    #[test]
    fn test_reset_usage() {
        // Prepare a promo with initial usage count of 5
        let mut promo = create_promo();
        promo.used = 5;

        // Reset the usage count
        reset_usage(&mut promo);

        // Assert that the usage count is now 0
        assert_eq!(promo.used, 0);
    }

    #[test]
    fn test_reset_usage_when_zero() {
        // Prepare a promo with initial usage count of 0
        let mut promo = create_promo();

        // Reset the usage count (should not change as it's already 0)
        reset_usage(&mut promo);

        // Assert that the usage count is still 0
        assert_eq!(promo.used, 0);
    }
}
