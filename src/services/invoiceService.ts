import { invoke } from '@tauri-apps/api/core'
import type { Invoice } from '../types'

export async function getAllInvoices(limit = 200, offset = 0): Promise<Invoice[]> {
  return invoke<Invoice[]>('get_all_invoices', { limit, offset })
}

export async function getInvoiceById(id: number): Promise<Invoice | null> {
  return invoke<Invoice | null>('get_invoice_by_id', { id })
}

export async function getInvoicesByClient(clientId: number): Promise<Invoice[]> {
  return invoke<Invoice[]>('get_invoices_by_client', { clientId })
}

export async function createInvoice(invoice: Invoice): Promise<number> {
  return invoke<number>('create_invoice', { invoice })
}

export async function updateInvoice(invoice: Invoice): Promise<void> {
  return invoke('update_invoice', { invoice })
}

export async function deleteInvoice(id: number): Promise<void> {
  return invoke('delete_invoice', { id })
}

export async function exportInvoicePdf(invoice: Invoice, outputPath: string): Promise<string> {
  return invoke<string>('export_invoice_pdf', { invoice, outputPath })
}

export async function generateRefNumber(): Promise<string> {
  return invoke<string>('generate_ref_number')
}

export async function getNextInvoiceNumber(): Promise<string> {
  return invoke<string>('get_next_invoice_number')
}

export async function exportInvoicesExcel(outputPath: string): Promise<string> {
  return invoke<string>('export_invoices_excel', { outputPath })
}
