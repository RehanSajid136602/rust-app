//! Tauri command handlers for import operations.

use crate::AppState;
use crate::importers;

/// Import products from Excel catalog file
#[tauri::command]
pub fn import_products_excel(state: tauri::State<AppState>, path: String) -> Result<String, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    
    // Wrap in transaction - all or nothing
    conn.execute_batch("BEGIN TRANSACTION").map_err(|e| e.to_string())?;
    
    let mut repo = crate::repositories::ProductRepository::new(&mut *conn);
    let result = importers::import_from_excel(&mut repo, &path);
    
    match result {
        Ok((count, errors)) => {
            conn.execute_batch("COMMIT").map_err(|e| e.to_string())?;
            let msg = if errors.is_empty() {
                format!("Successfully imported {} products", count)
            } else if count == 0 {
                conn.execute_batch("ROLLBACK").ok();
                format!("No products found in file.\nParser errors:\n{}", errors.join("\n"))
            } else {
                format!("Imported {} products. {} skipped:\n{}", count, errors.len(), errors.iter().take(5).map(|e| e.as_str()).collect::<Vec<_>>().join("\n"))
            };
            Ok(msg)
        }
        Err(e) => {
            conn.execute_batch("ROLLBACK").ok();
            Err(format!("Import failed: {}", e))
        }
    }
}

/// Import products from PDF catalog file
#[tauri::command]
pub fn import_products_pdf(state: tauri::State<AppState>, path: String) -> Result<String, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    
    // Wrap in transaction - all or nothing
    conn.execute_batch("BEGIN TRANSACTION").map_err(|e| e.to_string())?;
    
    let mut repo = crate::repositories::ProductRepository::new(&mut *conn);
    let result = importers::import_from_pdf(&mut repo, &path);
    
    match result {
        Ok((count, _errors)) => {
            conn.execute_batch("COMMIT").map_err(|e| e.to_string())?;
            let msg = if count > 0 {
                format!("Imported {} products from PDF", count)
            } else {
                conn.execute_batch("ROLLBACK").ok();
                format!("No products detected in PDF. Make sure the PDF contains text (not scanned images) with product names and prices.")
            };
            Ok(msg)
        }
        Err(e) => {
            conn.execute_batch("ROLLBACK").ok();
            Err(format!("Import failed: {}", e))
        }
    }
}
