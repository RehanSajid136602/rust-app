//! Services layer for business logic.
//! 
//! This module contains service implementations that wrap repositories
//! and provide business logic for:
//! - ProductService
//! - ClientService
//! - InvoiceService
//! - QuotationService
//! - SettingsService
//! - LedgerService

pub mod product_service;
pub mod client_service;
pub mod invoice_service;
pub mod quotation_service;
pub mod settings_service;
pub mod ledger_service;

pub use product_service::ProductService;
pub use client_service::ClientService;
pub use invoice_service::InvoiceService;
pub use quotation_service::QuotationService;
pub use settings_service::SettingsService;
pub use ledger_service::LedgerService;
