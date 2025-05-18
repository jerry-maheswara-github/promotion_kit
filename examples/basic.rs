use promotion_kit::promotion::{Promotion, DiscountType, TargetScope};
use promotion_kit::service::apply_promotion;

fn main() {
    let promo = Promotion {
        code: "WELCOME10".into(),
        description: "10% off for new users".into(),
        discount: DiscountType::Percentage(10.0),
        usage_limit: Some(100),
        used: 0,
        valid_from: 1_700_000_000,
        valid_until: 1_800_000_000,
        min_transaction: Some(100.0),
        target: TargetScope::Global,
        currency: Some("IDR".to_string()),
    };

    let discount = apply_promotion(&promo, 200.0, 1_750_000_000).unwrap();
    assert_eq!(discount, 20.0);
}
