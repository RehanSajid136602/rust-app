import { invoke } from '@tauri-apps/api/core'
import type { Quotation } from '../types'

export async function getAllQuotations(limit = 200, offset = 0): Promise<Quotation[]> {
  return invoke<Quotation[]>('get_all_quotations', { limit, offset })
}

export async function getQuotationById(id: number): Promise<Quotation | null> {
  return invoke<Quotation | null>('get_quotation_by_id', { id })
}

export async function createQuotation(quotation: Quotation): Promise<number> {
  return invoke<number>('create_quotation', { quotation })
}

export async function updateQuotation(quotation: Quotation): Promise<void> {
  return invoke('update_quotation', { quotation })
}

export async function deleteQuotation(id: number): Promise<void> {
  return invoke('delete_quotation', { id })
}

export async function exportQuotationPdf(quotation: Quotation, outputPath: string): Promise<string> {
  return invoke<string>('export_quotation_pdf', { quotation, outputPath })
}

export async function getNextQuotationNumber(): Promise<string> {
  return invoke<string>('get_next_quotation_number')
}
