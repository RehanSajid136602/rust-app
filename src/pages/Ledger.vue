<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <select v-model="selectedClientId" class="input w-72" @change="onClientChange">
          <option :value="null">Select client to view ledger...</option>
          <option v-for="c in clients" :key="c.id" :value="c.id">{{ c.name }}</option>
        </select>
        <div v-if="balance" class="flex items-center space-x-6 ml-4">
          <div>
            <span class="text-xs text-gray-500">Total Debit</span>
            <p class="text-sm font-semibold text-red-600">{{ fmt(balance.total_debit) }}</p>
          </div>
          <div>
            <span class="text-xs text-gray-500">Total Credit</span>
            <p class="text-sm font-semibold text-green-600">{{ fmt(balance.total_credit) }}</p>
          </div>
          <div class="pl-4 border-l border-gray-200">
            <span class="text-xs text-gray-500">Balance</span>
            <p class="text-sm font-bold" :class="balance.current_balance > 0 ? 'text-red-600' : 'text-green-600'">
              {{ fmt(balance.current_balance) }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Ledger Table -->
    <div v-if="selectedClientId" class="card">
      <div class="table-container">
        <table class="table">
          <thead class="table-header">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Date</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Description</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Debit</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Credit</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Balance</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="entry in entries" :key="entry.id" class="table-row">
              <td class="px-4 py-3 text-sm text-gray-700 whitespace-nowrap">{{ entry.date }}</td>
              <td class="px-4 py-3 text-sm text-gray-900">{{ entry.description }}</td>
              <td class="px-4 py-3 text-sm text-right font-medium text-red-600">
                {{ entry.debit > 0 ? fmt(entry.debit) : '-' }}
              </td>
              <td class="px-4 py-3 text-sm text-right font-medium text-green-600">
                {{ entry.credit > 0 ? fmt(entry.credit) : '-' }}
              </td>
              <td class="px-4 py-3 text-sm text-right font-semibold" :class="entry.balance > 0 ? 'text-red-600' : 'text-gray-900'">
                {{ fmt(entry.balance) }}
              </td>
            </tr>
            <tr v-if="entries.length === 0">
              <td colspan="5" class="px-6 py-8 text-center text-gray-500">
                {{ loading ? 'Loading...' : 'No ledger entries found for this client.' }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Empty state -->
    <div v-else class="card text-center py-12">
      <span class="text-4xl block mb-3">📒</span>
      <p class="text-gray-500">Select a client above to view their transaction history.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Client, LedgerEntry, ClientBalance } from '../types'

const clients = ref<Client[]>([])
const entries = ref<LedgerEntry[]>([])
const balance = ref<ClientBalance | null>(null)
const selectedClientId = ref<number | null>(null)
const loading = ref(false)

const fmt = (n: number | undefined): string => {
  if (n === undefined || n === null) n = 0
  return new Intl.NumberFormat('en-IN', { minimumFractionDigits: 2 }).format(n)
}

const loadClients = async () => {
  try {
    clients.value = await invoke<Client[]>('get_all_clients', { limit: 200, offset: 0 })
  } catch (e) {
    console.error('Failed to load clients:', e)
  }
}

const onClientChange = async () => {
  if (!selectedClientId.value) {
    entries.value = []
    balance.value = null
    return
  }
  loading.value = true
  try {
    const [ledger, bal] = await Promise.all([
      invoke<LedgerEntry[]>('get_client_ledger', { clientId: selectedClientId.value }),
      invoke<ClientBalance>('get_client_balance_summary', { clientId: selectedClientId.value }),
    ])
    entries.value = ledger || []
    balance.value = bal || null
  } catch (e) {
    console.error('Failed to load ledger:', e)
    entries.value = []
    balance.value = null
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadClients()
})
</script>
