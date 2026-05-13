export interface InvoiceItem {
  id?: number
  invoice_id?: number
  sno: number
  item_name: string
  quantity: number
  price_per_unit: number
  discount_amount: number
  tax_amount: number
  total_price: number
}

export interface Invoice {
  id?: number
  invoice_number: string
  ref_number: string
  client_id: number | null
  client_name: string
  client_address: string
  invoice_date: string
  due_date: string
  subtotal: number
  tax_total: number
  discount_total: number
  grand_total: number
  amount_paid: number
  remaining_debt: number
  adjustment_label: string
  adjustment_amount: number
  total: number
  notes: string
  status: string
  items: InvoiceItem[]
}
