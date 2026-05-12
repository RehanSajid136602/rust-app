//! Quotation and QuotationItem models.

use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// A line item in a quotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotationItem {
    pub id: Option<i32>,
    pub quotation_id: Option<i32>,
    pub sno: i32,
    pub item_name: String,
    pub quantity: Decimal,
    pub price_per_unit: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_price: Decimal,
}

impl QuotationItem {
    /// Create a new QuotationItem with default values
    pub fn new(sno: i32) -> Self {
        Self {
            id: None,
            quotation_id: None,
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
    pub fn calculate_total(&mut self) {
        let base = self.quantity * self.price_per_unit;
        let after_discount = base - self.discount_amount;
        self.total_price = after_discount + self.tax_amount;
    }
}

/// Calculated totals for a quotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotationTotals {
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub grand_total: Decimal,
    pub adjustment_amount: Decimal,
    pub net_amount: Decimal,
}

impl QuotationTotals {
    /// Calculate totals from a list of quotation items
    pub fn from_items(items: &[QuotationItem], adjustment: Decimal) -> Self {
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

/// Main quotation document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    pub id: Option<i32>,
    pub quotation_number: String,
    pub ref_number: String,
    pub client_id: Option<i32>,
    pub client_name: String,
    pub client_address: String,
    pub quotation_date: String,
    pub valid_until: String,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub discount_total: Decimal,
    pub grand_total: Decimal,
    pub adjustment_label: String,
    pub adjustment_amount: Decimal,
    pub total: Decimal,
    pub notes: String,
    pub status: String,
    pub created_at: Option<String>,
    #[serde(default)]
    pub items: Vec<QuotationItem>,
}

impl Quotation {
    /// Create a new Quotation with default values
    pub fn new() -> Self {
        Self {
            id: None,
            quotation_number: String::new(),
            ref_number: String::new(),
            client_id: None,
            client_name: String::new(),
            client_address: String::new(),
            quotation_date: String::new(),
            valid_until: String::new(),
            subtotal: Decimal::ZERO,
            tax_total: Decimal::ZERO,
            discount_total: Decimal::ZERO,
            grand_total: Decimal::ZERO,
            adjustment_label: String::new(),
            adjustment_amount: Decimal::ZERO,
            total: Decimal::ZERO,
            notes: String::new(),
            status: "draft".to_string(),
            created_at: None,
            items: Vec::new(),
        }
    }
    
    /// Recalculate all totals based on items
    pub fn recalculate_totals(&mut self) {
        let totals = QuotationTotals::from_items(&self.items, self.adjustment_amount);
        
        self.subtotal = totals.subtotal;
        self.tax_total = totals.tax_total;
        self.discount_total = totals.discount_total;
        self.grand_total = totals.grand_total;
        self.total = totals.net_amount;
    }
}

impl Default for Quotation {
    fn default() -> Self {
        Self::new()
    }
}

/// Request DTO for creating a quotation
#[derive(Debug, Clone, Deserialize)]
pub struct CreateQuotationRequest {
    pub ref_number: String,
    pub client_id: Option<i32>,
    pub client_name: String,
    pub client_address: String,
    pub quotation_date: String,
    pub valid_until: String,
    pub adjustment_label: String,
    pub adjustment_amount: Decimal,
    pub notes: String,
    pub items: Vec<CreateQuotationItemRequest>,
}

/// Request DTO for creating a quotation item
#[derive(Debug, Clone, Deserialize)]
pub struct CreateQuotationItemRequest {
    pub item_name: String,
    pub quantity: Decimal,
    pub price_per_unit: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
}

/// Response DTO for quotation operations
#[derive(Debug, Clone, Serialize)]
pub struct QuotationResponse {
    pub id: i32,
    pub quotation_number: String,
    pub success: bool,
    pub message: String,
}
