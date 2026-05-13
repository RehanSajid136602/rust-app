//! Invoice repository for database operations.

use rusqlite::{Connection, params_from_iter};
use chrono::Utc;
use crate::models::{Invoice, InvoiceItem, PaymentStatus};
use crate::errors::{AppResult, AppError};
use crate::db_types::{f64_to_decimal, decimal_to_f64};

/// Repository for Invoice data access
pub struct InvoiceRepository<'a> {
    conn: &'a mut Connection,
}

impl<'a> InvoiceRepository<'a> {
    /// Create a new InvoiceRepository
    pub fn new(conn: &'a mut Connection) -> Self {
        Self { conn }
    }
    
    /// Get all invoices with pagination
    pub fn get_all(&self, limit: i32, offset: i32) -> AppResult<Vec<Invoice>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, invoice_number, ref_number, client_id, client_name, client_address,
                    invoice_date, due_date, subtotal, tax_total, discount_total, grand_total,
                    amount_paid, remaining_debt, adjustment_label, adjustment_amount, total,
                    notes, status, created_at
             FROM invoices ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )?;
        
        let invoices = stmt.query_map([limit, offset], |row| {
            Ok(Invoice {
                id: row.get(0)?,
                invoice_number: row.get(1)?,
                ref_number: row.get(2)?,
                client_id: row.get(3)?,
                client_name: row.get(4)?,
                client_address: row.get(5)?,
                invoice_date: row.get(6)?,
                due_date: row.get(7)?,
                subtotal: f64_to_decimal(row.get(8)?),
                tax_total: f64_to_decimal(row.get(9)?),
                discount_total: f64_to_decimal(row.get(10)?),
                grand_total: f64_to_decimal(row.get(11)?),
                amount_paid: f64_to_decimal(row.get(12)?),
                remaining_debt: f64_to_decimal(row.get(13)?),
                adjustment_label: row.get(14)?,
                adjustment_amount: f64_to_decimal(row.get(15)?),
                total: f64_to_decimal(row.get(16)?),
                notes: row.get(17)?,
                status: {
                    let status_str: String = row.get(18)?;
                    match status_str.as_str() {
                        "paid" => PaymentStatus::Paid,
                        "partial" => PaymentStatus::Partial,
                        _ => PaymentStatus::Unpaid,
                    }
                },
                created_at: row.get(19)?,
                items: Vec::new(),
            })
        })?;
        
        let mut result = Vec::new();
        for invoice in invoices {
            let mut inv = invoice?;
            inv.items = self.get_items(inv.id.unwrap())?;
            result.push(inv);
        }
        
        Ok(result)
    }
    
    /// Get an invoice by ID with all items
    pub fn get_by_id(&self, id: i32) -> AppResult<Option<Invoice>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, invoice_number, ref_number, client_id, client_name, client_address,
                    invoice_date, due_date, subtotal, tax_total, discount_total, grand_total,
                    amount_paid, remaining_debt, adjustment_label, adjustment_amount, total,
                    notes, status, created_at
             FROM invoices WHERE id = ?"
        )?;
        
        let invoice = stmt.query_row([id], |row| {
            Ok(Invoice {
                id: row.get(0)?,
                invoice_number: row.get(1)?,
                ref_number: row.get(2)?,
                client_id: row.get(3)?,
                client_name: row.get(4)?,
                client_address: row.get(5)?,
                invoice_date: row.get(6)?,
                due_date: row.get(7)?,
                subtotal: f64_to_decimal(row.get(8)?),
                tax_total: f64_to_decimal(row.get(9)?),
                discount_total: f64_to_decimal(row.get(10)?),
                grand_total: f64_to_decimal(row.get(11)?),
                amount_paid: f64_to_decimal(row.get(12)?),
                remaining_debt: f64_to_decimal(row.get(13)?),
                adjustment_label: row.get(14)?,
                adjustment_amount: f64_to_decimal(row.get(15)?),
                total: f64_to_decimal(row.get(16)?),
                notes: row.get(17)?,
                status: {
                    let status_str: String = row.get(18)?;
                    match status_str.as_str() {
                        "paid" => PaymentStatus::Paid,
                        "partial" => PaymentStatus::Partial,
                        _ => PaymentStatus::Unpaid,
                    }
                },
                created_at: row.get(19)?,
                items: Vec::new(),
            })
        });
        
        match invoice {
            Ok(mut inv) => {
                inv.items = self.get_items(id)?;
                Ok(Some(inv))
            },
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e)),
        }
    }
    
    /// Get invoice items for a specific invoice
    fn get_items(&self, invoice_id: i32) -> AppResult<Vec<InvoiceItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, invoice_id, sno, item_name, quantity, price_per_unit, 
                    discount_amount, tax_amount, total_price
             FROM invoice_items WHERE invoice_id = ? ORDER BY sno"
        )?;
        
        let items = stmt.query_map([invoice_id], |row| {
            Ok(InvoiceItem {
                id: row.get(0)?,
                invoice_id: row.get(1)?,
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
    
    /// Create a new invoice with items
    pub fn create(&mut self, invoice: &Invoice) -> AppResult<i32> {
        let tx = self.conn.transaction()?;

        // Auto-generate invoice number if empty
        let invoice_number = if invoice.invoice_number.trim().is_empty() {
            // Query settings prefix
            let prefix: String = tx.query_row(
                "SELECT invoice_prefix FROM company_settings WHERE id = 1",
                [],
                |row| row.get(0),
            ).unwrap_or_else(|_| "ZE #".to_string());
            let year = Utc::now().format("%Y").to_string().parse::<u32>().unwrap_or(2026);
            let pattern = format!("{}-{}-%", prefix, year);
            let max_num: Option<i32> = tx.query_row(
                "SELECT COALESCE(MAX(CAST(SUBSTR(invoice_number, -4) AS INTEGER)), 0) + 1
                 FROM invoices WHERE invoice_number LIKE ?",
                rusqlite::params![pattern],
                |row| row.get(0),
            ).ok();
            format!("{}-{}-{:04}", prefix, year, max_num.unwrap_or(1))
        } else {
            invoice.invoice_number.clone()
        };

        let subtotal = decimal_to_f64(&invoice.subtotal);
        let tax_total = decimal_to_f64(&invoice.tax_total);
        let discount_total = decimal_to_f64(&invoice.discount_total);
        let grand_total = decimal_to_f64(&invoice.grand_total);
        let amount_paid = decimal_to_f64(&invoice.amount_paid);
        let remaining_debt = decimal_to_f64(&invoice.remaining_debt);
        let adjustment_amount = decimal_to_f64(&invoice.adjustment_amount);
        let total = decimal_to_f64(&invoice.total);
        let status_str = invoice.status.as_str();
        
        let params: Vec<&dyn rusqlite::ToSql> = vec![
            &invoice_number, &invoice.ref_number, &invoice.client_id,
            &invoice.client_name, &invoice.client_address, &invoice.invoice_date,
            &invoice.due_date, &subtotal, &tax_total,
            &discount_total, &grand_total, &amount_paid,
            &remaining_debt, &invoice.adjustment_label, &adjustment_amount,
            &total, &invoice.notes, &status_str,
        ];
        
        tx.execute(
            "INSERT INTO invoices (
                invoice_number, ref_number, client_id, client_name, client_address,
                invoice_date, due_date, subtotal, tax_total, discount_total,
                grand_total, amount_paid, remaining_debt, adjustment_label,
                adjustment_amount, total, notes, status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params_from_iter(params),
        )?;
        
        let invoice_id = tx.last_insert_rowid() as i32;
        
        for item in &invoice.items {
            let qty = decimal_to_f64(&item.quantity);
            let price = decimal_to_f64(&item.price_per_unit);
            let discount = decimal_to_f64(&item.discount_amount);
            let tax = decimal_to_f64(&item.tax_amount);
            let item_total = decimal_to_f64(&item.total_price);
            
            tx.execute(
                "INSERT INTO invoice_items (
                    invoice_id, sno, item_name, quantity, price_per_unit,
                    discount_amount, tax_amount, total_price
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                (invoice_id, item.sno, &item.item_name, qty, price, discount, tax, item_total),
            )?;
        }
        
        // Increment next invoice number
        tx.execute(
            "UPDATE company_settings SET next_invoice_number = next_invoice_number + 1",
            [],
        )?;

        // Create ledger debit entry for this invoice
        if let Some(cid) = invoice.client_id {
            let debit_desc = format!("Invoice {}", invoice_number);
            let debit_amount = decimal_to_f64(&invoice.total);
            let today = Utc::now().format("%Y-%m-%d").to_string();
            tx.execute(
                "INSERT INTO client_ledgers (client_id, date, description, debit, credit, balance, invoice_id)
                 VALUES (?, ?, ?, ?, 0, ?, ?)",
                rusqlite::params![cid, today, debit_desc, debit_amount, debit_amount, invoice_id],
            )?;
        }

        tx.commit()?;
        
        Ok(invoice_id)
    }
    
    /// Update an existing invoice with items
    pub fn update(&mut self, invoice: &Invoice) -> AppResult<()> {
        let id = invoice.id.ok_or_else(|| AppError::Validation("Invoice ID is required for update".to_string()))?;
        
        let tx = self.conn.transaction()?;
        
        let subtotal = decimal_to_f64(&invoice.subtotal);
        let tax_total = decimal_to_f64(&invoice.tax_total);
        let discount_total = decimal_to_f64(&invoice.discount_total);
        let grand_total = decimal_to_f64(&invoice.grand_total);
        let amount_paid = decimal_to_f64(&invoice.amount_paid);
        let remaining_debt = decimal_to_f64(&invoice.remaining_debt);
        let adjustment_amount = decimal_to_f64(&invoice.adjustment_amount);
        let total = decimal_to_f64(&invoice.total);
        let status_str = invoice.status.as_str();
        
        let params: Vec<&dyn rusqlite::ToSql> = vec![
            &invoice.invoice_number, &invoice.ref_number, &invoice.client_id,
            &invoice.client_name, &invoice.client_address, &invoice.invoice_date,
            &invoice.due_date, &subtotal, &tax_total,
            &discount_total, &grand_total, &amount_paid,
            &remaining_debt, &invoice.adjustment_label, &adjustment_amount,
            &total, &invoice.notes, &status_str, &id,
        ];
        
        tx.execute(
            "UPDATE invoices SET
                invoice_number = ?, ref_number = ?, client_id = ?, client_name = ?,
                client_address = ?, invoice_date = ?, due_date = ?, subtotal = ?,
                tax_total = ?, discount_total = ?, grand_total = ?, amount_paid = ?,
                remaining_debt = ?, adjustment_label = ?, adjustment_amount = ?,
                total = ?, notes = ?, status = ?
            WHERE id = ?",
            params_from_iter(params),
        )?;
        
        tx.execute("DELETE FROM invoice_items WHERE invoice_id = ?", [id])?;
        
        for item in &invoice.items {
            let qty = decimal_to_f64(&item.quantity);
            let price = decimal_to_f64(&item.price_per_unit);
            let discount = decimal_to_f64(&item.discount_amount);
            let tax = decimal_to_f64(&item.tax_amount);
            let item_total = decimal_to_f64(&item.total_price);
            
            tx.execute(
                "INSERT INTO invoice_items (
                    invoice_id, sno, item_name, quantity, price_per_unit,
                    discount_amount, tax_amount, total_price
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                (id, item.sno, &item.item_name, qty, price, discount, tax, item_total),
            )?;
        }
        
        tx.commit()?;
        
        Ok(())
    }
    
    /// Delete an invoice and related ledger entries
    pub fn delete(&mut self, id: i32) -> AppResult<()> {
        let tx = self.conn.transaction()?;
        
        let invoice_info: Option<(i32, f64, f64, String)> = tx.query_row(
            "SELECT client_id, grand_total, amount_paid, invoice_number FROM invoices WHERE id = ?",
            [id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        ).ok();
        
        if let Some((client_id, _grand_total, _amount_paid, invoice_number)) = invoice_info {
            tx.execute("DELETE FROM client_ledgers WHERE invoice_id = ?", [id])?;
            
            tx.execute(
                "DELETE FROM client_ledgers WHERE client_id = ? AND description LIKE ?",
                (client_id, format!("%{}%", invoice_number)),
            )?;
            
            let new_balance: f64 = tx.query_row(
                "SELECT COALESCE(SUM(debit), 0) - COALESCE(SUM(credit), 0) FROM client_ledgers WHERE client_id = ?",
                [client_id],
                |row| row.get(0),
            ).unwrap_or(0.0);
            
            tx.execute("UPDATE clients SET balance = ? WHERE id = ?", (new_balance, client_id))?;
        }
        
        // Delete invoice items (cascade should handle this, but be explicit)
        tx.execute("DELETE FROM invoice_items WHERE invoice_id = ?", [id])?;
        
        // Delete invoice
        let rows_affected = tx.execute("DELETE FROM invoices WHERE id = ?", [id])?;
        
        tx.commit()?;
        
        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Invoice with id {} not found", id)));
        }
        
        Ok(())
    }
    
    /// Get invoices by client ID
    pub fn get_by_client(&self, client_id: i32) -> AppResult<Vec<Invoice>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, invoice_number, ref_number, client_id, client_name, client_address,
                    invoice_date, due_date, subtotal, tax_total, discount_total, grand_total,
                    amount_paid, remaining_debt, adjustment_label, adjustment_amount, total,
                    notes, status, created_at
             FROM invoices WHERE client_id = ? ORDER BY invoice_date DESC, id DESC"
        )?;
        
        let invoices = stmt.query_map([client_id], |row| {
            Ok(Invoice {
                id: row.get(0)?,
                invoice_number: row.get(1)?,
                ref_number: row.get(2)?,
                client_id: row.get(3)?,
                client_name: row.get(4)?,
                client_address: row.get(5)?,
                invoice_date: row.get(6)?,
                due_date: row.get(7)?,
                subtotal: f64_to_decimal(row.get(8)?),
                tax_total: f64_to_decimal(row.get(9)?),
                discount_total: f64_to_decimal(row.get(10)?),
                grand_total: f64_to_decimal(row.get(11)?),
                amount_paid: f64_to_decimal(row.get(12)?),
                remaining_debt: f64_to_decimal(row.get(13)?),
                adjustment_label: row.get(14)?,
                adjustment_amount: f64_to_decimal(row.get(15)?),
                total: f64_to_decimal(row.get(16)?),
                notes: row.get(17)?,
                status: {
                    let status_str: String = row.get(18)?;
                    match status_str.as_str() {
                        "paid" => PaymentStatus::Paid,
                        "partial" => PaymentStatus::Partial,
                        _ => PaymentStatus::Unpaid,
                    }
                },
                created_at: row.get(19)?,
                items: Vec::new(),
            })
        })?;
        
        let mut result = Vec::new();
        for invoice in invoices {
            let mut inv = invoice?;
            inv.items = self.get_items(inv.id.unwrap())?;
            result.push(inv);
        }
        
        Ok(result)
    }
}
