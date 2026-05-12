//! Client repository for database operations.

use rusqlite::Connection;
use crate::models::Client;
use crate::errors::{AppResult, AppError};
use crate::db_types::{f64_to_decimal, decimal_to_f64};
use rust_decimal::Decimal;

/// Repository for Client data access
pub struct ClientRepository<'a> {
    conn: &'a mut Connection,
}

impl<'a> ClientRepository<'a> {
    /// Create a new ClientRepository
    pub fn new(conn: &'a mut Connection) -> Self {
        Self { conn }
    }
    
    /// Get all clients with pagination
    pub fn get_all(&self, limit: i32, offset: i32) -> AppResult<Vec<Client>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, address, phone, email, gstin, balance 
             FROM clients ORDER BY name LIMIT ? OFFSET ?"
        )?;
        
        let clients = stmt.query_map([limit, offset], |row| {
            Ok(Client {
                id: row.get(0)?,
                name: row.get(1)?,
                address: row.get(2)?,
                phone: row.get(3)?,
                email: row.get(4)?,
                gstin: row.get(5)?,
                balance: f64_to_decimal(row.get(6)?),
            })
        })?;
        
        let mut result = Vec::new();
        for client in clients {
            result.push(client?);
        }
        
        Ok(result)
    }
    
    /// Get a client by ID
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<Client>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, address, phone, email, gstin, balance 
             FROM clients WHERE id = ?"
        )?;
        
        let client = stmt.query_row([id], |row| {
            Ok(Client {
                id: row.get(0)?,
                name: row.get(1)?,
                address: row.get(2)?,
                phone: row.get(3)?,
                email: row.get(4)?,
                gstin: row.get(5)?,
                balance: f64_to_decimal(row.get(6)?),
            })
        });
        
        match client {
            Ok(c) => Ok(Some(c)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e)),
        }
    }
    
    /// Create a new client
    pub fn create(&self, client: &Client) -> AppResult<i32> {
        self.conn.execute(
            "INSERT INTO clients (name, address, phone, email, gstin, balance) 
             VALUES (?, ?, ?, ?, ?, ?)",
            (&client.name, &client.address, &client.phone, &client.email, &client.gstin, decimal_to_f64(&client.balance)),
        )?;
        
        Ok(self.conn.last_insert_rowid() as i32)
    }
    
    /// Update an existing client
    pub fn update(&self, client: &Client) -> AppResult<()> {
        let id = client.id.ok_or_else(|| AppError::Validation("Client ID is required for update".to_string()))?;
        
        let rows_affected = self.conn.execute(
            "UPDATE clients 
             SET name = ?, address = ?, phone = ?, email = ?, gstin = ?, balance = ? 
             WHERE id = ?",
            (&client.name, &client.address, &client.phone, &client.email, &client.gstin, decimal_to_f64(&client.balance), id),
        )?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Client with id {} not found", id)));
        }
        
        Ok(())
    }
    
    /// Delete a client and all related ledger entries
    /// Invoices are kept but their client_id is set to NULL to preserve history
    pub fn delete(&mut self, id: i32) -> AppResult<()> {
        let tx = self.conn.transaction()?;
        
        // First delete all ledger entries for this client
        tx.execute("DELETE FROM client_ledgers WHERE client_id = ?", [id])?;
        
        // Set client_id to NULL for all invoices of this client (preserve invoice history)
        tx.execute("UPDATE invoices SET client_id = NULL WHERE client_id = ?", [id])?;
        
        // Now delete the client
        let rows_affected = tx.execute("DELETE FROM clients WHERE id = ?", [id])?;
        
        tx.commit()?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Client with id {} not found", id)));
        }
        
        Ok(())
    }
    
    /// Update client balance by adding an amount (positive for debit, negative for credit)
    pub fn update_balance(&self, id: i32, amount: Decimal) -> AppResult<()> {
        self.conn.execute(
            "UPDATE clients SET balance = balance + ? WHERE id = ?",
            (decimal_to_f64(&amount), id),
        )?;
        
        Ok(())
    }
    
    /// Get client balance
    pub fn get_balance(&self, id: i32) -> AppResult<Decimal> {
        let balance: f64 = self.conn.query_row(
            "SELECT balance FROM clients WHERE id = ?",
            [id],
            |row| row.get(0),
        )?;
        
        Ok(f64_to_decimal(balance))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE clients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                address TEXT,
                phone TEXT,
                email TEXT,
                gstin TEXT,
                balance REAL DEFAULT 0
            )"
        ).unwrap();
        conn
    }
    
    #[test]
    fn test_create_client() {
        let mut conn = setup_test_db();
        let repo = ClientRepository::new(&mut conn);
        
        let client = Client {
            id: None,
            name: "Test Client".to_string(),
            address: "Test Address".to_string(),
            phone: "0300-1234567".to_string(),
            email: "test@example.com".to_string(),
            gstin: "123456".to_string(),
            balance: Decimal::ZERO,
        };
        
        let id = repo.create(&client).unwrap();
        assert!(id > 0);
        
        let retrieved = repo.get_by_id(id).unwrap().unwrap();
        assert_eq!(retrieved.name, "Test Client");
    }
    
    #[test]
    fn test_update_balance() {
        let mut conn = setup_test_db();
        let repo = ClientRepository::new(&mut conn);
        
        let client = Client::default();
        let id = repo.create(&client).unwrap();
        
        repo.update_balance(id, Decimal::new(10000, 2)).unwrap(); // Add 100.00
        
        let balance = repo.get_balance(id).unwrap();
        assert_eq!(balance, Decimal::new(10000, 2));
        
        repo.update_balance(id, Decimal::new(-3000, 2)).unwrap(); // Subtract 30.00
        
        let balance = repo.get_balance(id).unwrap();
        assert_eq!(balance, Decimal::new(7000, 2));
    }
}
