//! Tauri command handlers for Invoice operations.

use crate::AppState;
use crate::services::InvoiceService;
use crate::models::Invoice;

/// Get all invoices with pagination
#[tauri::command]
pub fn get_all_invoices(state: tauri::State<AppState>, limit: i32, offset: i32) -> Result<Vec<Invoice>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = InvoiceService::new(&mut *conn);
    service.get_all(limit, offset).map_err(|e| e.to_string())
}

/// Get an invoice by ID
#[tauri::command]
pub fn get_invoice_by_id(state: tauri::State<AppState>, id: i32) -> Result<Option<Invoice>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = InvoiceService::new(&mut *conn);
    service.get_by_id(id).map_err(|e| e.to_string())
}

/// Get invoices by client ID
#[tauri::command]
pub fn get_invoices_by_client(state: tauri::State<AppState>, client_id: i32) -> Result<Vec<Invoice>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = InvoiceService::new(&mut *conn);
    service.get_by_client(client_id).map_err(|e| e.to_string())
}

/// Create a new invoice
#[tauri::command]
pub fn create_invoice(state: tauri::State<AppState>, invoice: Invoice) -> Result<i32, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = InvoiceService::new(&mut *conn);
    service.create(&invoice).map_err(|e| e.to_string())
}

/// Update an existing invoice
#[tauri::command]
pub fn update_invoice(state: tauri::State<AppState>, invoice: Invoice) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = InvoiceService::new(&mut *conn);
    service.update(&invoice).map_err(|e| e.to_string())
}

/// Delete an invoice
#[tauri::command]
pub fn delete_invoice(state: tauri::State<AppState>, id: i32) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = InvoiceService::new(&mut *conn);
    service.delete(id).map_err(|e| e.to_string())
}
