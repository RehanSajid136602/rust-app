export interface PrintItem {
  sno: number
  item_name: string
  quantity: number
  price_per_unit: number
}

export interface PrintData {
  docType: string
  date: string
  refNumber: string
  salutation: string
  bodyText: string
  items: PrintItem[]
  subtotal: number
  adjustmentLabel: string
  adjustmentAmount: number
  total: number
}

export interface LedgerEntry {
  id?: number
  client_id: number
  date: string
  description: string
  debit: number
  credit: number
  balance: number
  invoice_id?: number
}

export interface ClientBalance {
  client_id: number
  client_name: string
  total_debit: number
  total_credit: number
  current_balance: number
}

export interface PaymentForm {
  invoice_id: number
  invoice_number: string
  total: number
  amount_paid: number
  amount: number
  date: string
}
