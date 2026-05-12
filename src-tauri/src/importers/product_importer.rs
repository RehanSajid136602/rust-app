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

/// Parse a line to extract product name and price.
fn parse_price_line(line: &str) -> Option<(String, Decimal)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let known_units = [
        "pcs", "pc", "kg", "g", "l", "ml", "m", "cm", "mm",
        "nos", "set", "box", "pack", "pair", "dozen", "doz",
        "vial", "vials", "bottle", "btl", "kit", "unit", "units",
        "strip", "tablet", "tab", "capsule", "cap", "amp", "ampoule",
        "roll", "sheet", "can", "drum", "barrel", "carton", "ctn",
    ];

    // Find price candidates by scanning for number patterns
    #[derive(Debug)]
    struct PriceCandidate {
        start: usize,
        end: usize,
        value: f64,
    }

    let mut candidates: Vec<PriceCandidate> = Vec::new();
    let chars: Vec<char> = line.chars().collect();

    // Scan through the line for price-like patterns
    let mut i = 0;
    while i < chars.len() {
        // Skip non-price-start characters
        if !chars[i].is_ascii_digit() && chars[i] != '₹' && chars[i] != 'R' {
            i += 1;
            continue;
        }

        let scan_start = i;

        // Skip currency prefix
        if chars[i] == '₹' {
            i += 1;
        } else if chars.get(i..i + 3).map_or(false, |s| s == ['R', 's', '.']) {
            i += 3;
        } else if chars.get(i..i + 2).map_or(false, |s| s == ['R', 's']) {
            i += 2;
        }

        // Skip whitespace after currency prefix
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }

        // Read the numeric price: digits, commas, decimal points
        let num_start = i;
        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == ',' || chars[i] == '.') {
            i += 1;
        }
        let num_end = i;

        if num_end <= num_start {
            // No digits found, continue
            i = scan_start + 1;
            continue;
        }

        let num_str: String = chars[num_start..num_end].iter().collect();
        let cleaned = num_str.replace(",", "");

        if let Ok(n) = cleaned.parse::<f64>() {
            if n > 0.0 && n < 100_000_000.0 {
                // Check for trailing /- or /= or / or -
                let mut price_end = num_end;
                while price_end < chars.len() && (chars[price_end] == '/' || chars[price_end] == '-' || chars[price_end] == '=') {
                    price_end += 1;
                }

                candidates.push(PriceCandidate {
                    start: scan_start,
                    end: price_end,
                    value: n,
                });
            }
        }

        i = num_end;
    }

    if candidates.is_empty() {
        // Fallback: word-by-word from right
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        for end_offset in 0..4.min(parts.len()) {
            let idx = parts.len() - 1 - end_offset;
            let candidate = parts[idx];

            if known_units.contains(&candidate.to_lowercase().as_str()) {
                continue;
            }

            let price_str = candidate
                .trim_start_matches('₹')
                .trim_start_matches("Rs.")
                .trim_start_matches("Rs")
                .trim_end_matches("/-")
                .trim_end_matches("/=")
                .trim_end_matches("/")
                .trim_end_matches("-")
                .trim_end_matches('.');

            if let Ok(price_num) = price_str.replace(",", "").parse::<f64>() {
                if price_num > 0.0 && price_num < 100_000_000.0 {
                    let byte_pos = line.find(candidate).unwrap_or(0);
                    candidates.push(PriceCandidate {
                        start: byte_pos,
                        end: byte_pos + candidate.len(),
                        value: price_num,
                    });
                    break;
                }
            }
        }
    }

    // Pick the rightmost price as the item price (closest to end is usually the price)
    let best = candidates.last()?;
    let price_start = best.start;
    let price_end = best.end;
    let price_num = best.value;

    // Build product name: everything before the price + after the price
    let mut name = String::new();
    if price_start > 0 {
        name.push_str(line[..price_start].trim());
    }
    if price_end < line.len() {
        let after = line[price_end..].trim();
        if !name.is_empty() && !after.is_empty() {
            name.push(' ');
        }
        name.push_str(after);
    }
    name = name.trim().to_string();

    // Strip leading number/index like "1.", "1)", "(1)"
    let first_word: &str = name.split_whitespace().next().unwrap_or("");
    let stripped_first = first_word
        .trim_end_matches('.')
        .trim_end_matches(')')
        .trim_start_matches('(');
    if !stripped_first.is_empty()
        && stripped_first.chars().all(|c| c.is_ascii_digit())
        && name.split_whitespace().count() > 1
    {
        name = name[first_word.len()..].trim().to_string();
    }

    // Remove trailing units from name
    let lower = name.to_lowercase();
    for unit in &known_units {
        if lower.ends_with(&format!(" {}", unit)) {
            name = name[..name.len() - unit.len() - 1].trim().to_string();
            break;
        }
        if lower == *unit {
            name.clear();
            break;
        }
    }

    // Strip trailing punctuation
    name = name.trim_end_matches(|c: char| c == ',' || c == ';' || c == '/' || c == '-' || c == '.' || c == '(' || c == ')').to_string();

    if name.len() >= 2 {
        let price = Decimal::try_from(price_num).unwrap_or(Decimal::ZERO);
        Some((name, price))
    } else {
        None
    }
}
