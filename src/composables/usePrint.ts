import { ref } from 'vue'

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

export const printData = ref<PrintData | null>(null)
