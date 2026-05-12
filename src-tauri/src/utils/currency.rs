//! Currency formatting and parsing utilities.

use rust_decimal::Decimal;

/// Format a decimal value as currency (Rs. X.XX)
pub fn format_currency(amount: Decimal) -> String {
    // Rust doesn't support grouping in format strings, so we format manually
    let formatted = amount.to_string();
    format!("Rs. {}", formatted)
}

/// Parse a currency string to Decimal
/// Accepts formats: "Rs. 1,234.56", "1,234.56", "1234.56"
pub fn parse_currency(s: &str) -> Result<Decimal, String> {
    let cleaned = s
        .replace("Rs.", "")
        .replace("Rs", "")
        .replace(',', "")
        .replace(' ', "")
        .trim()
        .to_string();
    
    cleaned.parse::<Decimal>().map_err(|e| format!("Invalid currency format: {}", e))
}

/// Convert cents to decimal (e.g., 1234 -> 12.34)
pub fn cents_to_decimal(cents: i64) -> Decimal {
    Decimal::new(cents, 2)
}

/// Round to 2 decimal places (standard currency rounding)
pub fn round_currency(value: Decimal) -> Decimal {
    value.round_dp(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_currency() {
        let amount = Decimal::new(123456, 2); // 1234.56
        assert_eq!(format_currency(amount), "Rs. 1,234.56");
    }
    
    #[test]
    fn test_parse_currency() {
        assert_eq!(parse_currency("Rs. 1,234.56").unwrap(), Decimal::new(123456, 2));
        assert_eq!(parse_currency("1,234.56").unwrap(), Decimal::new(123456, 2));
        assert_eq!(parse_currency("1234.56").unwrap(), Decimal::new(123456, 2));
    }
    
    #[test]
    fn test_round_currency() {
        let value = Decimal::new(1234567, 3); // 1234.567
        assert_eq!(round_currency(value), Decimal::new(123457, 2)); // 1234.57
    }
}
