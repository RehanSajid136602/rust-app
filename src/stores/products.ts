import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import * as productService from '../services/productService'
import type { Product, CreateProductRequest } from '../types'

export const useProductStore = defineStore('products', () => {
  const products = ref<Product[]>([])
  const searchQuery = ref('')
  const loading = ref(false)

  const filteredProducts = computed(() => {
    if (!searchQuery.value) return products.value
    const q = searchQuery.value.toLowerCase()
    return products.value.filter(p => p.name.toLowerCase().includes(q))
  })

  async function loadProducts() {
    loading.value = true
    try {
      products.value = await productService.getAllProducts()
    } catch (e) {
      console.error('Failed to load products:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function createProduct(req: CreateProductRequest) {
    const id = await productService.createProduct(req)
    await loadProducts()
    return id
  }

  async function updateProduct(id: number, req: Partial<CreateProductRequest>) {
    await productService.updateProduct(id, req)
    await loadProducts()
  }

  async function deleteProduct(id: number) {
    await productService.deleteProduct(id)
    await loadProducts()
  }

  async function importExcel(path: string) {
    const result = await productService.importProductsExcel(path)
    await loadProducts()
    return result
  }

  async function importPdf(path: string) {
    const result = await productService.importProductsPdf(path)
    await loadProducts()
    return result
  }

  return {
    products,
    searchQuery,
    loading,
    filteredProducts,
    loadProducts,
    createProduct,
    updateProduct,
    deleteProduct,
    importExcel,
    importPdf,
  }
})
