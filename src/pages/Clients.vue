<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <input v-model="searchQuery" type="text" placeholder="Search clients..." class="input w-64" />
      <button @click="openCreateModal" class="btn-primary">+ Add Client</button>
    </div>

    <div class="card">
      <div class="table-container">
        <table class="table">
          <thead class="table-header">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Name</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Phone</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Email</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">GSTIN</th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">Balance</th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">Actions</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="client in filteredClients" :key="client.id" class="table-row">
              <td class="px-6 py-4 text-sm font-medium text-gray-900">{{ client.name }}</td>
              <td class="px-6 py-4 text-sm text-gray-700">{{ client.phone || '-' }}</td>
              <td class="px-6 py-4 text-sm text-gray-700">{{ client.email || '-' }}</td>
              <td class="px-6 py-4 text-sm text-gray-700">{{ client.gstin || '-' }}</td>
              <td class="px-6 py-4 text-sm text-right" :class="client.balance > 0 ? 'text-danger' : 'text-success'">
                {{ formatCurrency(client.balance) }}
              </td>
              <td class="px-6 py-4 text-right">
                <button @click="editClient(client)" class="text-primary-600 hover:text-primary-700 mr-3">Edit</button>
                <button @click="deleteClient(client.id)" class="text-danger hover:text-red-700">Delete</button>
              </td>
            </tr>
            <tr v-if="filteredClients.length === 0">
              <td colspan="6" class="px-6 py-8 text-center text-gray-500">No clients found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Modal -->
    <div v-if="showModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 w-full max-w-md">
        <h3 class="text-lg font-semibold mb-4">{{ editingClient ? 'Edit Client' : 'Add Client' }}</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Name *</label>
            <input v-model="form.name" type="text" class="input" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Phone</label>
            <input v-model="form.phone" type="text" class="input" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
            <input v-model="form.email" type="email" class="input" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">GSTIN</label>
            <input v-model="form.gstin" type="text" class="input" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Address</label>
            <textarea v-model="form.address" class="input" rows="3"></textarea>
          </div>
        </div>
        <div class="flex justify-end space-x-3 mt-6">
          <button @click="closeModal" class="btn-secondary">Cancel</button>
          <button @click="saveClient" class="btn-primary">Save</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Client {
  id: number
  name: string
  phone: string
  email: string
  gstin: string
  address: string
  balance: number
}

const clients = ref<Client[]>([])
const searchQuery = ref('')
const showModal = ref(false)
const editingClient = ref<Client | null>(null)

const form = ref({ name: '', phone: '', email: '', gstin: '', address: '' })

const filteredClients = computed(() => {
  if (!searchQuery.value) return clients.value
  const query = searchQuery.value.toLowerCase()
  return clients.value.filter(c => c.name.toLowerCase().includes(query))
})

const formatCurrency = (amount: number): string => {
  return new Intl.NumberFormat('en-IN', { style: 'currency', currency: 'INR' }).format(amount)
}

const loadClients = async () => {
  try {
    const all = await invoke<Client[]>('get_all_clients', { limit: 100, offset: 0 })
    const balances = await invoke<any[]>('get_all_balances')
    const balanceMap = new Map(balances.map(b => [b.client_id, b.current_balance]))
    clients.value = all.map(c => ({ ...c, balance: balanceMap.get(c.id) || 0 }))
  } catch (error) {
    console.error('Failed to load clients:', error)
  }
}

const openCreateModal = () => {
  editingClient.value = null
  form.value = { name: '', phone: '', email: '', gstin: '', address: '' }
  showModal.value = true
}

const editClient = (client: Client) => {
  editingClient.value = client
  form.value = { ...client }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingClient.value = null
}

const saveClient = async () => {
  try {
    if (editingClient.value) {
      await invoke('update_client', { client: { ...editingClient.value, ...form.value } })
    } else {
      await invoke('create_client', { req: form.value })
    }
    closeModal()
    loadClients()
  } catch (error) {
    console.error('Failed to save client:', error)
    alert('Failed to save client: ' + error)
  }
}

const deleteClient = async (id: number) => {
  if (!confirm('Are you sure?')) return
  try {
    await invoke('delete_client', { id })
    loadClients()
  } catch (error) {
    console.error('Failed to delete client:', error)
    alert('Failed to delete: ' + error)
  }
}

onMounted(() => { loadClients() })
</script>
