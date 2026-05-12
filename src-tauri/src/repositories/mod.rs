//! Repositories for data access.
//! 
//! This module contains repository implementations for:
//! - ProductRepository
//! - ClientRepository
//! - InvoiceRepository
//! - QuotationRepository
//! - SettingsRepository
//! - LedgerRepository

pub mod product_repo;
pub mod client_repo;
pub mod invoice_repo;
pub mod quotation_repo;
pub mod settings_repo;
pub mod ledger_repo;

pub use product_repo::ProductRepository;
pub use client_repo::ClientRepository;
pub use invoice_repo::InvoiceRepository;
pub use quotation_repo::QuotationRepository;
pub use settings_repo::SettingsRepository;
pub use ledger_repo::LedgerRepository;
