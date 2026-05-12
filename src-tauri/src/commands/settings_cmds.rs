//! Tauri command handlers for Settings operations.

use crate::AppState;
use crate::services::SettingsService;
use crate::models::CompanySettings;

/// Get company settings
#[tauri::command]
pub fn get_company_settings(state: tauri::State<AppState>) -> Result<CompanySettings, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = SettingsService::new(&mut *conn);
    service.get().map_err(|e| e.to_string())
}

/// Update company settings
#[tauri::command]
pub fn update_company_settings(state: tauri::State<AppState>, settings: CompanySettings) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = SettingsService::new(&mut *conn);
    service.update(&settings).map_err(|e| e.to_string())
}

/// Get next invoice number
#[tauri::command]
pub fn get_next_invoice_number(state: tauri::State<AppState>, year: u32) -> Result<String, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = SettingsService::new(&mut *conn);
    service.get_next_invoice_number(year).map_err(|e| e.to_string())
}

/// Get next quotation number
#[tauri::command]
pub fn get_next_quotation_number(state: tauri::State<AppState>, year: u32) -> Result<String, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = SettingsService::new(&mut *conn);
    service.get_next_quotation_number(year).map_err(|e| e.to_string())
}
