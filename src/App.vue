<template>
  <div id="app-content" class="min-h-screen flex">
    <!-- Sidebar -->
    <aside class="w-64 bg-white border-r border-gray-200">
      <div class="p-6">
        <h1 class="text-xl font-bold text-primary-600">Zahra Invoice</h1>
        <p class="text-sm text-gray-500 mt-1">Enterprise</p>
      </div>
      
      <nav class="mt-6">
        <router-link
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="flex items-center px-6 py-3 text-gray-700 hover:bg-gray-100 hover:text-primary-600 transition-colors"
          :class="{ 'bg-primary-50 text-primary-600 border-r-4 border-primary-600': $route.path === item.path }"
        >
          <span class="text-lg mr-3">{{ item.icon }}</span>
          <span class="font-medium">{{ item.name }}</span>
        </router-link>
      </nav>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 overflow-auto">
      <header class="bg-white border-b border-gray-200 px-8 py-4">
        <div class="flex items-center justify-between">
          <h2 class="text-2xl font-semibold text-gray-800">{{ pageTitle }}</h2>
          <div class="flex items-center space-x-4">
            <span class="text-sm text-gray-500">{{ currentDate }}</span>
          </div>
        </div>
      </header>

      <div class="p-8">
        <router-view />
      </div>
    </main>
  </div>

  <!-- Print Template (hidden, shown only during print) -->
  <div id="print-container" class="print-only">
    <InvoicePrintTemplate :data="printData" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import InvoicePrintTemplate from './components/InvoicePrintTemplate.vue'
import { printData } from './composables/usePrint'

const route = useRoute()

const navItems = [
  { path: '/', name: 'Dashboard', icon: '📊' },
  { path: '/products', name: 'Products', icon: '📦' },
  { path: '/clients', name: 'Clients', icon: '👥' },
  { path: '/invoices', name: 'Invoices', icon: '📄' },
  { path: '/quotations', name: 'Quotations', icon: '📋' },
  { path: '/settings', name: 'Settings', icon: '⚙️' },
]

const pageTitle = computed(() => {
  const item = navItems.find(i => i.path === route.path)
  return item?.name || 'Dashboard'
})

const currentDate = new Date().toLocaleDateString('en-IN', {
  weekday: 'long',
  year: 'numeric',
  month: 'long',
  day: 'numeric',
})
</script>
