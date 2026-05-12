//! Tauri command handlers for Quotation operations.

use crate::AppState;
use crate::services::QuotationService;
use crate::models::Quotation;

/// Get all quotations with pagination
#[tauri::command]
pub fn get_all_quotations(state: tauri::State<AppState>, limit: i32, offset: i32) -> Result<Vec<Quotation>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = QuotationService::new(&mut *conn);
    service.get_all(limit, offset).map_err(|e| e.to_string())
}

/// Get a quotation by ID
#[tauri::command]
pub fn get_quotation_by_id(state: tauri::State<AppState>, id: i32) -> Result<Option<Quotation>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = QuotationService::new(&mut *conn);
    service.get_by_id(id).map_err(|e| e.to_string())
}

/// Create a new quotation
#[tauri::command]
pub fn create_quotation(state: tauri::State<AppState>, quotation: Quotation) -> Result<i32, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = QuotationService::new(&mut *conn);
    service.create(&quotation).map_err(|e| e.to_string())
}

/// Update an existing quotation
#[tauri::command]
pub fn update_quotation(state: tauri::State<AppState>, quotation: Quotation) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = QuotationService::new(&mut *conn);
    service.update(&quotation).map_err(|e| e.to_string())
}

/// Delete a quotation
#[tauri::command]
pub fn delete_quotation(state: tauri::State<AppState>, id: i32) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = QuotationService::new(&mut *conn);
    service.delete(id).map_err(|e| e.to_string())
}
