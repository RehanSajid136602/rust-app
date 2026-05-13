import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import * as clientService from '../services/clientService'
import * as ledgerService from '../services/ledgerService'
import type { Client, CreateClientRequest } from '../types'

export const useClientStore = defineStore('clients', () => {
  const clients = ref<Client[]>([])
  const searchQuery = ref('')
  const loading = ref(false)

  const filteredClients = computed(() => {
    if (!searchQuery.value) return clients.value
    const q = searchQuery.value.toLowerCase()
    return clients.value.filter(c => c.name.toLowerCase().includes(q))
  })

  async function loadClients() {
    loading.value = true
    try {
      const all = await clientService.getAllClients(200, 0)
      const balances = await ledgerService.getAllBalances()
      const balanceMap = new Map(balances.map(b => [b.client_id, b.current_balance]))
      clients.value = all.map(c => ({ ...c, balance: balanceMap.get(c.id) || 0 }))
    } catch (e) {
      console.error('Failed to load clients:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function createClient(req: CreateClientRequest) {
    await clientService.createClient(req)
    await loadClients()
  }

  async function updateClient(client: Client) {
    await clientService.updateClient(client)
    await loadClients()
  }

  async function deleteClient(id: number) {
    await clientService.deleteClient(id)
    await loadClients()
  }

  return {
    clients,
    searchQuery,
    loading,
    filteredClients,
    loadClients,
    createClient,
    updateClient,
    deleteClient,
  }
})
