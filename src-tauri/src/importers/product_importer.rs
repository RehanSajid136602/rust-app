//! Product importer from Excel and PDF file catalogues.

use crate::errors::{AppResult, AppError};
use crate::repositories::ProductRepository;
use crate::models::CreateProductRequest;
use rust_decimal::Decimal;

/// Import products from an Excel file
/// Expected columns: Name, Price, Unit (optional), HSN (optional)
/// First row is treated as a header and skipped
pub fn import_from_excel(
    repo: &mut ProductRepository,
    path: &str,
) -> AppResult<(usize, Vec<String>)> {
    use calamine::{open_workbook, Reader, Xlsx};
    
    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| AppError::Import(format!("Failed to open Excel file: {}", e)))?;
    
    let mut imported = 0usize;
    let mut errors = Vec::new();
    
    let sheet_names = workbook.sheet_names().to_vec();
    let sheet_name = sheet_names.first()
        .ok_or_else(|| AppError::Import("No sheets found in workbook".to_string()))?;
    
    if let Ok(range) = workbook.worksheet_range(sheet_name) {
        let rows_iter = range.rows().skip(1);
        
        for row in rows_iter {
            if row.is_empty() {
                continue;
            }
            
            let name = row.get(0)
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default();
            
            if name.is_empty() {
                continue;
            }
            
            let price = row.get(1)
                .and_then(|c| c.to_string().trim().parse::<f64>().ok())
                .unwrap_or(0.0);
            
            let unit = row.get(2)
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_else(|| "pcs".to_string());
            
            let hsn = row.get(3)
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default();
            
            let req = CreateProductRequest {
                name: name.clone(),
                price_per_unit: Decimal::try_from(price).unwrap_or(Decimal::ZERO),
                unit,
                hsn_code: hsn,
            };
            
            match repo.upsert(&req) {
                Ok(_) => imported += 1,
                Err(e) => errors.push(format!("{}: {}", name, e)),
            }
        }
    }
    
    Ok((imported, errors))
}

/// Import products from a PDF file
pub fn import_from_pdf(
    repo: &mut ProductRepository,
    path: &str,
) -> AppResult<(usize, Vec<String>)> {
    let bytes = std::fs::read(path)
        .map_err(|e| AppError::Import(format!("Failed to read PDF: {}", e)))?;
    
    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| AppError::Import(format!("Failed to extract PDF text: {}", e)))?;
    
    let mut imported = 0usize;
    let mut errors = Vec::new();
    
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if let Some((name, price)) = parse_price_line(line) {
            let req = CreateProductRequest {
                name,
                price_per_unit: price,
                unit: "pcs".to_string(),
                hsn_code: String::new(),
            };
            
            match repo.upsert(&req) {
                Ok(_) => imported += 1,
                Err(e) => errors.push(format!("Error on line '{}': {}", line, e)),
            }
        }
    }
    
    Ok((imported, errors))
}

/// Parse a line to extract product name and price
/// Handles formats:
///   "Widget A    100.00"
///   "Widget A    ₹100.00"
///   "1. Widget A    100.00"
///   "Widget A    100.00  pcs"
///   "100.00    Widget A"
fn parse_price_line(line: &str) -> Option<(String, Decimal)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    
    // Known unit strings to skip at the end
    let known_units = ["pcs", "pc", "kg", "g", "l", "ml", "m", "cm", "mm", "nos", "set", "box", "pack", "pair", "dozen", "doz"];
    
    // Try each word from right to left to find a price
    for end_offset in 0..3.min(parts.len()) {
        let idx = parts.len() - 1 - end_offset;
        let candidate = parts[idx];
        
        // Skip known units
        if known_units.contains(&candidate.to_lowercase().as_str()) {
            continue;
        }
        
        let price_str = candidate
            .trim_start_matches('₹')
            .trim_start_matches("Rs.")
            .trim_start_matches("Rs")
            .trim_end_matches('.');
        
        if let Ok(price_num) = price_str.replace(",", "").parse::<f64>() {
            if price_num > 0.0 && price_num < 100_000_000.0 {
                // Build name from parts before the price
                let mut name_parts: Vec<&str> = parts[..idx].to_vec();
                
                // Strip leading number/index
                if !name_parts.is_empty() {
                    let first = name_parts[0];
                    let first_stripped = first.trim_end_matches('.').trim_end_matches(')');
                    if first_stripped.chars().all(|c| c.is_ascii_digit()) && name_parts.len() > 1 {
                        name_parts.remove(0);
                    }
                }
                
                let name = name_parts.join(" ").trim().to_string();
                if name.len() >= 2 {
                    return Some((name, Decimal::try_from(price_num).unwrap_or(Decimal::ZERO)));
                }
            }
        }
    }
    
    None
}
