//! Tauri command handlers.

pub mod product_cmds;
pub mod client_cmds;
pub mod invoice_cmds;
pub mod quotation_cmds;
pub mod settings_cmds;
pub mod ledger_cmds;
pub mod import_cmds;

pub use product_cmds::*;
pub use client_cmds::*;
pub use invoice_cmds::*;
pub use quotation_cmds::*;
pub use settings_cmds::*;
pub use ledger_cmds::*;
pub use import_cmds::*;

/// Placeholder greet command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
