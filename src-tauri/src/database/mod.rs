//! Database connection and management module.
//! 
//! This module handles SQLite database connections with:
//! - WAL mode for better concurrency
//! - Foreign key enforcement
//! - Connection pooling via Arc<Mutex>
//! - Busy timeout for concurrent access

mod migrations;

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::errors::AppResult;

/// Database error types
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    
    #[error("Migration error: {0}")]
    Migration(#[from] migrations::MigrationError),
    
    #[error("Database initialization failed: {0}")]
    Initialization(String),
}

/// Get the database path
fn get_database_path() -> PathBuf {
    // Always use a path relative to the src-tauri directory
    // During `tauri dev`, CWD is src-tauri/, so this resolves correctly
    let db_path = PathBuf::from("invoice_app.db");
    let absolute = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(&db_path);
    absolute
}

/// Create a new database connection
pub fn get_connection() -> AppResult<Connection> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path)?;
    
    // Configure SQLite for optimal performance
    conn.execute_batch(
        "
        PRAGMA journal_mode=WAL;
        PRAGMA synchronous=NORMAL;
        PRAGMA foreign_keys=ON;
        PRAGMA busy_timeout=5000;
        PRAGMA cache_size=-64000;
        PRAGMA temp_store=MEMORY;
        "
    )?;
    
    Ok(conn)
}

/// Initialize the database connection pool
pub fn init_database() -> AppResult<Arc<Mutex<Connection>>> {
    let conn = get_connection()?;
    Ok(Arc::new(Mutex::new(conn)))
}

/// Run database migrations
pub fn run_migrations(db: &Arc<Mutex<Connection>>) -> AppResult<()> {
    let mut conn_locked = db.lock().unwrap();
    migrations::apply_migrations(&mut conn_locked)
        .map_err(DatabaseError::Migration)?;
    Ok(())
}

/// Seed database with default data
#[allow(dead_code)]
pub fn seed_defaults(db: &Arc<Mutex<Connection>>) -> AppResult<()> {
    let mut conn_locked = db.lock().unwrap();
    migrations::seed_defaults(&mut conn_locked)
        .map_err(DatabaseError::Migration)?;
    Ok(())
}

/// Rebuild missing ledger entries from existing invoices on startup.
/// Creates debit entries for invoices and credit entries for payments.
pub fn rebuild_ledger_if_needed(db: &Arc<Mutex<Connection>>) -> AppResult<()> {
    let conn = db.lock().unwrap();
    
    // Find invoices missing debit ledger entries
    let rows: Vec<(i32, i32, String, f64, String, f64)> = {
        let mut stmt = conn.prepare(
            "SELECT i.id, i.client_id, i.invoice_number, i.total, i.invoice_date, i.amount_paid
             FROM invoices i
             LEFT JOIN client_ledgers cl ON cl.invoice_id = i.id AND cl.debit > 0
             WHERE i.client_id IS NOT NULL AND cl.id IS NULL"
        )?;
        stmt.query_map([], |row| Ok((
            row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?,
        )))?
        .filter_map(|r| r.ok())
        .collect()
    };
    
    for (inv_id, client_id, inv_num, total, date, amount_paid) in &rows {
        conn.execute(
            "INSERT INTO client_ledgers (client_id, date, description, debit, credit, balance, invoice_id)
             VALUES (?1, ?2, ?3, ?4, 0, ?4, ?5)",
            rusqlite::params![client_id, date, format!("Invoice {}", inv_num), total, inv_id],
        )?;
        
        // If invoice has payments, also create credit entry
        if *amount_paid > 0.0 {
            conn.execute(
                "INSERT INTO client_ledgers (client_id, date, description, debit, credit, balance, invoice_id)
                 VALUES (?1, ?2, ?3, 0, ?4, ?4, ?5)",
                rusqlite::params![client_id, date, format!("Payment for {}", inv_num), amount_paid, inv_id],
            )?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_connection() {
        let conn = get_connection();
        assert!(conn.is_ok());
    }
    
    #[test]
    fn test_wal_mode() {
        let conn = get_connection().unwrap();
        let result: String = conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .unwrap();
        assert_eq!(result, "wal");
    }
    
    #[test]
    fn test_foreign_keys() {
        let conn = get_connection().unwrap();
        let result: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(result, 1);
    }
}
