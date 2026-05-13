export interface QuotationItem {
  id?: number
  quotation_id?: number
  sno: number
  item_name: string
  quantity: number
  price_per_unit: number
  discount_amount: number
  tax_amount: number
  total_price: number
}

export interface Quotation {
  id?: number
  quotation_number: string
  ref_number: string
  client_id: number | null
  client_name: string
  client_address: string
  quotation_date: string
  valid_until: string
  subtotal: number
  tax_total: number
  discount_total: number
  grand_total: number
  adjustment_label: string
  adjustment_amount: number
  total: number
  notes: string
  status: string
  items: QuotationItem[]
}
