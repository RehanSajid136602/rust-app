//! Tauri command handler for sequential reference number generation.
//! Pattern: ZE-{YEAR}-{3-digit-padded-sequence}

use tauri_plugin_store::StoreExt;
use chrono::Datelike;

/// Generate the next sequential reference number
#[tauri::command]
pub fn generate_ref_number(app: tauri::AppHandle) -> Result<String, String> {
    let store = app.store("app_data.json").map_err(|e| e.to_string())?;
    let current_year = chrono::Local::now().year();

    let last = store.get("lastRefSequence");

    let (year, mut seq) = match last {
        Some(val) => {
            let y = val.get("year").and_then(|v| v.as_i64()).unwrap_or(current_year as i64) as i32;
            let s = val.get("seq").and_then(|v| v.as_i64()).unwrap_or(0);
            if y != current_year {
                (current_year, 0)
            } else {
                (y, s)
            }
        }
        None => (current_year, 0),
    };

    seq += 1;

    store.set(
        "lastRefSequence".to_string(),
        serde_json::json!({ "year": year, "seq": seq }),
    );
    store.save().map_err(|e| e.to_string())?;

    Ok(format!("ZE-{}-{:03}", year, seq))
}
