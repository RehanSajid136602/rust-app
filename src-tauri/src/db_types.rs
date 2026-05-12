//! Database type conversions for Decimal.
//!
//! Helper functions for converting between f64 (SQLite REAL) and Decimal.

use rust_decimal::Decimal;

/// Convert f64 from SQLite to Decimal
#[inline]
pub fn f64_to_decimal(f: f64) -> Decimal {
    Decimal::try_from(f).unwrap_or(Decimal::ZERO)
}

/// Convert Decimal to f64 for SQLite REAL storage
#[inline]
pub fn decimal_to_f64(d: &Decimal) -> f64 {
    d.to_string().parse::<f64>().unwrap_or(0.0)
}
