//! Tauri command handlers for Product operations.

use crate::AppState;
use crate::services::ProductService;
use crate::models::{Product, CreateProductRequest};

/// Get all products - direct DB query for reliability
#[tauri::command]
pub fn get_all_products(state: tauri::State<AppState>) -> Result<Vec<Product>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare(
        "SELECT id, name, price_per_unit, unit, hsn_code, created_at 
         FROM products ORDER BY name"
    ).map_err(|e| e.to_string())?;
    
    let products = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            price_per_unit: crate::db_types::f64_to_decimal(row.get(2)?),
            unit: row.get(3)?,
            hsn_code: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for product in products {
        result.push(product.map_err(|e| e.to_string())?);
    }
    
    Ok(result)
}

/// Get a product by ID
#[tauri::command]
pub fn get_product_by_id(state: tauri::State<AppState>, id: i32) -> Result<Option<Product>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = ProductService::new(&mut *conn);
    service.get_by_id(id).map_err(|e| e.to_string())
}

/// Search products by name - direct DB query
#[tauri::command]
pub fn search_products(state: tauri::State<AppState>, query: String) -> Result<Vec<Product>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    let search_pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT id, name, price_per_unit, unit, hsn_code, created_at 
         FROM products WHERE name LIKE ? ORDER BY name LIMIT 50"
    ).map_err(|e| e.to_string())?;
    
    let products = stmt.query_map([search_pattern], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            price_per_unit: crate::db_types::f64_to_decimal(row.get(2)?),
            unit: row.get(3)?,
            hsn_code: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for product in products {
        result.push(product.map_err(|e| e.to_string())?);
    }
    
    Ok(result)
}

/// Create a new product - direct DB for reliability
#[tauri::command]
pub fn create_product(state: tauri::State<AppState>, req: CreateProductRequest) -> Result<i32, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    // Validate
    if req.name.trim().is_empty() {
        return Err("Product name is required".to_string());
    }
    
    let price = crate::db_types::decimal_to_f64(&req.price_per_unit);
    
    conn.execute(
        "INSERT INTO products (name, price_per_unit, unit, hsn_code) 
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(name) DO UPDATE SET
            price_per_unit = excluded.price_per_unit,
            unit = excluded.unit,
            hsn_code = excluded.hsn_code",
        rusqlite::params![&req.name, price, &req.unit, &req.hsn_code],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid() as i32)
}

/// Update an existing product
#[tauri::command]
pub fn update_product(state: tauri::State<AppState>, id: i32, req: CreateProductRequest) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = ProductService::new(&mut *conn);
    service.update(id, &req).map_err(|e| e.to_string())
}

/// Delete a product
#[tauri::command]
pub fn delete_product(state: tauri::State<AppState>, id: i32) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = ProductService::new(&mut *conn);
    service.delete(id).map_err(|e| e.to_string())
}
