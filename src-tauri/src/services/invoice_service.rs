//! Invoice service for business logic.

use crate::repositories::InvoiceRepository;
use crate::models::Invoice;
use crate::errors::{AppResult, AppError};

/// Service for Invoice business logic
pub struct InvoiceService<'a> {
    repo: InvoiceRepository<'a>,
}

impl<'a> InvoiceService<'a> {
    /// Create a new InvoiceService
    pub fn new(conn: &'a mut rusqlite::Connection) -> Self {
        Self {
            repo: InvoiceRepository::new(conn),
        }
    }
    
    /// Get all invoices with pagination
    pub fn get_all(&self, limit: i32, offset: i32) -> AppResult<Vec<Invoice>> {
        self.repo.get_all(limit, offset)
    }
    
    /// Get an invoice by ID
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<Invoice>> {
        self.repo.get_by_id(id)
    }
    
    /// Get invoices by client ID
    pub fn get_by_client(&self, client_id: i32) -> AppResult<Vec<Invoice>> {
        self.repo.get_by_client(client_id)
    }
    
    /// Create a new invoice
    pub fn create(&mut self, invoice: &Invoice) -> AppResult<i32> {
        self.validate_invoice(invoice)?;
        let mut normalized = invoice.clone();
        self.normalize_items(&mut normalized);
        self.repo.create(&normalized)
    }
    
    /// Update an existing invoice
    pub fn update(&mut self, invoice: &Invoice) -> AppResult<()> {
        self.validate_invoice(invoice)?;
        let mut normalized = invoice.clone();
        self.normalize_items(&mut normalized);
        self.repo.update(&normalized)
    }
    
    /// Compute item totals if not set
    fn normalize_items(&self, invoice: &mut Invoice) {
        for item in &mut invoice.items {
            if item.total_price == rust_decimal::Decimal::ZERO {
                item.calculate_total();
            }
        }
    }
    
    /// Delete an invoice
    pub fn delete(&mut self, id: i32) -> AppResult<()> {
        self.repo.delete(id)
    }
    
    /// Validate invoice data
    fn validate_invoice(&self, invoice: &Invoice) -> AppResult<()> {
        if invoice.client_name.trim().is_empty() {
            return Err(AppError::Validation("Client name is required".to_string()));
        }
        if invoice.items.is_empty() {
            return Err(AppError::Validation("At least one item is required".to_string()));
        }
        for (i, item) in invoice.items.iter().enumerate() {
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
