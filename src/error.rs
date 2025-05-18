use thiserror::Error;

/// Represents the possible errors that can occur when validating or applying a promotion.
#[derive(Debug, Error, PartialEq)]
pub enum PromoError {
    /// The current time is outside the promotion's valid time window.
    #[error("Promotion is not valid at this time")]
    InvalidTime,

    /// The transaction amount is below the minimum threshold required to apply the promotion.
    #[error("Transaction amount is below the required minimum")]
    BelowMinimumTransaction,

    /// The promotion has reached its maximum allowed usage limit.
    #[error("Promotion usage limit has been reached")]
    UsageLimitReached,

    /// The promotion is not applicable in the current context (e.g., mismatched scope or conditions).
    #[error("Promotion is not applicable")]
    NotApplicable,

    /// The promotion has an invalid period: `valid_from` is greater than or equal to `valid_until`.
    #[error("Promotion period is invalid (valid_from must be earlier than valid_until)")]
    InvalidPeriod,
}
