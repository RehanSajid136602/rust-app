//! Input validation utilities.

/// Validate that a required field is not empty
pub fn validate_required(value: &str, field_name: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{} is required", field_name));
    }
    Ok(())
}

/// Validate email format (simple validation)
pub fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Ok(()); // Empty email is allowed (optional field)
    }
    
    if !email.contains('@') || !email.contains('.') {
        return Err("Invalid email format".to_string());
    }
    
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err("Invalid email format".to_string());
    }
    
    Ok(())
}

/// Validate phone number (Pakistan format)
pub fn validate_phone(phone: &str) -> Result<(), String> {
    if phone.is_empty() {
        return Ok(()); // Empty phone is allowed (optional field)
    }
    
    // Allow formats: 0300-5259751, 03458510130, +92-300-5259751
    let cleaned = phone.replace('-', "").replace(' ', "").replace('+', "");
    
    if cleaned.len() < 10 || cleaned.len() > 15 {
        return Err("Phone number must be 10-15 digits".to_string());
    }
    
    if !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return Err("Phone number must contain only digits".to_string());
    }
    
    Ok(())
}

/// Validate that a value is positive
pub fn validate_positive(value: f64, field_name: &str) -> Result<(), String> {
    if value < 0.0 {
        return Err(format!("{} cannot be negative", field_name));
    }
    Ok(())
}

/// Validate string length
pub fn validate_length(value: &str, field_name: &str, max: usize) -> Result<(), String> {
    if value.len() > max {
        return Err(format!("{} must be less than {} characters", field_name, max));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_required() {
        assert!(validate_required("Test", "Name").is_ok());
        assert!(validate_required("", "Name").is_err());
        assert!(validate_required("   ", "Name").is_err());
    }
    
    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("").is_ok()); // Empty is allowed
    }
    
    #[test]
    fn test_validate_phone() {
        assert!(validate_phone("0300-5259751").is_ok());
        assert!(validate_phone("03458510130").is_ok());
        assert!(validate_phone("123").is_err());
        assert!(validate_phone("").is_ok()); // Empty is allowed
    }
}
