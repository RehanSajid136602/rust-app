// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Module declarations
pub mod database;
pub mod models;
pub mod errors;
pub mod utils;
pub mod db_types;
pub mod repositories;
pub mod services;
pub mod commands;
pub mod exporters;
pub mod importers;
pub mod state;

use database::{init_database, run_migrations};
use std::sync::Arc;
use std::sync::Mutex;
use rusqlite::Connection;

/// Application state shared across all commands
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

/// Initialize and run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize database connection
    let db_conn = match init_database() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
    };
    
    // Run migrations
    if let Err(e) = run_migrations(&db_conn) {
        eprintln!("Failed to run migrations: {}", e);
        // Continue anyway - migrations might fail if DB already exists
    }
    
    // Create app state
    let app_state = AppState {
        db: db_conn,
    };
    
    // Build and run Tauri app
    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_all_products,
            commands::get_product_by_id,
            commands::search_products,
            commands::create_product,
            commands::update_product,
            commands::delete_product,
            commands::get_all_clients,
            commands::get_client_by_id,
            commands::create_client,
            commands::update_client,
            commands::delete_client,
            commands::get_client_balance,
            commands::get_all_client_balances,
            commands::get_all_invoices,
            commands::get_invoice_by_id,
            commands::get_invoices_by_client,
            commands::create_invoice,
            commands::update_invoice,
            commands::delete_invoice,
            commands::get_all_quotations,
            commands::get_quotation_by_id,
            commands::create_quotation,
            commands::update_quotation,
            commands::delete_quotation,
            commands::get_company_settings,
            commands::update_company_settings,
            commands::get_next_invoice_number,
            commands::get_next_quotation_number,
            commands::get_client_ledger,
            commands::get_ledger_entry_by_id,
            commands::get_client_balance_summary,
            commands::get_all_balances,
            commands::add_debit_entry,
            commands::add_credit_entry,
            commands::get_clients_with_balance,
            commands::import_products_excel,
            commands::import_products_pdf,
            commands::export_invoice_pdf,
            commands::export_quotation_pdf,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
