import { invoke } from '@tauri-apps/api/core'
import type { Product, CreateProductRequest } from '../types'

export async function getAllProducts(): Promise<Product[]> {
  return invoke<Product[]>('get_all_products')
}

export async function getProductById(id: number): Promise<Product> {
  return invoke<Product>('get_product_by_id', { id })
}

export async function searchProducts(query: string): Promise<Product[]> {
  return invoke<Product[]>('search_products', { query })
}

export async function createProduct(req: CreateProductRequest): Promise<number> {
  return invoke<number>('create_product', { req })
}

export async function updateProduct(id: number, req: Partial<CreateProductRequest>): Promise<void> {
  return invoke('update_product', { id, req })
}

export async function deleteProduct(id: number): Promise<void> {
  return invoke('delete_product', { id })
}

export async function importProductsExcel(path: string): Promise<string> {
  return invoke<string>('import_products_excel', { path })
}

export async function importProductsPdf(path: string): Promise<string> {
  return invoke<string>('import_products_pdf', { path })
}
