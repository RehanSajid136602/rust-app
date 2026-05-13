export interface Product {
  id: number
  name: string
  price_per_unit: number
  unit: string
  hsn_code: string
}

export interface CreateProductRequest {
  name: string
  price_per_unit: number
  unit: string
  hsn_code: string
}

export type UpdateProductRequest = CreateProductRequest

export interface ProductSearchResult {
  id: number
  name: string
  price_per_unit: number
  unit: string
  hsn_code: string
}
