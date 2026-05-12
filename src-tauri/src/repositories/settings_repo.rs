//! Settings repository for database operations.

use rusqlite::Connection;
use crate::models::CompanySettings;
use crate::errors::{AppResult, AppError};

/// Repository for CompanySettings data access (singleton)
pub struct SettingsRepository<'a> {
    conn: &'a Connection,
}

impl<'a> SettingsRepository<'a> {
    /// Create a new SettingsRepository
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
    
    /// Get company settings (singleton - always id=1)
    pub fn get(&self) -> AppResult<CompanySettings> {
        let settings = self.conn.query_row(
            "SELECT company_name, tagline, ntn_number, office_address, phone1, phone2, email,
                    logo_path, invoice_prefix, next_invoice_number, quotation_prefix, 
                    next_quotation_number, salutation, body_text, banner_color, footer_color
             FROM company_settings WHERE id = 1",
            [],
            |row| {
                Ok(CompanySettings {
                    id: 1,
                    company_name: row.get(0)?,
                    tagline: row.get(1)?,
                    ntn_number: row.get(2)?,
                    office_address: row.get(3)?,
                    phone1: row.get(4)?,
                    phone2: row.get(5)?,
                    email: row.get(6)?,
                    logo_path: row.get(7)?,
                    invoice_prefix: row.get(8)?,
                    next_invoice_number: row.get(9)?,
                    quotation_prefix: row.get(10)?,
                    next_quotation_number: row.get(11)?,
                    salutation: row.get(12)?,
                    body_text: row.get(13)?,
                    banner_color: row.get(14)?,
                    footer_color: row.get(15)?,
                })
            },
        );
        
        match settings {
            Ok(s) => Ok(s),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Return defaults if no settings exist
                Ok(CompanySettings::default_settings())
            },
            Err(e) => Err(AppError::Database(e)),
        }
    }
    
    /// Update company settings
    pub fn update(&self, settings: &CompanySettings) -> AppResult<()> {
        // Filter out fields that might not exist in the DB table yet
        self.conn.execute(
            "UPDATE company_settings SET 
                company_name = ?, tagline = ?, ntn_number = ?, office_address = ?,
                phone1 = ?, phone2 = ?, email = ?, logo_path = ?, invoice_prefix = ?,
                quotation_prefix = ?, salutation = ?, body_text = ?, banner_color = ?,
                footer_color = ?
            WHERE id = 1",
            (
                &settings.company_name, &settings.tagline, &settings.ntn_number,
                &settings.office_address, &settings.phone1, &settings.phone2,
                &settings.email, &settings.logo_path, &settings.invoice_prefix,
                &settings.quotation_prefix, &settings.salutation, &settings.body_text,
                &settings.banner_color, &settings.footer_color,
            ),
        )?;
        
        Ok(())
    }
    
    /// Get next invoice number and increment it
    pub fn get_next_invoice_number(&self, year: u32) -> AppResult<String> {
        let settings = self.get()?;
        let prefix = &settings.invoice_prefix;
        let pattern = format!("{}-{}-%", prefix, year);
        
        // Get max invoice number for this year
        let max_num: Option<i32> = self.conn.query_row(
            "SELECT COALESCE(MAX(CAST(SUBSTR(invoice_number, -4) AS INTEGER)), 0) + 1
             FROM invoices WHERE invoice_number LIKE ?",
            [&pattern],
            |row| row.get(0),
        ).ok();
        
        let next_num = max_num.unwrap_or(settings.next_invoice_number);
        
        Ok(format!("{}-{}-{:04}", prefix, year, next_num))
    }
    
    /// Get next quotation number and increment it
    pub fn get_next_quotation_number(&self, year: u32) -> AppResult<String> {
        let settings = self.get()?;
        let prefix = &settings.quotation_prefix;
        let pattern = format!("{}-{}-%", prefix, year);
        
        // Get max quotation number for this year
        let max_num: Option<i32> = self.conn.query_row(
            "SELECT COALESCE(MAX(CAST(SUBSTR(quotation_number, -4) AS INTEGER)), 0) + 1
             FROM quotations WHERE quotation_number LIKE ?",
            [&pattern],
            |row| row.get(0),
        ).ok();
        
        let next_num = max_num.unwrap_or(settings.next_quotation_number);
        
        Ok(format!("{}-{}-{:04}", prefix, year, next_num))
    }
}
