use thiserror::Error;

#[derive(Debug, Error)]
pub enum PromoError {
    #[error("Promotion is not valid at this time")]
    InvalidTime,

    #[error("Transaction amount is below the required minimum")]
    BelowMinimumTransaction,

    #[error("Promotion usage limit has been reached")]
    UsageLimitReached,

    #[error("Promotion is not applicable")]
    NotApplicable,

    #[error("Promotion period is invalid (valid_from must be earlier than valid_until)")]
    InvalidPeriod,
}
