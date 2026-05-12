//! Settings service for business logic.

use crate::repositories::SettingsRepository;
use crate::models::CompanySettings;
use crate::errors::{AppResult, AppError};

/// Service for Company Settings business logic
pub struct SettingsService<'a> {
    repo: SettingsRepository<'a>,
}

impl<'a> SettingsService<'a> {
    /// Create a new SettingsService
    pub fn new(conn: &'a mut rusqlite::Connection) -> Self {
        Self {
            repo: SettingsRepository::new(conn),
        }
    }
    
    /// Get company settings
    pub fn get(&self) -> AppResult<CompanySettings> {
        self.repo.get()
    }
    
    /// Update company settings
    pub fn update(&mut self, settings: &CompanySettings) -> AppResult<()> {
        self.validate_settings(settings)?;
        self.repo.update(settings)
    }
    
    /// Get next invoice number
    pub fn get_next_invoice_number(&self, year: u32) -> AppResult<String> {
        self.repo.get_next_invoice_number(year)
    }
    
    /// Get next quotation number
    pub fn get_next_quotation_number(&self, year: u32) -> AppResult<String> {
        self.repo.get_next_quotation_number(year)
    }
    
    /// Validate settings data
    fn validate_settings(&self, settings: &CompanySettings) -> AppResult<()> {
        if settings.company_name.trim().is_empty() {
            return Err(AppError::Validation("Company name is required".to_string()));
        }
        Ok(())
    }
}
