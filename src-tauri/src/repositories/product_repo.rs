//! Product repository for database operations.

use rusqlite::Connection;
use crate::models::{Product, CreateProductRequest};
use crate::errors::{AppResult, AppError};
use crate::db_types::{f64_to_decimal, decimal_to_f64};

/// Repository for Product data access
pub struct ProductRepository<'a> {
    conn: &'a mut Connection,
}

impl<'a> ProductRepository<'a> {
    /// Create a new ProductRepository
    pub fn new(conn: &'a mut Connection) -> Self {
        Self { conn }
    }
    
    /// Get all products
    pub fn get_all(&self) -> AppResult<Vec<Product>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, price_per_unit, unit, hsn_code, created_at 
             FROM products ORDER BY name"
        )?;
        
        let products = stmt.query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price_per_unit: f64_to_decimal(row.get(2)?),
                unit: row.get(3)?,
                hsn_code: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        
        let mut result = Vec::new();
        for product in products {
            result.push(product?);
        }
        
        Ok(result)
    }
    
    /// Get a product by ID
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<Product>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, price_per_unit, unit, hsn_code, created_at 
             FROM products WHERE id = ?"
        )?;
        
        let product = stmt.query_row([id], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price_per_unit: f64_to_decimal(row.get(2)?),
                unit: row.get(3)?,
                hsn_code: row.get(4)?,
                created_at: row.get(5)?,
            })
        });
        
        match product {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e)),
        }
    }
    
    /// Search products by name (fuzzy matching done in Rust, not SQL)
    pub fn search(&self, query: &str) -> AppResult<Vec<Product>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, price_per_unit, unit, hsn_code, created_at 
             FROM products WHERE name LIKE ? ORDER BY name LIMIT 50"
        )?;
        
        let search_pattern = format!("%{}%", query);
        let products = stmt.query_map([search_pattern], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price_per_unit: f64_to_decimal(row.get(2)?),
                unit: row.get(3)?,
                hsn_code: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        
        let mut result = Vec::new();
        for product in products {
            result.push(product?);
        }
        
        Ok(result)
    }
    
    /// Create a new product
    pub fn create(&self, req: &CreateProductRequest) -> AppResult<i32> {
        self.conn.execute(
            "INSERT INTO products (name, price_per_unit, unit, hsn_code) 
             VALUES (?, ?, ?, ?)",
            (&req.name, decimal_to_f64(&req.price_per_unit), &req.unit, &req.hsn_code),
        )?;
        
        Ok(self.conn.last_insert_rowid() as i32)
    }
    
    /// Update an existing product
    pub fn update(&self, id: i32, req: &CreateProductRequest) -> AppResult<()> {
        let rows_affected = self.conn.execute(
            "UPDATE products 
             SET name = ?, price_per_unit = ?, unit = ?, hsn_code = ? 
             WHERE id = ?",
            (&req.name, decimal_to_f64(&req.price_per_unit), &req.unit, &req.hsn_code, id),
        )?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Product with id {} not found", id)));
        }
        
        Ok(())
    }
    
    /// Delete a product
    pub fn delete(&self, id: i32) -> AppResult<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM products WHERE id = ?",
            [id],
        )?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Product with id {} not found", id)));
        }
        
        Ok(())
    }
    
    /// Upsert a product (insert or update by name)
    /// Used for catalog import
    pub fn upsert(&self, req: &CreateProductRequest) -> AppResult<i32> {
        self.conn.execute(
            "INSERT INTO products (name, price_per_unit, unit, hsn_code) 
             VALUES (?, ?, ?, ?)
             ON CONFLICT(name) DO UPDATE SET
                price_per_unit = excluded.price_per_unit,
                unit = excluded.unit,
                hsn_code = excluded.hsn_code",
            (&req.name, decimal_to_f64(&req.price_per_unit), &req.unit, &req.hsn_code),
        )?;
        
        // Get the ID of the upserted row
        let mut stmt = self.conn.prepare(
            "SELECT id FROM products WHERE name = ?"
        )?;
        
        let id: i32 = stmt.query_row([&req.name], |row| row.get(0))?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use rust_decimal::Decimal;
    
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                price_per_unit REAL NOT NULL,
                unit TEXT DEFAULT 'pcs',
                hsn_code TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        ).unwrap();
        conn
    }
    
    #[test]
    fn test_create_product() {
        let mut conn = setup_test_db();
        let repo = ProductRepository::new(&mut conn);
        
        let req = CreateProductRequest {
            name: "Test Product".to_string(),
            price_per_unit: Decimal::new(100, 2),
            unit: "pcs".to_string(),
            hsn_code: "1234".to_string(),
        };
        
        let id = repo.create(&req).unwrap();
        assert!(id > 0);
        
        let product = repo.get_by_id(id).unwrap().unwrap();
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price_per_unit, Decimal::new(100, 2));
    }
    
    #[test]
    fn test_search_products() {
        let mut conn = setup_test_db();
        let repo = ProductRepository::new(&mut conn);
        
        // Create test products
        for i in 1..=5 {
            let req = CreateProductRequest {
                name: format!("Product {}", i),
                price_per_unit: Decimal::new(100, 2),
                unit: "pcs".to_string(),
                hsn_code: "1234".to_string(),
            };
            repo.create(&req).unwrap();
        }
        
        let results = repo.search("Product 2").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Product 2");
    }
    
    #[test]
    fn test_upsert_product() {
        let mut conn = setup_test_db();
        let repo = ProductRepository::new(&mut conn);
        
        let req = CreateProductRequest {
            name: "Test Product".to_string(),
            price_per_unit: Decimal::new(100, 2),
            unit: "pcs".to_string(),
            hsn_code: "1234".to_string(),
        };
        
        // First insert
        let id1 = repo.upsert(&req).unwrap();
        
        // Update with same name
        let req2 = CreateProductRequest {
            name: "Test Product".to_string(),
            price_per_unit: Decimal::new(200, 2),
            unit: "pcs".to_string(),
            hsn_code: "5678".to_string(),
        };
        let id2 = repo.upsert(&req2).unwrap();
        
        assert_eq!(id1, id2); // Same ID
        
        let product = repo.get_by_id(id1).unwrap().unwrap();
        assert_eq!(product.price_per_unit, Decimal::new(200, 2)); // Updated price
    }
}
