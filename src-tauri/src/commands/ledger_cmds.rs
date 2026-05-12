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
