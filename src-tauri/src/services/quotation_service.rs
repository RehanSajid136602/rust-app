//! Quotation service for business logic.

use crate::repositories::QuotationRepository;
use crate::models::Quotation;
use crate::errors::{AppResult, AppError};

/// Service for Quotation business logic
pub struct QuotationService<'a> {
    repo: QuotationRepository<'a>,
}

impl<'a> QuotationService<'a> {
    /// Create a new QuotationService
    pub fn new(conn: &'a mut rusqlite::Connection) -> Self {
        Self {
            repo: QuotationRepository::new(conn),
        }
    }
    
    /// Get all quotations with pagination
    pub fn get_all(&self, limit: i32, offset: i32) -> AppResult<Vec<Quotation>> {
        self.repo.get_all(limit, offset)
    }
    
    /// Get a quotation by ID
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<Quotation>> {
        self.repo.get_by_id(id)
    }
    
    /// Create a new quotation
    pub fn create(&mut self, quotation: &Quotation) -> AppResult<i32> {
        self.validate_quotation(quotation)?;
        self.repo.create(quotation)
    }
    
    /// Update an existing quotation
    pub fn update(&mut self, quotation: &Quotation) -> AppResult<()> {
        self.validate_quotation(quotation)?;
        self.repo.update(quotation)
    }
    
    /// Delete a quotation
    pub fn delete(&mut self, id: i32) -> AppResult<()> {
        self.repo.delete(id)
    }
    
    /// Validate quotation data
    fn validate_quotation(&self, quotation: &Quotation) -> AppResult<()> {
        if quotation.client_name.trim().is_empty() {
            return Err(AppError::Validation("Client name is required".to_string()));
        }
        if quotation.items.is_empty() {
            return Err(AppError::Validation("At least one item is required".to_string()));
        }
        for (i, item) in quotation.items.iter().enumerate() {
            if item.item_name.trim().is_empty() {
                return Err(AppError::Validation(
                    format!("Item {} name is required", i + 1)
                ));
            }
            if item.quantity <= rust_decimal::Decimal::ZERO {
                return Err(AppError::Validation(
                    format!("Item {} quantity must be positive", i + 1)
                ));
            }
            if item.price_per_unit < rust_decimal::Decimal::ZERO {
                return Err(AppError::Validation(
                    format!("Item {} price cannot be negative", i + 1)
                ));
            }
        }
        Ok(())
    }
}
