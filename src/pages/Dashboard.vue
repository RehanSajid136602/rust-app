<template>
  <div class="space-y-6">
    <!-- Stats Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <div class="card">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-500">Total Invoices</p>
            <p class="text-2xl font-bold text-gray-900">{{ stats.invoiceCount }}</p>
          </div>
          <div class="w-12 h-12 bg-primary-100 rounded-lg flex items-center justify-center">
            <span class="text-2xl">📄</span>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-500">Total Quotations</p>
            <p class="text-2xl font-bold text-gray-900">{{ stats.quotationCount }}</p>
          </div>
          <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
            <span class="text-2xl">📋</span>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-500">Total Clients</p>
            <p class="text-2xl font-bold text-gray-900">{{ stats.clientCount }}</p>
          </div>
          <div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center">
            <span class="text-2xl">👥</span>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-500">Outstanding</p>
            <p class="text-2xl font-bold text-red-600">{{ fmt(stats.outstanding) }}</p>
          </div>
          <div class="w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center">
            <span class="text-2xl">⚠️</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Activity -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Recent Invoices -->
      <div class="card">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-semibold text-gray-800">Recent Invoices</h3>
          <router-link to="/invoices" class="text-primary-600 hover:text-primary-700 text-sm font-medium">
            View All →
          </router-link>
        </div>
        
        <div class="space-y-3">
          <div v-for="inv in recentInvoices" :key="inv.id" class="flex items-center justify-between py-2 border-b border-gray-100 last:border-0">
            <div>
              <p class="text-sm font-medium text-gray-900">{{ inv.client_name }}</p>
              <p class="text-xs text-gray-500">{{ inv.invoice_number }} · {{ inv.invoice_date }}</p>
            </div>
            <div class="text-right">
              <p class="text-sm font-medium">{{ fmt(inv.total) }}</p>
              <span :class="statusBadge(inv.status)" class="px-1.5 py-0.5 text-xs font-medium rounded-full capitalize">
                {{ inv.status }}
              </span>
            </div>
          </div>
          <p v-if="recentInvoices.length === 0" class="text-center text-gray-400 py-6 text-sm">
            No invoices yet. <router-link to="/invoices" class="text-primary-600">Create one →</router-link>
          </p>
        </div>
      </div>

      <!-- Recent Quotations -->
      <div class="card">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-semibold text-gray-800">Recent Quotations</h3>
          <router-link to="/quotations" class="text-primary-600 hover:text-primary-700 text-sm font-medium">
            View All →
          </router-link>
        </div>
        
        <div class="space-y-3">
          <div v-for="q in recentQuotations" :key="q.id" class="flex items-center justify-between py-2 border-b border-gray-100 last:border-0">
            <div>
              <p class="text-sm font-medium text-gray-900">{{ q.client_name }}</p>
              <p class="text-xs text-gray-500">{{ q.quotation_number }} · {{ q.quotation_date }}</p>
            </div>
            <div class="text-right">
              <p class="text-sm font-medium">{{ fmt(q.total) }}</p>
              <span :class="statusBadgeQuotation(q.status)" class="px-1.5 py-0.5 text-xs font-medium rounded-full capitalize">
                {{ q.status }}
              </span>
            </div>
          </div>
          <p v-if="recentQuotations.length === 0" class="text-center text-gray-400 py-6 text-sm">
            No quotations yet. <router-link to="/quotations" class="text-primary-600">Create one →</router-link>
          </p>
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-800 mb-4">Quick Actions</h3>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <router-link to="/invoices" class="p-4 border border-gray-200 rounded-lg hover:border-primary-300 hover:bg-primary-50 transition-colors text-center">
          <span class="text-2xl block mb-1">📄</span>
          <span class="text-sm font-medium text-gray-700">New Invoice</span>
        </router-link>
        <router-link to="/quotations" class="p-4 border border-gray-200 rounded-lg hover:border-purple-300 hover:bg-purple-50 transition-colors text-center">
          <span class="text-2xl block mb-1">📋</span>
          <span class="text-sm font-medium text-gray-700">New Quotation</span>
        </router-link>
        <router-link to="/clients" class="p-4 border border-gray-200 rounded-lg hover:border-green-300 hover:bg-green-50 transition-colors text-center">
          <span class="text-2xl block mb-1">👤</span>
          <span class="text-sm font-medium text-gray-700">Add Client</span>
        </router-link>
        <router-link to="/products" class="p-4 border border-gray-200 rounded-lg hover:border-orange-300 hover:bg-orange-50 transition-colors text-center">
          <span class="text-2xl block mb-1">📦</span>
          <span class="text-sm font-medium text-gray-700">Add Product</span>
        </router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Invoice {
  id: number
  invoice_number: string
  client_name: string
  invoice_date: string
  total: number
  status: string
}

interface Quotation {
  id: number
  quotation_number: string
  client_name: string
  quotation_date: string
  total: number
  status: string
}

const stats = ref({
  invoiceCount: 0,
  quotationCount: 0,
  clientCount: 0,
  outstanding: 0,
})

const recentInvoices = ref<Invoice[]>([])
const recentQuotations = ref<Quotation[]>([])

const fmt = (n: number | undefined): string => {
  if (n === undefined || n === null) n = 0
  return new Intl.NumberFormat('en-IN', { minimumFractionDigits: 2 }).format(n)
}

const statusBadge = (s: string): string => {
  const map: Record<string, string> = {
    paid: 'bg-green-50 text-green-700',
    partial: 'bg-yellow-50 text-yellow-700',
    unpaid: 'bg-red-50 text-red-700',
  }
  return map[s] || 'bg-gray-100 text-gray-600'
}

const statusBadgeQuotation = (s: string): string => {
  const map: Record<string, string> = {
    draft: 'bg-gray-100 text-gray-600',
    sent: 'bg-blue-50 text-blue-700',
    accepted: 'bg-green-50 text-green-700',
    rejected: 'bg-red-50 text-red-700',
    converted: 'bg-purple-50 text-purple-700',
  }
  return map[s] || 'bg-gray-100 text-gray-600'
}

const loadDashboard = async () => {
  try {
    const [invoices, quotations, balances, clients] = await Promise.all([
      invoke<Invoice[]>('get_all_invoices', { limit: 5, offset: 0 }),
      invoke<Quotation[]>('get_all_quotations', { limit: 5, offset: 0 }),
      invoke<any[]>('get_all_balances'),
      invoke<any[]>('get_all_clients', { limit: 100, offset: 0 }),
    ])

    recentInvoices.value = invoices
    recentQuotations.value = quotations

    // Get total invoice count from full load
    const allInvoices = await invoke<Invoice[]>('get_all_invoices', { limit: 1000, offset: 0 })
    const allQuotations = await invoke<Quotation[]>('get_all_quotations', { limit: 1000, offset: 0 })

    const outstanding = balances.reduce((sum: number, b: any) => sum + (b.current_balance || 0), 0)

    stats.value = {
      invoiceCount: allInvoices.length,
      quotationCount: allQuotations.length,
      clientCount: clients.length,
      outstanding,
    }
  } catch (e) {
    console.error('Dashboard load error:', e)
  }
}

onMounted(() => {
  loadDashboard()
})
</script>
