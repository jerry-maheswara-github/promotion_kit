use crate::promotion::Promotion;

pub fn increment_usage(promo: &mut Promotion) {
    promo.used += 1;
}

pub fn reset_usage(promo: &mut Promotion) {
    promo.used = 0;
}
