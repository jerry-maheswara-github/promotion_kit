//! # promotion_kit
//!
//! A powerful Rust toolkit for managing dynamic promotions with flexible rules and seamless integration.
//!
//! This crate provides a lightweight and extensible toolkit for managing promotions,
//! vouchers, and discounts in any transactional system, such as e-commerce, SaaS,
//! digital products, or other service platforms.
//!
//! ---
//! 
//! ## ✅ Features
//!
//! - Define flexible promotions with percentage or fixed amount discounts
//! - Configure minimum transaction amounts, usage limits, and validity periods
//! - Apply promotions and calculate discounts using a clean, composable API
//! - Track usage counts and validate conditions with a minimal rule engine
//!
//! ---
//! 
//! ## ✨ Quick Start
//!
//! ```rust
//!  use promotion_kit::promotion::{Promotion, DiscountType, TargetScope};
//!  use promotion_kit::service::apply_promotion;
//! 
//!  let promo = Promotion {
//!     code: "WELCOME10".into(),
//!     description: "10% off for new users".into(),
//!     discount: DiscountType::Percentage(10.0),
//!     usage_limit: Some(100),
//!     used: 0,
//!     valid_from: 1_700_000_000,
//!     valid_until: 1_800_000_000,
//!     min_transaction: Some(100.0),
//!     target: TargetScope::Global,
//!     currency: Some("IDR".to_string()),
//!  };
//! 
//!  let discount = apply_promotion(&promo, 200.0, 1_750_000_000).unwrap();
//!  assert_eq!(discount, 20.0);
//! ```
//!
//! ---
//!
//! ## 📄 License
//!
//! Licensed under the [Apache-2.0 license](http://www.apache.org/licenses/LICENSE-2.0.txt)
//!
//! ---
//!
//! ## 👨 Author
//!
//! Jerry Maheswara <jerrymaheswara@gmail.com>
//!
//! ---
//!
//! ## ❤️ Built with Love in Rust
//!
//! This project is built with ❤️ using **Rust** — a systems programming language that is safe, fast, and concurrent.  
//! Rust is the perfect choice for building reliable and efficient applications.
//!
//! ---
//!
//! ## 🤝 Contributing
//!
//! Pull requests, issues, and feedback are welcome!  
//! If you find this crate useful, give it a ⭐ and share it with others in the Rustacean community.
//!

/// Contains the main application logic for validating and applying promotions.
///
/// This is the entry point for consuming systems to evaluate if a promotion can be applied.
pub mod service;

/// Defines all errors related to promotion validation and application.
///
/// Uses the `thiserror` crate for ergonomic error handling and clear descriptions.
pub mod error;

/// Contains the core data structures for defining a promotion and discount rules.
///
/// Includes `Promotion`, `DiscountType`, and `TargetScope` types.
pub mod promotion;

/// Implements the logic for calculating the monetary value of a discount
/// based on the provided promotion and transaction amount.
pub mod discount;

/// Provides a lightweight rule engine and context model for validating promotions.
///
/// Used internally by the service layer and externally for basic checks like filtering valid promos.
pub mod rule_engine;
