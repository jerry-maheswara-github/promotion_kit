use promotion_kit::error::PromoError;
use promotion_kit::promotion::{Promotion, DiscountType, TargetScope};
use promotion_kit::service::apply_promotion;

fn main() -> Result<(), PromoError> {
    let promo = Promotion {
        code: "WELCOME10".into(),
        description: "10% off for new users".into(),
        discount: DiscountType::Percentage(10.0),
        usage_limit: Some(100),
        used: 10,
        valid_from: 1_700_000_000,
        valid_until: 1_800_000_000,
        min_transaction: Some(100.0),
        target: TargetScope::Global,
        currency: Some("USD".to_string()),
    };
    
    match apply_promotion(&promo, 200.0, 1_750_000_000) {
        Ok(discount) => {
            println!("Promotion applied! Discount: {} {}", promo.currency.as_deref().unwrap_or(""), discount);
        },
        Err(e) => {
            eprintln!("Failed to apply promotion: {}", e);
        }
    }

    println!("{:#?}", promo);
    
    Ok(())

}
