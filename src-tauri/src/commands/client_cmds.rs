//! Tauri command handlers for Client operations.

use crate::AppState;
use crate::services::{ClientService, LedgerService};
use crate::models::{Client, CreateClientRequest, ClientBalance};

/// Get all clients with pagination
#[tauri::command]
pub fn get_all_clients(state: tauri::State<AppState>, limit: i32, offset: i32) -> Result<Vec<Client>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = ClientService::new(&mut *conn);
    service.get_all(limit, offset).map_err(|e| e.to_string())
}

/// Get a client by ID
#[tauri::command]
pub fn get_client_by_id(state: tauri::State<AppState>, id: i32) -> Result<Option<Client>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = ClientService::new(&mut *conn);
    service.get_by_id(id).map_err(|e| e.to_string())
}

/// Create a new client
#[tauri::command]
pub fn create_client(state: tauri::State<AppState>, req: CreateClientRequest) -> Result<i32, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = ClientService::new(&mut *conn);
    service.create(&req).map_err(|e| e.to_string())
}

/// Update an existing client
#[tauri::command]
pub fn update_client(state: tauri::State<AppState>, client: Client) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = ClientService::new(&mut *conn);
    service.update(&client).map_err(|e| e.to_string())
}

/// Delete a client
#[tauri::command]
pub fn delete_client(state: tauri::State<AppState>, id: i32) -> Result<(), String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let mut service = ClientService::new(&mut *conn);
    service.delete(id).map_err(|e| e.to_string())
}

/// Get client balance
#[tauri::command]
pub fn get_client_balance(state: tauri::State<AppState>, client_id: i32) -> Result<ClientBalance, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = LedgerService::new(&mut *conn);
    service.get_client_balance(client_id).map_err(|e| e.to_string())
}

/// Get all client balances
#[tauri::command]
pub fn get_all_client_balances(state: tauri::State<AppState>) -> Result<Vec<ClientBalance>, String> {
    let mut conn = state.db.lock().map_err(|e| e.to_string())?;
    let service = LedgerService::new(&mut *conn);
    service.get_all_balances().map_err(|e| e.to_string())
}
