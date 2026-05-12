<template>
  <div class="space-y-6">
    <!-- Header Actions -->
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search products..."
          class="input w-64"
        />
        <span class="text-sm text-gray-500">{{ products.length }} products</span>
      </div>
      <div class="flex items-center space-x-3">
        <div class="relative" v-if="showImportMenu">
          <div class="absolute right-0 top-full mt-1 bg-white border border-gray-200 rounded-lg shadow-lg z-50 w-48">
            <button @click="importExcel" class="w-full text-left px-4 py-2 text-sm hover:bg-gray-50 rounded-t-lg">📊 Import Excel</button>
            <button @click="importPdf" class="w-full text-left px-4 py-2 text-sm hover:bg-gray-50 rounded-b-lg">📄 Import PDF</button>
          </div>
        </div>
        <button @click="showImportMenu = !showImportMenu" class="btn-secondary">
          📥 Import
        </button>
        <button @click="openCreateModal" class="btn-primary">
          + Add Product
        </button>
      </div>
    </div>

    <!-- Products Table -->
    <div class="card">
      <div class="table-container">
        <table class="table">
          <thead class="table-header">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Name</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Price</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Unit</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">HSN</th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">Actions</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="product in filteredProducts" :key="product.id" class="table-row">
              <td class="px-6 py-4 text-sm font-medium text-gray-900">{{ product.name }}</td>
              <td class="px-6 py-4 text-sm text-gray-900">{{ formatCurrency(product.price_per_unit) }}</td>
              <td class="px-6 py-4 text-sm text-gray-700">{{ product.unit }}</td>
              <td class="px-6 py-4 text-sm text-gray-700">{{ product.hsn_code || '-' }}</td>
              <td class="px-6 py-4 text-right">
                <button @click="editProduct(product)" class="text-primary-600 hover:text-primary-700 mr-3">
                  Edit
                </button>
                <button @click="deleteProduct(product.id)" class="text-danger hover:text-red-700">
                  Delete
                </button>
              </td>
            </tr>
            <tr v-if="filteredProducts.length === 0">
              <td colspan="5" class="px-6 py-8 text-center text-gray-500">
                No products found. Add your first product!
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div v-if="showModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 w-full max-w-md">
        <h3 class="text-lg font-semibold mb-4">{{ editingProduct ? 'Edit Product' : 'Add Product' }}</h3>
        
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Name *</label>
            <input v-model="form.name" type="text" class="input" />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Price *</label>
            <input v-model.number="form.price_per_unit" type="number" step="0.01" class="input" />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Unit *</label>
            <input v-model="form.unit" type="text" class="input" placeholder="e.g., pcs, kg, m" />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">HSN Code</label>
            <input v-model="form.hsn_code" type="text" class="input" />
          </div>
        </div>
        
        <div class="flex justify-end space-x-3 mt-6">
          <button @click="closeModal" class="btn-secondary">Cancel</button>
          <button @click="saveProduct" class="btn-primary">Save</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

interface Product {
  id: number
  name: string
  price_per_unit: number
  unit: string
  hsn_code: string
}

const products = ref<Product[]>([])
const searchQuery = ref('')
const showModal = ref(false)
const showImportMenu = ref(false)
const editingProduct = ref<Product | null>(null)

const form = ref({
  name: '',
  price_per_unit: 0,
  unit: 'pcs',
  hsn_code: '',
})

const filteredProducts = computed(() => {
  if (!searchQuery.value) return products.value
  const query = searchQuery.value.toLowerCase()
  return products.value.filter(p => p.name.toLowerCase().includes(query))
})

const formatCurrency = (amount: number): string => {
  return new Intl.NumberFormat('en-IN', {
    minimumFractionDigits: 2,
  }).format(amount)
}

const loadProducts = async () => {
  try {
    const result = await invoke<Product[]>('get_all_products')
    console.log('get_all_products returned:', result?.length, 'products', result)
    products.value = result || []
  } catch (error) {
    console.error('Failed to load products:', String(error))
    alert('Failed to load products: ' + String(error))
  }
}

const saveProduct = async () => {
  if (!form.value.name.trim()) {
    alert('Product name is required.')
    return
  }
  try {
    if (editingProduct.value) {
      await invoke('update_product', { id: editingProduct.value.id, req: form.value })
    } else {
      const id = await invoke<number>('create_product', { req: form.value })
      console.log('Created product with id:', id)
    }
    closeModal()
    await loadProducts()
  } catch (error) {
    console.error('Failed to save product:', error)
    alert('Failed to save: ' + String(error))
  }
}

const openCreateModal = () => {
  editingProduct.value = null
  form.value = { name: '', price_per_unit: 0, unit: 'pcs', hsn_code: '' }
  showModal.value = true
}

const editProduct = (product: Product) => {
  editingProduct.value = product
  form.value = { ...product }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingProduct.value = null
}

const deleteProduct = async (id: number) => {
  if (!confirm('Are you sure you want to delete this product?')) return
  try {
    await invoke('delete_product', { id })
    loadProducts()
  } catch (error) {
    console.error('Failed to delete product:', error)
    alert('Failed to delete product: ' + error)
  }
}

const importExcel = async () => {
  showImportMenu.value = false
  try {
    const file = await open({
      filters: [{ name: 'Excel Files', extensions: ['xlsx', 'xls'] }],
      multiple: false,
    })
    if (file) {
      const result = await invoke<string>('import_products_excel', { path: file })
      alert(result)
      loadProducts()
    }
  } catch (e) {
    alert('Import error: ' + e)
  }
}

const importPdf = async () => {
  showImportMenu.value = false
  try {
    const file = await open({
      filters: [{ name: 'PDF Files', extensions: ['pdf'] }],
      multiple: false,
    })
    if (file) {
      const result = await invoke<string>('import_products_pdf', { path: file })
      alert(result)
      loadProducts()
    }
  } catch (e) {
    alert('Import error: ' + e)
  }
}

onMounted(() => {
  loadProducts()
})
</script>
