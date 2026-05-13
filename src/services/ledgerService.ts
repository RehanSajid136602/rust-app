import { invoke } from '@tauri-apps/api/core'
import type { LedgerEntry, ClientBalance } from '../types'

export async function getClientLedger(clientId: number): Promise<LedgerEntry[]> {
  return invoke<LedgerEntry[]>('get_client_ledger', { clientId })
}

export async function getLedgerEntryById(id: number): Promise<LedgerEntry | null> {
  return invoke<LedgerEntry | null>('get_ledger_entry_by_id', { id })
}

export async function getClientBalanceSummary(clientId: number): Promise<ClientBalance> {
  return invoke<ClientBalance>('get_client_balance_summary', { clientId })
}

export async function getAllBalances(): Promise<ClientBalance[]> {
  return invoke<ClientBalance[]>('get_all_balances')
}

export async function addDebitEntry(
  clientId: number, amount: number, description: string, date: string
): Promise<number> {
  return invoke<number>('add_debit_entry', { clientId, amount, description, date })
}

export async function addCreditEntry(
  clientId: number, amount: number, description: string, date: string
): Promise<number> {
  return invoke<number>('add_credit_entry', { clientId, amount, description, date })
}

export async function getClientsWithBalance(): Promise<ClientBalance[]> {
  return invoke<ClientBalance[]>('get_clients_with_balance')
}
