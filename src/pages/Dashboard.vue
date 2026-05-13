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

    <!-- Charts Section -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Monthly Revenue Bar Chart -->
      <div class="card">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">Monthly Revenue (Last 6 Months)</h3>
        <div class="h-64">
          <Bar v-if="monthlyRevenue" :data="monthlyRevenue" :options="barOptions" />
          <p v-else class="text-center text-gray-400 py-16">No revenue data available.</p>
        </div>
      </div>

      <!-- Payment Status Pie Chart -->
      <div class="card">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">Payment Status</h3>
        <div class="h-64 flex items-center justify-center">
          <div v-if="paymentData" class="w-56">
            <Doughnut :data="paymentData" :options="doughnutOptions" />
          </div>
          <p v-else class="text-center text-gray-400">No invoice data available.</p>
        </div>
      </div>
    </div>

    <!-- Top Clients -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-800 mb-4">Top 5 Clients by Revenue</h3>
      <div v-if="topClients.length > 0" class="space-y-3">
        <div v-for="(c, idx) in topClients" :key="c.name" class="flex items-center justify-between py-2 border-b border-gray-100 last:border-0">
          <div class="flex items-center space-x-3">
            <span class="text-sm font-bold text-gray-400 w-6">{{ idx + 1 }}</span>
            <span class="text-sm font-medium text-gray-900">{{ c.name }}</span>
          </div>
          <div class="flex items-center space-x-6">
            <div class="w-48 bg-gray-100 rounded-full h-2">
              <div class="bg-accent-600 h-2 rounded-full" :style="{ width: c.percentage + '%' }"></div>
            </div>
            <span class="text-sm font-semibold text-gray-800 w-24 text-right">{{ fmt(c.total) }}</span>
          </div>
        </div>
      </div>
      <p v-else class="text-center text-gray-400 py-4 text-sm">No client data available.</p>
    </div>

    <!-- Quick Actions -->
    <div class="card">
      <h3 class="text-lg font-semibold text-gray-800 mb-4">Quick Actions</h3>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <router-link to="/invoices" class="p-4 border border-gray-200 rounded-lg hover:border-accent-300 hover:bg-accent-50 transition-colors text-center">
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
import { Bar, Doughnut } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  ArcElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js'

ChartJS.register(CategoryScale, LinearScale, BarElement, ArcElement, Title, Tooltip, Legend)

interface Invoice {
  id: number
  invoice_number: string
  client_name: string
  invoice_date: string
  total: number
  status: string
  amount_paid: number
  remaining_debt: number
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
const monthlyRevenue = ref<any>(null)
const paymentData = ref<any>(null)
const topClients = ref<{ name: string; total: number; percentage: number }[]>([])

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

const barOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { display: false } },
  scales: {
    y: { beginAtZero: true, ticks: { callback: (v: any) => 'Rs. ' + v.toLocaleString('en-IN') } },
  },
}

const doughnutOptions = {
  responsive: true,
  maintainAspectRatio: true,
  plugins: {
    legend: { position: 'bottom' as const },
  },
}

const computeCharts = (allInvoices: Invoice[]) => {
  // Monthly Revenue
  const now = new Date()
  const months: string[] = []
  const revenue: number[] = []
  for (let i = 5; i >= 0; i--) {
    const d = new Date(now.getFullYear(), now.getMonth() - i, 1)
    months.push(d.toLocaleDateString('en-IN', { month: 'short', year: '2-digit' }))
    revenue.push(0)
  }

  let totalPaid = 0
  let totalUnpaid = 0
  const clientTotals = new Map<string, number>()

  for (const inv of allInvoices) {
    // Monthly revenue
    if (inv.invoice_date) {
      const parts = inv.invoice_date.split('-')
      if (parts.length >= 2) {
        const invDate = new Date(parseInt(parts[0]), parseInt(parts[1]) - 1, 1)
        for (let i = 5; i >= 0; i--) {
          const monthDate = new Date(now.getFullYear(), now.getMonth() - i, 1)
          if (invDate.getFullYear() === monthDate.getFullYear() && invDate.getMonth() === monthDate.getMonth()) {
            revenue[5 - i] += inv.total
            break
          }
        }
      }
    }

    // Payment status
    if (inv.status === 'paid') totalPaid += inv.total
    else totalUnpaid += inv.remaining_debt || inv.total - (inv.amount_paid || 0)

    // Client totals
    const prev = clientTotals.get(inv.client_name) || 0
    clientTotals.set(inv.client_name, prev + inv.total)
  }

  monthlyRevenue.value = {
    labels: months,
    datasets: [{
      label: 'Revenue',
      data: revenue,
      backgroundColor: '#1a2540',
      borderRadius: 4,
    }],
  }

  if (totalPaid > 0 || totalUnpaid > 0) {
    paymentData.value = {
      labels: ['Paid', 'Outstanding'],
      datasets: [{
        data: [totalPaid, totalUnpaid],
        backgroundColor: ['#22c55e', '#ef4444'],
        borderWidth: 0,
      }],
    }
  }

  // Top 5 clients
  const sorted = [...clientTotals.entries()]
    .sort((a, b) => b[1] - a[1])
    .slice(0, 5)
  const maxTotal = sorted.length > 0 ? sorted[0][1] : 1
  topClients.value = sorted.map(([name, total]) => ({
    name,
    total,
    percentage: Math.round((total / maxTotal) * 100),
  }))
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

    const allInvoices = await invoke<Invoice[]>('get_all_invoices', { limit: 1000, offset: 0 })
    const allQuotations = await invoke<Quotation[]>('get_all_quotations', { limit: 1000, offset: 0 })

    const outstanding = balances.reduce((sum: number, b: any) => sum + (b.current_balance || 0), 0)

    stats.value = {
      invoiceCount: allInvoices.length,
      quotationCount: allQuotations.length,
      clientCount: clients.length,
      outstanding,
    }

    computeCharts(allInvoices)
  } catch (e) {
    console.error('Dashboard load error:', e)
  }
}

onMounted(() => {
  loadDashboard()
})
</script>
