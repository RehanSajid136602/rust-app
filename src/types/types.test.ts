import { describe, it, expect } from 'vitest'

import type { Product, Client, Invoice, Quotation, CompanySettings, LedgerEntry, ClientBalance } from '../types'

describe('Type definitions', () => {
  it('Product type has required fields', () => {
    const p: Product = {
      id: 1, name: 'Test', price_per_unit: 100, unit: 'pcs', hsn_code: '1234',
    }
    expect(p.name).toBe('Test')
    expect(p.price_per_unit).toBe(100)
  })

  it('Client type has required fields', () => {
    const c: Client = {
      id: 1, name: 'Test', phone: '', email: '', gstin: '', address: '', balance: 0,
    }
    expect(c.name).toBe('Test')
    expect(c.balance).toBe(0)
  })

  it('Invoice type has required fields', () => {
    const inv: Invoice = {
      invoice_number: 'INV-001', ref_number: '', client_id: 1,
      client_name: 'Test', client_address: '',
      invoice_date: '2024-01-01', due_date: '',
      subtotal: 100, tax_total: 0, discount_total: 0, grand_total: 100,
      amount_paid: 0, remaining_debt: 100,
      adjustment_label: '', adjustment_amount: 0, total: 100,
      notes: '', status: 'unpaid', items: [],
    }
    expect(inv.invoice_number).toBe('INV-001')
    expect(inv.items).toHaveLength(0)
  })

  it('Quotation type has required fields', () => {
    const q: Quotation = {
      quotation_number: 'Q-001', ref_number: '', client_id: 1,
      client_name: 'Test', client_address: '',
      quotation_date: '2024-01-01', valid_until: '',
      subtotal: 100, tax_total: 0, discount_total: 0, grand_total: 100,
      adjustment_label: '', adjustment_amount: 0, total: 100,
      notes: '', status: 'draft', items: [],
    }
    expect(q.quotation_number).toBe('Q-001')
  })

  it('CompanySettings type accepts optional fields', () => {
    const s: CompanySettings = {
      company_name: 'Test Co', address: '', phone: '', email: '', gst_number: '', pan_number: '',
    }
    expect(s.company_name).toBe('Test Co')
  })

  it('LedgerEntry type handles debit/credit', () => {
    const entry: LedgerEntry = {
      client_id: 1, date: '2024-01-01', description: 'Invoice #1',
      debit: 100, credit: 0, balance: 100,
    }
    expect(entry.debit).toBe(100)
    expect(entry.credit).toBe(0)
  })

  it('ClientBalance type computes balance', () => {
    const bal: ClientBalance = {
      client_id: 1, client_name: 'Test',
      total_debit: 500, total_credit: 300, current_balance: 200,
    }
    expect(bal.current_balance).toBe(200)
  })
})
