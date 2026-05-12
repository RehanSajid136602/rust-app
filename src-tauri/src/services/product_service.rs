//! Product service for business logic.

use crate::repositories::ProductRepository;
use crate::models::{Product, CreateProductRequest};
use crate::errors::{AppResult, AppError};
use rust_decimal::Decimal;

/// Service for Product business logic
pub struct ProductService<'a> {
    repo: ProductRepository<'a>,
}

impl<'a> ProductService<'a> {
    /// Create a new ProductService
    pub fn new(conn: &'a mut rusqlite::Connection) -> Self {
        Self {
            repo: ProductRepository::new(conn),
        }
    }
    
    /// Get all products
    pub fn get_all(&self) -> AppResult<Vec<Product>> {
        self.repo.get_all()
    }
    
    /// Get a product by ID
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<Product>> {
        self.repo.get_by_id(id)
    }
    
    /// Search products by name
    pub fn search(&self, query: &str) -> AppResult<Vec<Product>> {
        if query.trim().is_empty() {
            return self.repo.get_all();
        }
        self.repo.search(query)
    }
    
    /// Create a new product with validation (upserts - updates if name exists)
    pub fn create(&mut self, req: &CreateProductRequest) -> AppResult<i32> {
        self.validate_product(req)?;
        self.repo.upsert(req)
    }
    
    /// Update an existing product with validation
    pub fn update(&mut self, id: i32, req: &CreateProductRequest) -> AppResult<()> {
        self.validate_product(req)?;
        self.repo.update(id, req)
    }
    
    /// Delete a product
    pub fn delete(&mut self, id: i32) -> AppResult<()> {
        self.repo.delete(id)
    }
    
    /// Upsert a product (for import operations)
    pub fn upsert(&mut self, req: &CreateProductRequest) -> AppResult<i32> {
        self.validate_product(req)?;
        self.repo.upsert(req)
    }
    
    /// Validate product data
    fn validate_product(&self, req: &CreateProductRequest) -> AppResult<()> {
        if req.name.trim().is_empty() {
            return Err(AppError::Validation("Product name is required".to_string()));
        }
        if req.name.len() < 2 {
            return Err(AppError::Validation(
                "Product name must be at least 2 characters".to_string()
            ));
        }
        if req.price_per_unit < Decimal::ZERO {
            return Err(AppError::Validation(
                "Price cannot be negative".to_string()
            ));
        }
        if req.unit.trim().is_empty() {
            return Err(AppError::Validation("Unit is required".to_string()));
        }
        Ok(())
    }
}
