//! Quotation repository for database operations.

use rusqlite::{Connection, params_from_iter};
use chrono::Utc;
use crate::models::{Quotation, QuotationItem};
use crate::errors::{AppResult, AppError};
use crate::db_types::{f64_to_decimal, decimal_to_f64};

/// Repository for Quotation data access
pub struct QuotationRepository<'a> {
    conn: &'a mut Connection,
}

impl<'a> QuotationRepository<'a> {
    /// Create a new QuotationRepository
    pub fn new(conn: &'a mut Connection) -> Self {
        Self { conn }
    }
    
    /// Get all quotations with pagination
    pub fn get_all(&self, limit: i32, offset: i32) -> AppResult<Vec<Quotation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, quotation_number, ref_number, client_id, client_name, client_address,
                    quotation_date, valid_until, subtotal, tax_total, discount_total, grand_total,
                    adjustment_label, adjustment_amount, total, notes, status, created_at
             FROM quotations ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )?;
        
        let quotations = stmt.query_map([limit, offset], |row| {
            Ok(Quotation {
                id: row.get(0)?,
                quotation_number: row.get(1)?,
                ref_number: row.get(2)?,
                client_id: row.get(3)?,
                client_name: row.get(4)?,
                client_address: row.get(5)?,
                quotation_date: row.get(6)?,
                valid_until: row.get(7)?,
                subtotal: f64_to_decimal(row.get(8)?),
                tax_total: f64_to_decimal(row.get(9)?),
                discount_total: f64_to_decimal(row.get(10)?),
                grand_total: f64_to_decimal(row.get(11)?),
                adjustment_label: row.get(12)?,
                adjustment_amount: f64_to_decimal(row.get(13)?),
                total: f64_to_decimal(row.get(14)?),
                notes: row.get(15)?,
                status: row.get(16)?,
                created_at: row.get(17)?,
                items: Vec::new(),
            })
        })?;
        
        let mut result = Vec::new();
        for quotation in quotations {
            let mut qt = quotation?;
            qt.items = self.get_items(qt.id.unwrap())?;
            result.push(qt);
        }
        
        Ok(result)
    }
    
    /// Get a quotation by ID with all items
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<Quotation>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, quotation_number, ref_number, client_id, client_name, client_address,
                    quotation_date, valid_until, subtotal, tax_total, discount_total, grand_total,
                    adjustment_label, adjustment_amount, total, notes, status, created_at
             FROM quotations WHERE id = ?"
        )?;
        
        let quotation = stmt.query_row([id], |row| {
            Ok(Quotation {
                id: row.get(0)?,
                quotation_number: row.get(1)?,
                ref_number: row.get(2)?,
                client_id: row.get(3)?,
                client_name: row.get(4)?,
                client_address: row.get(5)?,
                quotation_date: row.get(6)?,
                valid_until: row.get(7)?,
                subtotal: f64_to_decimal(row.get(8)?),
                tax_total: f64_to_decimal(row.get(9)?),
                discount_total: f64_to_decimal(row.get(10)?),
                grand_total: f64_to_decimal(row.get(11)?),
                adjustment_label: row.get(12)?,
                adjustment_amount: f64_to_decimal(row.get(13)?),
                total: f64_to_decimal(row.get(14)?),
                notes: row.get(15)?,
                status: row.get(16)?,
                created_at: row.get(17)?,
                items: Vec::new(),
            })
        });
        
        match quotation {
            Ok(mut qt) => {
                qt.items = self.get_items(id)?;
                Ok(Some(qt))
            },
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e)),
        }
    }
    
    /// Get quotation items for a specific quotation
    fn get_items(&self, quotation_id: i32) -> AppResult<Vec<QuotationItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, quotation_id, sno, item_name, quantity, price_per_unit, 
                    discount_amount, tax_amount, total_price
             FROM quotation_items WHERE quotation_id = ? ORDER BY sno"
        )?;
        
        let items = stmt.query_map([quotation_id], |row| {
            Ok(QuotationItem {
                id: row.get(0)?,
                quotation_id: row.get(1)?,
                sno: row.get(2)?,
                item_name: row.get(3)?,
                quantity: f64_to_decimal(row.get(4)?),
                price_per_unit: f64_to_decimal(row.get(5)?),
                discount_amount: f64_to_decimal(row.get(6)?),
                tax_amount: f64_to_decimal(row.get(7)?),
                total_price: f64_to_decimal(row.get(8)?),
            })
        })?;
        
        let mut result = Vec::new();
        for item in items {
            result.push(item?);
        }
        
        Ok(result)
    }
    
    /// Create a new quotation with items
    pub fn create(&mut self, quotation: &Quotation) -> AppResult<i32> {
        let tx = self.conn.transaction()?;

        // Auto-generate quotation number if empty
        let quotation_number = if quotation.quotation_number.trim().is_empty() {
            let prefix: String = tx.query_row(
                "SELECT quotation_prefix FROM company_settings WHERE id = 1",
                [],
                |row| row.get(0),
            )?;
            let year = Utc::now().format("%Y").to_string().parse::<u32>().unwrap_or(2026);
            let pattern = format!("{}-{}-%", prefix, year);
            let max_num: Option<i32> = tx.query_row(
                "SELECT COALESCE(MAX(CAST(SUBSTR(quotation_number, -4) AS INTEGER)), 0) + 1
                 FROM quotations WHERE quotation_number LIKE ?",
                rusqlite::params![pattern],
                |row| row.get(0),
            ).ok();
            format!("{}-{}-{:04}", prefix, year, max_num.unwrap_or(1))
        } else {
            quotation.quotation_number.clone()
        };

        let subtotal = decimal_to_f64(&quotation.subtotal);
        let tax_total = decimal_to_f64(&quotation.tax_total);
        let discount_total = decimal_to_f64(&quotation.discount_total);
        let grand_total = decimal_to_f64(&quotation.grand_total);
        let adjustment_amount = decimal_to_f64(&quotation.adjustment_amount);
        let total = decimal_to_f64(&quotation.total);
        
        let params: Vec<&dyn rusqlite::ToSql> = vec![
            &quotation_number, &quotation.ref_number, &quotation.client_id,
            &quotation.client_name, &quotation.client_address, &quotation.quotation_date,
            &quotation.valid_until, &subtotal, &tax_total,
            &discount_total, &grand_total, &quotation.adjustment_label,
            &adjustment_amount, &total, &quotation.notes, &quotation.status,
        ];
        
        tx.execute(
            "INSERT INTO quotations (
                quotation_number, ref_number, client_id, client_name, client_address,
                quotation_date, valid_until, subtotal, tax_total, discount_total,
                grand_total, adjustment_label, adjustment_amount, total, notes, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params_from_iter(params),
        )?;
        
        let quotation_id = tx.last_insert_rowid() as i32;
        
        for item in &quotation.items {
            let qty = decimal_to_f64(&item.quantity);
            let price = decimal_to_f64(&item.price_per_unit);
            let discount = decimal_to_f64(&item.discount_amount);
            let tax = decimal_to_f64(&item.tax_amount);
            let item_total = decimal_to_f64(&item.total_price);
            
            tx.execute(
                "INSERT INTO quotation_items (
                    quotation_id, sno, item_name, quantity, price_per_unit,
                    discount_amount, tax_amount, total_price
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                (quotation_id, item.sno, &item.item_name, qty, price, discount, tax, item_total),
            )?;
        }
        
        // Increment next quotation number
        tx.execute(
            "UPDATE company_settings SET next_quotation_number = next_quotation_number + 1",
            [],
        )?;
        
        tx.commit()?;
        
        Ok(quotation_id)
    }
    
    /// Update an existing quotation with items
    pub fn update(&mut self, quotation: &Quotation) -> AppResult<()> {
        let id = quotation.id.ok_or_else(|| AppError::Validation("Quotation ID is required for update".to_string()))?;
        
        let tx = self.conn.transaction()?;
        
        let subtotal = decimal_to_f64(&quotation.subtotal);
        let tax_total = decimal_to_f64(&quotation.tax_total);
        let discount_total = decimal_to_f64(&quotation.discount_total);
        let grand_total = decimal_to_f64(&quotation.grand_total);
        let adjustment_amount = decimal_to_f64(&quotation.adjustment_amount);
        let total = decimal_to_f64(&quotation.total);
        
        let params: Vec<&dyn rusqlite::ToSql> = vec![
            &quotation.quotation_number, &quotation.ref_number, &quotation.client_id,
            &quotation.client_name, &quotation.client_address, &quotation.quotation_date,
            &quotation.valid_until, &subtotal, &tax_total,
            &discount_total, &grand_total, &quotation.adjustment_label,
            &adjustment_amount, &total, &quotation.notes, &quotation.status, &id,
        ];
        
        tx.execute(
            "UPDATE quotations SET
                quotation_number = ?, ref_number = ?, client_id = ?, client_name = ?,
                client_address = ?, quotation_date = ?, valid_until = ?, subtotal = ?,
                tax_total = ?, discount_total = ?, grand_total = ?, adjustment_label = ?,
                adjustment_amount = ?, total = ?, notes = ?, status = ?
            WHERE id = ?",
            params_from_iter(params),
        )?;
        
        tx.execute("DELETE FROM quotation_items WHERE quotation_id = ?", [id])?;
        
        for item in &quotation.items {
            let qty = decimal_to_f64(&item.quantity);
            let price = decimal_to_f64(&item.price_per_unit);
            let discount = decimal_to_f64(&item.discount_amount);
            let tax = decimal_to_f64(&item.tax_amount);
            let item_total = decimal_to_f64(&item.total_price);
            
            tx.execute(
                "INSERT INTO quotation_items (
                    quotation_id, sno, item_name, quantity, price_per_unit,
                    discount_amount, tax_amount, total_price
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                (id, item.sno, &item.item_name, qty, price, discount, tax, item_total),
            )?;
        }
        
        tx.commit()?;
        
        Ok(())
    }
    
    /// Delete a quotation
    pub fn delete(&mut self, id: i32) -> AppResult<()> {
        let tx = self.conn.transaction()?;
        
        // Delete quotation items (cascade should handle this, but be explicit)
        tx.execute("DELETE FROM quotation_items WHERE quotation_id = ?", [id])?;
        
        // Delete quotation
        let rows_affected = tx.execute("DELETE FROM quotations WHERE id = ?", [id])?;
        
        tx.commit()?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Quotation with id {} not found", id)));
        }
        
        Ok(())
    }
}
