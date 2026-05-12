//! Tauri command handlers for PDF export operations.

use crate::AppState;
use crate::models::{Invoice, Quotation};
use crate::services::SettingsService;
use crate::exporters::pdf_exporter;

/// Export an invoice to PDF at the given file path
#[tauri::command]
pub fn export_invoice_pdf(
    state: tauri::State<AppState>,
    invoice: Invoice,
    output_path: String,
) -> Result<String, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = SettingsService::new(&mut *conn);
    let settings = service.get().map_err(|e| e.to_string())?;

    pdf_exporter::export_invoice_pdf(&invoice, &settings, &output_path)?;

    Ok(output_path)
}

/// Export a quotation to PDF at the given file path
#[tauri::command]
pub fn export_quotation_pdf(
    state: tauri::State<AppState>,
    quotation: Quotation,
    output_path: String,
) -> Result<String, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = SettingsService::new(&mut *conn);
    let settings = service.get().map_err(|e| e.to_string())?;

    pdf_exporter::export_quotation_pdf(&quotation, &settings, &output_path)?;

    Ok(output_path)
}
