//! Domain models for the invoice application.
//! 
//! These structs represent the core business entities and are used
//! for serialization/deserialization between Rust and the frontend.

pub mod invoice;
pub mod quotation;
pub mod client;
pub mod product;
pub mod settings;
pub mod ledger;

// Re-export all models for easier imports
pub use invoice::{Invoice, InvoiceItem, InvoiceTotals, PaymentStatus, CreateInvoiceRequest, CreateInvoiceItemRequest};
pub use quotation::{Quotation, QuotationItem, QuotationTotals, CreateQuotationRequest, CreateQuotationItemRequest};
pub use client::{Client, CreateClientRequest};
pub use product::{Product, CreateProductRequest};
pub use settings::CompanySettings;
pub use ledger::{LedgerEntry, ClientBalance};
