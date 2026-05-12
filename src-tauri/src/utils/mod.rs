//! Utility functions for the application.

pub mod currency;
pub mod validation;

// Re-export utilities
pub use currency::{format_currency, parse_currency, cents_to_decimal};
pub use validation::{validate_email, validate_phone, validate_required};
