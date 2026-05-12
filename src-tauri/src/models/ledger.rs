//! Ledger entry model for client transaction history.

use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// Ledger entry for tracking client transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: Option<i32>,
    pub client_id: i32,
    pub date: String,
    pub description: String,
    pub debit: Decimal,
    pub credit: Decimal,
    pub balance: Decimal,
    pub invoice_id: Option<i32>,
}

impl LedgerEntry {
    /// Create a new ledger entry
    pub fn new(client_id: i32, date: String, description: String) -> Self {
        Self {
            id: None,
            client_id,
            date,
            description,
            debit: Decimal::ZERO,
            credit: Decimal::ZERO,
            balance: Decimal::ZERO,
            invoice_id: None,
        }
    }
    
    /// Create a debit entry (increase balance - invoice)
    pub fn debit(client_id: i32, date: String, description: String, amount: Decimal, invoice_id: Option<i32>) -> Self {
        Self {
            id: None,
            client_id,
            date,
            description,
            debit: amount,
            credit: Decimal::ZERO,
            balance: Decimal::ZERO,
            invoice_id,
        }
    }
    
    /// Create a credit entry (decrease balance - payment)
    pub fn credit(client_id: i32, date: String, description: String, amount: Decimal, invoice_id: Option<i32>) -> Self {
        Self {
            id: None,
            client_id,
            date,
            description,
            debit: Decimal::ZERO,
            credit: amount,
            balance: Decimal::ZERO,
            invoice_id,
        }
    }
}

/// Client balance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientBalance {
    pub client_id: i32,
    pub client_name: String,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub current_balance: Decimal,
}

impl ClientBalance {
    pub fn new(client_id: i32, client_name: String) -> Self {
        Self {
            client_id,
            client_name,
            total_debit: Decimal::ZERO,
            total_credit: Decimal::ZERO,
            current_balance: Decimal::ZERO,
        }
    }
}
