//! Client service for business logic.

use crate::repositories::ClientRepository;
use crate::models::{Client, CreateClientRequest};
use crate::errors::{AppResult, AppError};

/// Service for Client business logic
pub struct ClientService<'a> {
    repo: ClientRepository<'a>,
}

impl<'a> ClientService<'a> {
    /// Create a new ClientService
    pub fn new(conn: &'a mut rusqlite::Connection) -> Self {
        Self {
            repo: ClientRepository::new(conn),
        }
    }
    
    /// Get all clients
    pub fn get_all(&self, limit: i32, offset: i32) -> AppResult<Vec<Client>> {
        self.repo.get_all(limit, offset)
    }
    
    /// Get a client by ID
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<Client>> {
        self.repo.get_by_id(id)
    }
    
    /// Create a new client with validation
    pub fn create(&mut self, req: &CreateClientRequest) -> AppResult<i32> {
        self.validate_client(req)?;
        let client = Client {
            id: None,
            name: req.name.clone(),
            address: req.address.clone(),
            phone: req.phone.clone(),
            email: req.email.clone(),
            gstin: req.gstin.clone(),
            balance: rust_decimal::Decimal::ZERO,
        };
        self.repo.create(&client)
    }
    
    /// Update an existing client with validation
    pub fn update(&mut self, client: &Client) -> AppResult<()> {
        self.validate_client(&CreateClientRequest {
            name: client.name.clone(),
            address: client.address.clone(),
            phone: client.phone.clone(),
            email: client.email.clone(),
            gstin: client.gstin.clone(),
        })?;
        self.repo.update(client)
    }
    
    /// Delete a client
    pub fn delete(&mut self, id: i32) -> AppResult<()> {
        self.repo.delete(id)
    }
    
    /// Update client balance
    pub fn update_balance(&mut self, client_id: i32, balance: rust_decimal::Decimal) -> AppResult<()> {
        self.repo.update_balance(client_id, balance)
    }
    
    /// Validate client data
    fn validate_client(&self, req: &CreateClientRequest) -> AppResult<()> {
        if req.name.trim().is_empty() {
            return Err(AppError::Validation("Client name is required".to_string()));
        }
        if req.name.len() < 2 {
            return Err(AppError::Validation(
                "Client name must be at least 2 characters".to_string()
            ));
        }
        Ok(())
    }
}
