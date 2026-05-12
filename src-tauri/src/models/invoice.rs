//! Invoice and InvoiceItem models.

use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// Payment status for an invoice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Unpaid,
    Partial,
    Paid,
}

impl PaymentStatus {
    /// Compute payment status based on amount paid vs grand total
    pub fn from_amounts(amount_paid: Decimal, grand_total: Decimal) -> Self {
        if grand_total <= Decimal::ZERO {
            return Self::Paid;
        }
        if amount_paid <= Decimal::ZERO {
            return Self::Unpaid;
        }
        if amount_paid >= grand_total {
            return Self::Paid;
        }
        Self::Partial
    }
    
    /// Convert to string for JSON serialization
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unpaid => "unpaid",
            Self::Partial => "partial",
            Self::Paid => "paid",
        }
    }
}

/// A line item in an invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: Option<i32>,
    pub invoice_id: Option<i32>,
    pub sno: i32,
    pub item_name: String,
    pub quantity: Decimal,
    pub price_per_unit: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_price: Decimal,
}

impl InvoiceItem {
    /// Create a new InvoiceItem with default values
    pub fn new(sno: i32) -> Self {
        Self {
            id: None,
            invoice_id: None,
            sno,
            item_name: String::new(),
            quantity: Decimal::ONE,
            price_per_unit: Decimal::ZERO,
            discount_amount: Decimal::ZERO,
            tax_amount: Decimal::ZERO,
            total_price: Decimal::ZERO,
        }
    }
    
    /// Calculate the total price for this item
    /// Formula: (quantity * price) - discount + tax
    pub fn calculate_total(&mut self) {
        let base = self.quantity * self.price_per_unit;
        let after_discount = base - self.discount_amount;
        self.total_price = after_discount + self.tax_amount;
    }
}

/// Calculated totals for an invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceTotals {
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub grand_total: Decimal,
    pub adjustment_amount: Decimal,
    pub net_amount: Decimal,
}

impl InvoiceTotals {
    /// Calculate totals from a list of invoice items
    pub fn from_items(items: &[InvoiceItem], adjustment: Decimal) -> Self {
        let mut subtotal = Decimal::ZERO;
        let mut tax_total = Decimal::ZERO;
        let mut discount_total = Decimal::ZERO;
        
        for item in items {
            let base = item.quantity * item.price_per_unit;
            subtotal += base;
            discount_total += item.discount_amount;
            tax_total += item.tax_amount;
        }
        
        let grand_total = subtotal - discount_total + tax_total;
        let net_amount = grand_total - adjustment;
        
        Self {
            subtotal,
            tax_total,
            discount_total,
            grand_total,
            adjustment_amount: adjustment,
            net_amount,
        }
    }
}

/// Main invoice document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Option<i32>,
    pub invoice_number: String,
    pub ref_number: String,
    pub client_id: Option<i32>,
    pub client_name: String,
    pub client_address: String,
    pub invoice_date: String,
    pub due_date: String,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub grand_total: Decimal,
    pub amount_paid: Decimal,
    pub remaining_debt: Decimal,
    pub adjustment_label: String,
    pub adjustment_amount: Decimal,
    pub total: Decimal,
    pub notes: String,
    pub status: PaymentStatus,
    pub created_at: Option<String>,
    #[serde(default)]
    pub items: Vec<InvoiceItem>,
}

impl Invoice {
    /// Create a new Invoice with default values
    pub fn new() -> Self {
        Self {
            id: None,
            invoice_number: String::new(),
            ref_number: String::new(),
            client_id: None,
            client_name: String::new(),
            client_address: String::new(),
            invoice_date: String::new(),
            due_date: String::new(),
            subtotal: Decimal::ZERO,
            tax_total: Decimal::ZERO,
            discount_total: Decimal::ZERO,
            grand_total: Decimal::ZERO,
            amount_paid: Decimal::ZERO,
            remaining_debt: Decimal::ZERO,
            adjustment_label: String::new(),
            adjustment_amount: Decimal::ZERO,
            total: Decimal::ZERO,
            notes: String::new(),
            status: PaymentStatus::Unpaid,
            created_at: None,
            items: Vec::new(),
        }
    }
    
    /// Recalculate all totals based on items
    pub fn recalculate_totals(&mut self) {
        let totals = InvoiceTotals::from_items(&self.items, self.adjustment_amount);
        
        self.subtotal = totals.subtotal;
        self.tax_total = totals.tax_total;
        self.discount_total = totals.discount_total;
        self.grand_total = totals.grand_total;
        self.total = totals.net_amount;
        self.remaining_debt = self.total - self.amount_paid;
        self.status = PaymentStatus::from_amounts(self.amount_paid, self.total);
    }
}

impl Default for Invoice {
    fn default() -> Self {
        Self::new()
    }
}

/// Request DTO for creating an invoice
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInvoiceRequest {
    pub ref_number: String,
    pub client_id: Option<i32>,
    pub client_name: String,
    pub client_address: String,
    pub invoice_date: String,
    pub due_date: String,
    pub adjustment_label: String,
    pub adjustment_amount: Decimal,
    pub amount_paid: Decimal,
    pub notes: String,
    pub items: Vec<CreateInvoiceItemRequest>,
}

/// Request DTO for creating an invoice item
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInvoiceItemRequest {
    pub item_name: String,
    pub quantity: Decimal,
    pub price_per_unit: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
}

/// Response DTO for invoice operations
#[derive(Debug, Clone, Serialize)]
pub struct InvoiceResponse {
    pub id: i32,
    pub invoice_number: String,
    pub success: bool,
    pub message: String,
}
