//! Tauri command handlers for Ledger operations.

use crate::AppState;
use crate::services::LedgerService;
use crate::models::{LedgerEntry, ClientBalance};
use rust_decimal::Decimal;

/// Get ledger entries for a client
#[tauri::command]
pub fn get_client_ledger(state: tauri::State<AppState>, client_id: i32) -> Result<Vec<LedgerEntry>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = LedgerService::new(&mut *conn);
    service.get_client_ledger(client_id).map_err(|e| e.to_string())
}

/// Get a single ledger entry by ID
#[tauri::command]
pub fn get_ledger_entry_by_id(state: tauri::State<AppState>, id: i32) -> Result<Option<LedgerEntry>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = LedgerService::new(&mut *conn);
    service.get_entry_by_id(id).map_err(|e| e.to_string())
}

/// Get client balance summary
#[tauri::command]
pub fn get_client_balance_summary(state: tauri::State<AppState>, client_id: i32) -> Result<ClientBalance, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = LedgerService::new(&mut *conn);
    service.get_client_balance(client_id).map_err(|e| e.to_string())
}

/// Get all client balances
#[tauri::command]
pub fn get_all_balances(state: tauri::State<AppState>) -> Result<Vec<ClientBalance>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = LedgerService::new(&mut *conn);
    service.get_all_balances().map_err(|e| e.to_string())
}

/// Add a debit entry
#[tauri::command]
pub fn add_debit_entry(state: tauri::State<AppState>, client_id: i32, amount: Decimal, description: String, date: String) -> Result<i32, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = LedgerService::new(&mut *conn);
    service.add_debit(client_id, amount, &description, &date).map_err(|e| e.to_string())
}

/// Add a credit entry
#[tauri::command]
pub fn add_credit_entry(state: tauri::State<AppState>, client_id: i32, amount: Decimal, description: String, date: String) -> Result<i32, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = LedgerService::new(&mut *conn);
    service.add_credit(client_id, amount, &description, &date).map_err(|e| e.to_string())
}

/// Get clients with outstanding balance
#[tauri::command]
pub fn get_clients_with_balance(state: tauri::State<AppState>) -> Result<Vec<ClientBalance>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = LedgerService::new(&mut *conn);
    service.get_clients_with_balance().map_err(|e| e.to_string())
}

/// Rebuild all ledger entries from existing invoices
/// Creates missing debit entries for invoices and credit entries for payments
#[tauri::command]
pub fn rebuild_ledger(state: tauri::State<AppState>) -> Result<String, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    
    // Rebuild debit entries from invoices
    let invoices: Vec<(i32, i32, String, f64, String, f64)> = {
        let mut stmt = tx.prepare(
            "SELECT i.id, i.client_id, i.invoice_number, i.total, i.invoice_date, i.amount_paid
             FROM invoices i
             LEFT JOIN client_ledgers cl ON cl.invoice_id = i.id AND cl.debit > 0
             WHERE i.client_id IS NOT NULL AND cl.id IS NULL"
        ).map_err(|e| e.to_string())?;
        
        stmt.query_map([], |row| {
            Ok((
                row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?,
            ))
        }).map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect()
    };
    
    let mut debit_count = 0;
    for (inv_id, client_id, inv_num, total, date, amount_paid) in &invoices {
        tx.execute(
            "INSERT INTO client_ledgers (client_id, date, description, debit, credit, balance, invoice_id)
             VALUES (?1, ?2, ?3, ?4, 0, ?4, ?5)",
            rusqlite::params![client_id, date, format!("Invoice {}", inv_num), total, inv_id],
        ).map_err(|e| e.to_string())?;
        debit_count += 1;
        
        // If invoice has payments, create credit entry
        if *amount_paid > 0.0 {
            tx.execute(
                "INSERT INTO client_ledgers (client_id, date, description, debit, credit, balance, invoice_id)
                 VALUES (?1, ?2, ?3, 0, ?4, ?4, ?5)",
                rusqlite::params![client_id, date, format!("Payment for {}", inv_num), amount_paid, inv_id],
            ).map_err(|e| e.to_string())?;
            debit_count += 1;
        }
    }
    
    tx.commit().map_err(|e| e.to_string())?;
    
    Ok(format!("Rebuilt {} ledger entries from {} invoices", debit_count, invoices.len()))
}
