//! Product model.

use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// Product model for the catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub price_per_unit: Decimal,
    pub unit: String,
    pub hsn_code: String,
    pub created_at: Option<String>,
}

impl Product {
    /// Create a new Product with default values
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::new(),
            price_per_unit: Decimal::ZERO,
            unit: "pcs".to_string(),
            hsn_code: String::new(),
            created_at: None,
        }
    }
    
    /// Validate the product data
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Product name is required".to_string());
        }
        if self.name.len() < 2 {
            return Err("Product name must be at least 2 characters".to_string());
        }
        if self.price_per_unit < Decimal::ZERO {
            return Err("Price cannot be negative".to_string());
        }
        Ok(())
    }
}

impl Default for Product {
    fn default() -> Self {
        Self::new()
    }
}

/// Request DTO for creating a product
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price_per_unit: Decimal,
    pub unit: String,
    pub hsn_code: String,
}

/// Request DTO for updating a product
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProductRequest {
    pub name: String,
    pub price_per_unit: Decimal,
    pub unit: String,
    pub hsn_code: String,
}

/// Response DTO for product operations
#[derive(Debug, Clone, Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub success: bool,
    pub message: String,
}

/// Search result for product autocomplete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSearchResult {
    pub id: i32,
    pub name: String,
    pub price_per_unit: Decimal,
    pub unit: String,
    pub score: u8,
}
