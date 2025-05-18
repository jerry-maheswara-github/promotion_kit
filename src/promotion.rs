/// Type of discount being applied.
#[derive(Debug, Clone, PartialEq)]
pub enum DiscountType {
    /// Percentage-based discount, e.g. 10.0 = 10%
    Percentage(f64),

    /// Fixed amount discount, e.g. 5000 = 5000 units of currency
    FixedAmount(f64),
}

/// Target category where the promotion can be applied.
///
/// Users of the library can define their own categories (e.g. "product", "service", etc.)
#[derive(Debug, Clone, PartialEq)]
pub enum TargetScope {
    /// Promotion is applicable to any item or category
    Global,

    /// Promotion is restricted to a specific label, tag, or category
    Specific(String),
}

/// Represents a generic promotion or discount campaign.
#[derive(Debug, Clone)]
pub struct Promotion {
    /// Unique code used to apply the promotion
    pub code: String,

    /// Human-readable description
    pub description: String,

    /// Discount mechanics
    pub discount: DiscountType,

    /// Optional maximum number of times the promo can be used
    pub usage_limit: Option<u32>,

    /// Current number of times the promo has been used
    pub used: u32,

    /// Validity start time (epoch seconds)
    pub valid_from: u64,

    /// Validity end time (epoch seconds)
    pub valid_until: u64,

    /// Minimum required transaction value to apply the promo
    pub min_transaction: Option<f64>,

    /// Category or scope where this promo is valid
    pub target: TargetScope,

    /// The currency used for the promotion (e.g., "USD", "EUR")
    /// This field could be optional or have a default currency.
    pub currency: Option<String>,
}
