//! Application error types.

use thiserror::Error;
use crate::database::DatabaseError;

/// Main application error type
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Database error: {0}")]
    DatabaseInit(#[from] DatabaseError),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Duplicate entry: {0}")]
    Duplicate(String),
    
    #[error("Export error: {0}")]
    Export(String),
    
    #[error("Import error: {0}")]
    Import(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Application result type alias
pub type AppResult<T> = Result<T, AppError>;
