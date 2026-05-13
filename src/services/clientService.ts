import { invoke } from '@tauri-apps/api/core'
import type { Client, CreateClientRequest, ClientBalance } from '../types'

export async function getAllClients(limit = 200, offset = 0): Promise<Client[]> {
  return invoke<Client[]>('get_all_clients', { limit, offset })
}

export async function getClientById(id: number): Promise<Client> {
  return invoke<Client>('get_client_by_id', { id })
}

export async function createClient(req: CreateClientRequest): Promise<number> {
  return invoke<number>('create_client', { req })
}

export async function updateClient(client: Client): Promise<void> {
  return invoke('update_client', { client })
}

export async function deleteClient(id: number): Promise<void> {
  return invoke('delete_client', { id })
}

export async function getClientBalance(clientId: number): Promise<ClientBalance> {
  return invoke<ClientBalance>('get_client_balance', { clientId })
}

export async function getAllBalances(): Promise<ClientBalance[]> {
  return invoke<ClientBalance[]>('get_all_balances')
}
