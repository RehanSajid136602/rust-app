//! Ledger repository for client transaction history and balance calculations.

use rusqlite::Connection;
use crate::models::{LedgerEntry, ClientBalance};
use crate::errors::{AppResult, AppError};
use crate::db_types::{f64_to_decimal, decimal_to_f64};
use rust_decimal::Decimal;

/// Repository for Ledger data access
pub struct LedgerRepository<'a> {
    conn: &'a Connection,
}

impl<'a> LedgerRepository<'a> {
    /// Create a new LedgerRepository
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
    
    /// Get all ledger entries for a client
    pub fn get_by_client(&self, client_id: i32) -> AppResult<Vec<LedgerEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, client_id, date, description, debit, credit, balance, invoice_id 
             FROM client_ledgers 
             WHERE client_id = ? 
             ORDER BY date, id"
        )?;
        
        let entries = stmt.query_map([client_id], |row| {
            Ok(LedgerEntry {
                id: row.get(0)?,
                client_id: row.get(1)?,
                date: row.get(2)?,
                description: row.get(3)?,
                debit: f64_to_decimal(row.get(4)?),
                credit: f64_to_decimal(row.get(5)?),
                balance: f64_to_decimal(row.get(6)?),
                invoice_id: row.get(7)?,
            })
        })?;
        
        let mut result = Vec::new();
        for entry in entries {
            result.push(entry?);
        }
        
        Ok(result)
    }
    
    /// Get a single ledger entry by ID
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<LedgerEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, client_id, date, description, debit, credit, balance, invoice_id 
             FROM client_ledgers 
             WHERE id = ?"
        )?;
        
        let entry = stmt.query_row([id], |row| {
            Ok(LedgerEntry {
                id: row.get(0)?,
                client_id: row.get(1)?,
                date: row.get(2)?,
                description: row.get(3)?,
                debit: f64_to_decimal(row.get(4)?),
                credit: f64_to_decimal(row.get(5)?),
                balance: f64_to_decimal(row.get(6)?),
                invoice_id: row.get(7)?,
            })
        });
        
        match entry {
            Ok(e) => Ok(Some(e)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e)),
        }
    }
    
    /// Create a new ledger entry
    pub fn create(&self, entry: &LedgerEntry) -> AppResult<i32> {
        self.conn.execute(
            "INSERT INTO client_ledgers (client_id, date, description, debit, credit, balance, invoice_id) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            (
                entry.client_id,
                &entry.date,
                &entry.description,
                decimal_to_f64(&entry.debit),
                decimal_to_f64(&entry.credit),
                decimal_to_f64(&entry.balance),
                entry.invoice_id,
            ),
        )?;
        
        Ok(self.conn.last_insert_rowid() as i32)
    }
    
    /// Add a debit entry (increase balance - typically from an invoice)
    pub fn add_debit(
        &self, 
        client_id: i32, 
        date: String, 
        description: String, 
        amount: Decimal, 
        invoice_id: Option<i32>
    ) -> AppResult<i32> {
        let entry = LedgerEntry::debit(client_id, date, description, amount, invoice_id);
        self.create(&entry)
    }
    
    /// Add a credit entry (decrease balance - typically from a payment)
    pub fn add_credit(
        &self, 
        client_id: i32, 
        date: String, 
        description: String, 
        amount: Decimal, 
        invoice_id: Option<i32>
    ) -> AppResult<i32> {
        let entry = LedgerEntry::credit(client_id, date, description, amount, invoice_id);
        self.create(&entry)
    }
    
    /// Get current balance for a client
    pub fn get_client_balance(&self, client_id: i32) -> AppResult<ClientBalance> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                SUM(COALESCE(debit, 0)) as total_debit,
                SUM(COALESCE(credit, 0)) as total_credit
             FROM client_ledgers 
             WHERE client_id = ?"
        )?;
        
        let (total_debit, total_credit) = stmt.query_row([client_id], |row| {
            let debit: f64 = row.get(0)?;
            let credit: f64 = row.get(1)?;
            Ok((f64_to_decimal(debit), f64_to_decimal(credit)))
        }).unwrap_or((Decimal::ZERO, Decimal::ZERO));
        
        let current_balance = total_debit - total_credit;
        
        // Get client name
        let client_name = self.get_client_name(client_id)?;
        
        let mut balance = ClientBalance::new(client_id, client_name);
        balance.total_debit = total_debit;
        balance.total_credit = total_credit;
        balance.current_balance = current_balance;
        
        Ok(balance)
    }
    
    /// Get all client balances
    pub fn get_all_balances(&self) -> AppResult<Vec<ClientBalance>> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                c.id,
                c.name,
                SUM(COALESCE(l.debit, 0)) as total_debit,
                SUM(COALESCE(l.credit, 0)) as total_credit
             FROM clients c
             LEFT JOIN client_ledgers l ON c.id = l.client_id
             GROUP BY c.id, c.name
             ORDER BY c.name"
        )?;
        
        let balances = stmt.query_map([], |row| {
            let client_id: i32 = row.get(0)?;
            let client_name: String = row.get(1)?;
            let total_debit: f64 = row.get(2)?;
            let total_credit: f64 = row.get(3)?;
            let current_balance = f64_to_decimal(total_debit) - f64_to_decimal(total_credit);
            
            Ok(ClientBalance {
                client_id,
                client_name,
                total_debit: f64_to_decimal(total_debit),
                total_credit: f64_to_decimal(total_credit),
                current_balance,
            })
        })?;
        
        let mut result = Vec::new();
        for balance in balances {
            result.push(balance?);
        }
        
        Ok(result)
    }
    
    /// Get client name by ID
    fn get_client_name(&self, client_id: i32) -> AppResult<String> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM clients WHERE id = ?"
        )?;
        
        stmt.query_row([client_id], |row| row.get(0))
            .map_err(|e| AppError::Database(e))
    }
    
    /// Recalculate all ledger balances for a client and return the final balance
    /// Used after deleting an invoice to ensure consistency
    pub fn recalculate_client_balance(&self, client_id: i32) -> AppResult<Decimal> {
        let mut entries = self.get_by_client(client_id)?;
        
        let mut running_balance = Decimal::ZERO;
        for entry in &mut entries {
            running_balance = running_balance + entry.debit - entry.credit;
            entry.balance = running_balance;
            
            // Update the entry with new balance
            self.conn.execute(
                "UPDATE client_ledgers SET balance = ? WHERE id = ?",
                (decimal_to_f64(&entry.balance), entry.id.unwrap()),
            )?;
        }
        
        Ok(running_balance)
    }
    
    /// Delete ledger entries for a specific invoice
    pub fn delete_by_invoice(&self, invoice_id: i32) -> AppResult<usize> {
        let rows_affected = self.conn.execute(
            "DELETE FROM client_ledgers WHERE invoice_id = ?",
            [invoice_id],
        )?;
        
        Ok(rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use rust_decimal::Decimal;
    
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE clients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                contact_name TEXT,
                phone TEXT,
                email TEXT,
                address TEXT,
                gst_number TEXT,
                pan_number TEXT,
                opening_balance REAL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE client_ledgers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                client_id INTEGER NOT NULL,
                date TEXT NOT NULL,
                description TEXT NOT NULL,
                debit REAL NOT NULL DEFAULT 0,
                credit REAL NOT NULL DEFAULT 0,
                balance REAL NOT NULL DEFAULT 0,
                invoice_id INTEGER,
                FOREIGN KEY (client_id) REFERENCES clients(id)
            );"
        ).unwrap();
        
        // Insert test client
        conn.execute(
            "INSERT INTO clients (name) VALUES (?)",
            ["Test Client"],
        ).unwrap();
        
        conn
    }
    
    #[test]
    fn test_add_debit_entry() {
        let conn = setup_test_db();
        let repo = LedgerRepository::new(&conn);
        
        let entry_id = repo.add_debit(
            1,
            "2024-01-15".to_string(),
            "Invoice #1".to_string(),
            Decimal::new(100000, 2), // 1000.00
            Some(1),
        ).unwrap();
        
        assert!(entry_id > 0);
        
        let entry = repo.get_by_id(entry_id).unwrap().unwrap();
        assert_eq!(entry.debit, Decimal::new(100000, 2));
        assert_eq!(entry.credit, Decimal::ZERO);
        assert_eq!(entry.invoice_id, Some(1));
    }
    
    #[test]
    fn test_add_credit_entry() {
        let conn = setup_test_db();
        let repo = LedgerRepository::new(&conn);
        
        let entry_id = repo.add_credit(
            1,
            "2024-01-20".to_string(),
            "Payment received".to_string(),
            Decimal::new(50000, 2), // 500.00
            None,
        ).unwrap();
        
        assert!(entry_id > 0);
        
        let entry = repo.get_by_id(entry_id).unwrap().unwrap();
        assert_eq!(entry.credit, Decimal::new(50000, 2));
        assert_eq!(entry.debit, Decimal::ZERO);
    }
    
    #[test]
    fn test_get_client_balance() {
        let conn = setup_test_db();
        let repo = LedgerRepository::new(&conn);
        
        // Add debit
        repo.add_debit(
            1,
            "2024-01-15".to_string(),
            "Invoice #1".to_string(),
            Decimal::new(100000, 2),
            Some(1),
        ).unwrap();
        
        // Add credit
        repo.add_credit(
            1,
            "2024-01-20".to_string(),
            "Payment".to_string(),
            Decimal::new(40000, 2),
            None,
        ).unwrap();
        
        let balance = repo.get_client_balance(1).unwrap();
        assert_eq!(balance.current_balance, Decimal::new(60000, 2)); // 600.00
        assert_eq!(balance.total_debit, Decimal::new(100000, 2));
        assert_eq!(balance.total_credit, Decimal::new(40000, 2));
    }
    
    #[test]
    fn test_get_all_balances() {
        let conn = setup_test_db();
        let repo = LedgerRepository::new(&conn);
        
        // Add second client
        conn.execute(
            "INSERT INTO clients (name) VALUES (?)",
            ["Client 2"],
        ).unwrap();
        
        // Add transactions for client 1
        repo.add_debit(
            1,
            "2024-01-15".to_string(),
            "Invoice".to_string(),
            Decimal::new(100000, 2),
            Some(1),
        ).unwrap();
        
        let balances = repo.get_all_balances().unwrap();
        assert_eq!(balances.len(), 2);
        
        let client1_balance = balances.iter().find(|b| b.client_id == 1).unwrap();
        assert_eq!(client1_balance.current_balance, Decimal::new(100000, 2));
    }
    
    #[test]
    fn test_delete_by_invoice() {
        let conn = setup_test_db();
        let repo = LedgerRepository::new(&conn);
        
        // Add entries for invoice 1
        repo.add_debit(
            1,
            "2024-01-15".to_string(),
            "Invoice #1".to_string(),
            Decimal::new(100000, 2),
            Some(1),
        ).unwrap();
        
        // Add entries for invoice 2
        repo.add_debit(
            1,
            "2024-01-16".to_string(),
            "Invoice #2".to_string(),
            Decimal::new(50000, 2),
            Some(2),
        ).unwrap();
        
        let deleted = repo.delete_by_invoice(1).unwrap();
        assert_eq!(deleted, 1);
        
        let entries = repo.get_by_client(1).unwrap();
        assert_eq!(entries.len(), 1); // Only invoice 2 remains
        assert_eq!(entries[0].invoice_id, Some(2));
    }
}
