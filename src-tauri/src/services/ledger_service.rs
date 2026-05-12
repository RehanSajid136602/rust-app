//! Ledger service for business logic.

use crate::repositories::LedgerRepository;
use crate::models::{LedgerEntry, ClientBalance};
use crate::errors::{AppResult, AppError};
use rust_decimal::Decimal;

/// Service for Ledger business logic
pub struct LedgerService<'a> {
    repo: LedgerRepository<'a>,
}

impl<'a> LedgerService<'a> {
    /// Create a new LedgerService
    pub fn new(conn: &'a mut rusqlite::Connection) -> Self {
        Self {
            repo: LedgerRepository::new(conn),
        }
    }
    
    /// Get ledger entries for a client
    pub fn get_client_ledger(&self, client_id: i32) -> AppResult<Vec<LedgerEntry>> {
        self.repo.get_by_client(client_id)
    }
    
    /// Get a single ledger entry by ID
    pub fn get_entry_by_id(&self, id: i32) -> AppResult<Option<LedgerEntry>> {
        self.repo.get_by_id(id)
    }
    
    /// Get client balance summary
    pub fn get_client_balance(&self, client_id: i32) -> AppResult<ClientBalance> {
        self.repo.get_client_balance(client_id)
    }
    
    /// Get all client balances
    pub fn get_all_balances(&self) -> AppResult<Vec<ClientBalance>> {
        self.repo.get_all_balances()
    }
    
    /// Add a manual debit entry (increase balance)
    pub fn add_debit(
        &mut self,
        client_id: i32,
        amount: Decimal,
        description: &str,
        date: &str,
    ) -> AppResult<i32> {
        if amount <= Decimal::ZERO {
            return Err(AppError::Validation("Debit amount must be positive".to_string()));
        }
        
        self.repo.add_debit(client_id, date.to_string(), description.to_string(), amount, None)
    }
    
    /// Add a manual credit entry (decrease balance)
    pub fn add_credit(
        &mut self,
        client_id: i32,
        amount: Decimal,
        description: &str,
        date: &str,
    ) -> AppResult<i32> {
        if amount <= Decimal::ZERO {
            return Err(AppError::Validation("Credit amount must be positive".to_string()));
        }
        
        self.repo.add_credit(client_id, date.to_string(), description.to_string(), amount, None)
    }
    
    /// Get clients with outstanding balance
    pub fn get_clients_with_balance(&self) -> AppResult<Vec<ClientBalance>> {
        let all = self.repo.get_all_balances()?;
        Ok(all.into_iter()
            .filter(|b| b.current_balance != Decimal::ZERO)
            .collect())
    }
}
