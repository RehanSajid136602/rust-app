//! Client model.

use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use crate::utils::validation;

/// Client/Customer model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: Option<i32>,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub gstin: String,
    pub balance: Decimal,
}

impl Client {
    /// Create a new Client with default values
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::new(),
            address: String::new(),
            phone: String::new(),
            email: String::new(),
            gstin: String::new(),
            balance: Decimal::ZERO,
        }
    }
    
    /// Validate the client data
    pub fn validate(&self) -> Result<(), String> {
        validation::validate_required(&self.name, "Client name")?;
        
        if self.name.len() > 200 {
            return Err("Client name must be less than 200 characters".to_string());
        }
        
        if !self.email.is_empty() {
            validation::validate_email(&self.email)
                .map_err(|e| format!("Invalid email: {}", e))?;
        }
        
        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// Request DTO for creating a client
#[derive(Debug, Clone, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub gstin: String,
}
