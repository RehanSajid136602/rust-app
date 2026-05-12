//! Database migrations module.
//! 
//! Manages schema versioning and migrations for the SQLite database.

use rusqlite::{Connection, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    
    #[error("Migration file not found: {0}")]
    FileNotFound(String),
    
    #[error("Migration failed: {0}")]
    MigrationFailed(String),
}

pub type MigrationResult<T> = Result<T, MigrationError>;

/// Represents a single migration
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: i64,
    pub description: &'static str,
    pub sql: &'static str,
}

/// Initial schema migration - copied from Python app's database.py
const MIGRATION_001_INITIAL: &str = r#"
-- Products table
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    price_per_unit REAL NOT NULL,
    unit TEXT DEFAULT 'pcs',
    hsn_code TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Clients table
CREATE TABLE IF NOT EXISTS clients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    address TEXT,
    phone TEXT,
    email TEXT,
    gstin TEXT,
    balance REAL DEFAULT 0
);

-- Client ledgers table
CREATE TABLE IF NOT EXISTS client_ledgers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    date TEXT NOT NULL,
    description TEXT,
    debit REAL DEFAULT 0,
    credit REAL DEFAULT 0,
    balance REAL DEFAULT 0,
    invoice_id INTEGER REFERENCES invoices(id) ON DELETE SET NULL
);

-- Invoices table
CREATE TABLE IF NOT EXISTS invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_number TEXT UNIQUE NOT NULL,
    ref_number TEXT,
    client_id INTEGER REFERENCES clients(id),
    client_name TEXT,
    client_address TEXT,
    invoice_date TEXT NOT NULL,
    due_date TEXT,
    subtotal REAL DEFAULT 0,
    tax_total REAL DEFAULT 0,
    discount_total REAL DEFAULT 0,
    grand_total REAL DEFAULT 0,
    amount_paid REAL DEFAULT 0,
    remaining_debt REAL DEFAULT 0,
    adjustment_label TEXT,
    adjustment_amount REAL DEFAULT 0,
    total REAL DEFAULT 0,
    notes TEXT,
    status TEXT DEFAULT 'draft',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Invoice items table
CREATE TABLE IF NOT EXISTS invoice_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_id INTEGER NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    sno INTEGER NOT NULL,
    item_name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    price_per_unit REAL NOT NULL DEFAULT 0,
    discount_amount REAL DEFAULT 0,
    tax_amount REAL DEFAULT 0,
    total_price REAL NOT NULL DEFAULT 0
);

-- Quotations table
CREATE TABLE IF NOT EXISTS quotations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    quotation_number TEXT UNIQUE NOT NULL,
    ref_number TEXT,
    client_id INTEGER REFERENCES clients(id),
    client_name TEXT,
    client_address TEXT,
    quotation_date TEXT NOT NULL,
    valid_until TEXT,
    subtotal REAL DEFAULT 0,
    tax_total REAL DEFAULT 0,
    discount_total REAL DEFAULT 0,
    grand_total REAL DEFAULT 0,
    adjustment_label TEXT,
    adjustment_amount REAL DEFAULT 0,
    total REAL DEFAULT 0,
    notes TEXT,
    status TEXT DEFAULT 'draft',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Quotation items table
CREATE TABLE IF NOT EXISTS quotation_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    quotation_id INTEGER NOT NULL REFERENCES quotations(id) ON DELETE CASCADE,
    sno INTEGER NOT NULL,
    item_name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    price_per_unit REAL NOT NULL DEFAULT 0,
    discount_amount REAL DEFAULT 0,
    tax_amount REAL DEFAULT 0,
    total_price REAL NOT NULL DEFAULT 0
);

-- Company settings table (singleton)
CREATE TABLE IF NOT EXISTS company_settings (
    id INTEGER PRIMARY KEY DEFAULT 1,
    company_name TEXT DEFAULT 'ZAHRA ENTERPRISES',
    tagline TEXT DEFAULT 'Deals in lab Consumables, Reagents & Medical Equipments.',
    ntn_number TEXT DEFAULT 'NTN NO. 2140708-8',
    office_address TEXT DEFAULT 'Office # 2-3, Basement Asif Plaza, Fazal-e-Haq Road, Blue Area, Islamabad',
    phone1 TEXT DEFAULT '0300-5259751',
    phone2 TEXT DEFAULT '0345-8510130',
    email TEXT DEFAULT 'zahraenterprises4@gmail.com',
    logo_path TEXT,
    invoice_prefix TEXT DEFAULT 'ZE #',
    next_invoice_number INTEGER DEFAULT 1,
    quotation_prefix TEXT DEFAULT 'QT #',
    next_quotation_number INTEGER DEFAULT 1,
    salutation TEXT DEFAULT 'Respected Sir,',
    body_text TEXT DEFAULT 'This is with reference to our quotation submitted; we are pleased to inform you that we have delivered following items.',
    banner_color TEXT DEFAULT '#1a2540',
    footer_color TEXT DEFAULT '#e05a2b'
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_invoices_client_id ON invoices(client_id);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
CREATE INDEX IF NOT EXISTS idx_invoices_created_at ON invoices(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_invoice_items_invoice_id ON invoice_items(invoice_id);
CREATE INDEX IF NOT EXISTS idx_quotations_client_id ON quotations(client_id);
CREATE INDEX IF NOT EXISTS idx_quotations_status ON quotations(status);
CREATE INDEX IF NOT EXISTS idx_quotations_created_at ON quotations(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_quotation_items_quotation_id ON quotation_items(quotation_id);
CREATE INDEX IF NOT EXISTS idx_ledgers_client_id ON client_ledgers(client_id);
CREATE INDEX IF NOT EXISTS idx_clients_name ON clients(name COLLATE NOCASE);
CREATE UNIQUE INDEX IF NOT EXISTS idx_invoices_number ON invoices(invoice_number);
CREATE UNIQUE INDEX IF NOT EXISTS idx_quotations_number ON quotations(quotation_number);
"#;

/// All migrations in order
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        description: "create_initial_tables",
        sql: MIGRATION_001_INITIAL,
    },
];

/// Create migrations table if it doesn't exist
fn ensure_migrations_table(conn: &Connection) -> MigrationResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

/// Get the current schema version
fn get_current_version(conn: &Connection) -> MigrationResult<i64> {
    conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
        [],
        |row| row.get(0),
    )
    .map_err(|e| e.into())
}

/// Check if a migration has been applied
fn migration_applied(conn: &Connection, version: i64) -> MigrationResult<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM schema_migrations WHERE version = ?",
        [version],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// Record a migration as applied
fn record_migration(conn: &Connection, version: i64) -> MigrationResult<()> {
    conn.execute(
        "INSERT INTO schema_migrations (version) VALUES (?)",
        [version],
    )?;
    Ok(())
}

/// Apply all pending migrations
pub fn apply_migrations(conn: &mut Connection) -> MigrationResult<()> {
    ensure_migrations_table(conn)?;
    
    let current_version = get_current_version(conn)?;
    
    for migration in MIGRATIONS {
        if migration.version <= current_version {
            continue;
        }
        
        if migration_applied(conn, migration.version)? {
            continue;
        }
        
        println!("Applying migration {}: {}", migration.version, migration.description);
        
        // Run migration in a transaction
        let tx = conn.transaction()?;
        tx.execute_batch(migration.sql)?;
        record_migration(&tx, migration.version)?;
        tx.commit()?;
        
        println!("Migration {} applied successfully", migration.version);
    }
    
    Ok(())
}

/// Initialize database with default settings if empty
pub fn seed_defaults(conn: &mut Connection) -> MigrationResult<()> {
    // Seed company settings if empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM company_settings", [], |row| row.get(0))?;
    if count == 0 {
        conn.execute("INSERT INTO company_settings (id) VALUES (1)", [])?;
    }
    
    // Seed products if empty (from original Python app)
    let product_count: i64 = conn.query_row("SELECT COUNT(*) FROM products", [], |row| row.get(0))?;
    if product_count == 0 {
        let seed_data = [
            ("Cp Vails", 11.0, "pcs"),
            ("Gel Vails", 16.0, "pcs"),
            ("CRP Fine Care", 360.0, "pcs"),
            ("Hba1C I coroma", 460.0, "pcs"),
            ("T3 Fine Care", 450.0, "pcs"),
            ("t4 Fine Care", 450.0, "pcs"),
            ("TSh Fine Care", 450.0, "pcs"),
            ("Blue Tips", 350.0, "pcs"),
            ("Yellow Tips", 400.0, "pcs"),
            ("Cover Slip", 250.0, "pcs"),
            ("Swabs Stick", 14.0, "pcs"),
            ("Trip i", 280.0, "pcs"),
            ("Blood Collection Set", 22.0, "pcs"),
            ("Dengue Ns1", 250.0, "pcs"),
            ("MP Ag", 85.0, "pcs"),
            ("Urine Container", 250.0, "pcs"),
            ("HbsAg+ HCV", 25.0, "pcs"),
        ];
        
        for (name, price, unit) in seed_data {
            conn.execute(
                "INSERT INTO products (name, price_per_unit, unit) VALUES (?, ?, ?)",
                (name, price, unit),
            )?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    #[test]
    fn test_apply_migrations() {
        let mut conn = Connection::open_in_memory().unwrap();
        let result = apply_migrations(&mut conn);
        assert!(result.is_ok());
        
        // Verify tables were created
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        
        assert!(tables.contains(&"products".to_string()));
        assert!(tables.contains(&"clients".to_string()));
        assert!(tables.contains(&"invoices".to_string()));
        assert!(tables.contains(&"quotations".to_string()));
    }
    
    #[test]
    fn test_seed_defaults() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_migrations(&mut conn).unwrap();
        seed_defaults(&mut conn).unwrap();
        
        // Verify settings were seeded
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM company_settings", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
        
        // Verify products were seeded
        let product_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM products", [], |row| row.get(0))
            .unwrap();
        assert_eq!(product_count, 17);
    }
}
