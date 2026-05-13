use crate::AppState;
use crate::services::InvoiceService;
use crate::exporters::excel_exporter;

#[tauri::command]
pub fn export_invoices_excel(
    state: tauri::State<AppState>,
    output_path: String,
) -> Result<String, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = InvoiceService::new(&mut *conn);
    let invoices = service.get_all(10000, 0).map_err(|e| e.to_string())?;

    excel_exporter::export_invoices_excel(&invoices, &output_path)?;

    Ok(output_path)
}
