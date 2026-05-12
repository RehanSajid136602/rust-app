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

/// Import products from a PDF file.
///
/// Specifically designed to handle invoice-style PDFs with table-formatted items:
///   S.No  Items Details  Quantity  Price/Unit  Total Price
///   1     Cp Vails       400       11          4,400
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

        // Skip summary/footer/header lines
        if is_skip_line(line) {
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
                Err(e) => errors.push(format!("{}: {}", line, e)),
            }
        }
    }

    Ok((imported, errors))
}

/// Check if a line should be skipped (header/footer/summary text, not a product)
fn is_skip_line(line: &str) -> bool {
    let lower = line.to_lowercase();
    let skip_starts = [
        "dated:", "ref #", "invoice", "respected", "this is",
        "inform", "s. no", "s.no", "total amount", "adjusted",
        "subtotal", "grand total", "net amount", "thank you",
        "and hope", "office #", "office no", "zahra", "www.",
        "ntn", "gst", "phone", "mobile", "fax", "web:",
    ];
    for prefix in &skip_starts {
        if lower.starts_with(prefix) {
            return true;
        }
    }

    // Skip lines that are just "Rs. X" (net amount row)
    if lower.starts_with("rs.") || lower.starts_with("rs ") {
        return true;
    }

    // Skip email lines
    if line.contains('@') {
        return true;
    }

    // Skip phone number lines (starts with 0, mostly digits/spaces)
    let digits_only: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits_only.len() >= 8 && line.chars().filter(|c| c.is_ascii_digit()).count() as f64 / line.len().max(1) as f64 > 0.6 {
        return true;
    }

    false
}

/// Parse a line from a table-formatted invoice PDF to extract product name and unit price.
///
/// Handles lines like:
///   "1 Cp Vails 400 11 4,400"       → name="Cp Vails", price=11
///   "Blood Collection Set 100 22 2,200" → name="Blood Collection Set", price=22
///   "Widget A 500/-"                → name="Widget A", price=500
fn parse_price_line(line: &str) -> Option<(String, Decimal)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Split into tokens (whitespace-separated)
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 2 {
        return None;
    }

    // Collect numeric tokens (last N tokens that are valid numbers)
    let mut numeric_indices: Vec<usize> = Vec::new();
    for (i, tok) in tokens.iter().enumerate() {
        let cleaned = tok
            .trim_start_matches('₹')
            .trim_start_matches("Rs.")
            .trim_start_matches("Rs")
            .trim_end_matches("/-")
            .trim_end_matches("/=")
            .trim_end_matches("/")
            .trim_end_matches("-")
            .trim_end_matches('.')
            .replace(",", "");

        if cleaned.parse::<f64>().is_ok() {
            // Exclude single-digit numbers if they're the first token (S.No)
            if i == 0 {
                let n: f64 = cleaned.parse().unwrap_or(0.0);
                if n >= 0.0 && n <= 999.0 {
                    // Could be S.No, include it but we'll strip from name later
                    numeric_indices.push(i);
                    continue;
                }
            }
            numeric_indices.push(i);
        }
    }

    if numeric_indices.is_empty() {
        return None;
    }

    // Find the last consecutive run of numeric tokens at the end
    let mut num_end_run: Vec<usize> = Vec::new();
    let mut expected = numeric_indices.last().copied()?;
    for &idx in numeric_indices.iter().rev() {
        if idx == expected {
            num_end_run.push(idx);
            if expected == 0 { break; }
            expected = expected.wrapping_sub(1);
        } else {
            break;
        }
    }
    num_end_run.reverse(); // Restore original order

    if num_end_run.is_empty() {
        num_end_run.push(*numeric_indices.last()?);
    }

    let num_count = num_end_run.len();

    // Determine price_per_unit based on how many numeric tokens at end
    let price_per_unit_idx = if num_count >= 3 {
        // Table format: qty, price_per_unit, total → pick 2nd from last
        num_end_run[num_end_run.len() - 2]
    } else if num_count == 2 {
        // Could be qty+total or price+total → pick first of the pair
        num_end_run[num_end_run.len() - 2]
    } else {
        // Single number: that's the price
        num_end_run[0]
    };

    // Get the price value
    let price_token = tokens[price_per_unit_idx];
    let price_str = price_token
        .trim_start_matches('₹')
        .trim_start_matches("Rs.")
        .trim_start_matches("Rs")
        .trim_end_matches("/-")
        .trim_end_matches("/=")
        .trim_end_matches("/")
        .trim_end_matches("-")
        .trim_end_matches('.')
        .replace(",", "");

    let price_num = price_str.parse::<f64>().ok()?;
    if price_num <= 0.0 || price_num >= 100_000_000.0 {
        return None;
    }

    // Build product name: tokens before the numeric run
    // Skip leading currency tokens like "Rs.", "₹", "Rs"
    let mut name_start: usize = 0;
    for (i, t) in tokens.iter().enumerate() {
        let lower = t.to_lowercase();
        if lower == "rs" || lower == "rs." || lower == "₹" {
            name_start = i + 1;
        } else {
            break;
        }
    }

    // Strip S.No (first token if it's purely numeric and <= 999)
    if name_start < tokens.len() {
        let first = tokens[name_start];
        let cleaned_first = first.trim_end_matches('.').trim_end_matches(')').trim_start_matches('(');
        if cleaned_first.chars().all(|c| c.is_ascii_digit()) {
            let n: f64 = cleaned_first.parse().unwrap_or(0.0);
            if n <= 999.0 && tokens.len() > name_start + 1 {
                name_start += 1;
            }
        }
    }

    // Build product name from tokens that are not numeric and not currency/unit
    let currency_tokens = ["rs", "rs.", "₹"];
    let name_tokens: Vec<&str> = tokens[name_start..tokens.len()]
        .iter()
        .enumerate()
        .filter(|(i, t)| {
            let idx = name_start + i;
            // Skip currency tokens
            if currency_tokens.contains(&t.to_lowercase().as_str()) {
                return false;
            }
            // Skip tokens that are part of the numeric run
            if num_end_run.contains(&idx) {
                return false;
            }
            true
        })
        .map(|(_, t)| *t)
        .collect();

    let name = name_tokens.join(" ").trim().to_string();
    if name.len() < 2 {
        return None;
    }

    let price = Decimal::try_from(price_num).unwrap_or(Decimal::ZERO);
    Some((name, price))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_invoice_lines() {
        let cases = vec![
            ("1 Cp Vails 400 11 4,400", "Cp Vails", 11.0),
            ("2 Gel Vails 200 16 3,200", "Gel Vails", 16.0),
            ("3 CRP Fine Care 125 360 45,000", "CRP Fine Care", 360.0),
            ("4 Hba1C I coroma 50 460 23,000", "Hba1C I coroma", 460.0),
            ("5 T3 Fine Care 50 450 22,500", "T3 Fine Care", 450.0),
            ("6 t4 Fine Care 50 450 22,500", "t4 Fine Care", 450.0),
            ("7 TSh Fine Care 50 450 22,500", "TSh Fine Care", 450.0),
            ("8 Blue Tips 1 350 350", "Blue Tips", 350.0),
            ("9 Yellow Tips 2 400 800", "Yellow Tips", 400.0),
            ("10 Cover Slip 2 250 500", "Cover Slip", 250.0),
            ("11 Swabs Stick 100 14 1,400", "Swabs Stick", 14.0),
            ("12 Trip i 25 280 7,000", "Trip i", 280.0),
            ("13 Blood Collection Set 100 22 2,200", "Blood Collection Set", 22.0),
            ("14 Dengue Ns1 50 250 12,500", "Dengue Ns1", 250.0),
            ("15 MP Ag 50 85 4,250", "MP Ag", 85.0),
            ("16 Urine Container 10 250 2,500", "Urine Container", 250.0),
            ("17 HbsAg+ HCV 75 25 1,875", "HbsAg+ HCV", 25.0),
            // Standalone price line
            ("Widget A 500/-", "Widget A", 500.0),
            ("Rs. 1,500/-  Item B", "Item B", 1500.0),
        ];

        for (line, expected_name, expected_price) in &cases {
            let result = parse_price_line(line);
            assert!(result.is_some(), "Failed to parse: '{}'", line);
            let (name, price) = result.unwrap();
            let price_f64: f64 = price.try_into().unwrap_or(0.0);
            assert_eq!(name, *expected_name, "Line: '{}' - expected name '{}', got '{}'", line, expected_name, name);
            assert!((price_f64 - expected_price).abs() < 0.01, "Line: '{}' - expected price {}, got {}", line, expected_price, price_f64);
        }
    }

    #[test]
    fn test_skip_lines() {
        let skip_lines = vec![
            "Dated: 28- 04 -2026",
            "Ref # ZE # GMC",
            "Invoice",
            "Respected Sir,",
            "This is with reference to our quotation submitted",
            "S. No Items Details Quantity Price/Unit Total Price",
            "Total Amount Rs. 176,475/-",
            "Adjusted Amount ( Used Micro Scope) 18000/-",
            "Rs. 158,574/-",
            "Thank you for considering Zahra Enterprises",
            "Office # 2-3, Basement Asif Plaza",
            "zahraenterprises4@gmail.com",
            "0300-5259751 0345-8510130",
        ];

        for line in &skip_lines {
            assert!(is_skip_line(line), "Should skip: '{}'", line);
        }
    }
}
