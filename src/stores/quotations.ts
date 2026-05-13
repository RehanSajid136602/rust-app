import { defineStore } from 'pinia'
import { ref, reactive, computed } from 'vue'
import * as quotationService from '../services/quotationService'
import * as invoiceService from '../services/invoiceService'
import * as settingsService from '../services/settingsService'
import type { Quotation, QuotationItem, Invoice } from '../types'
import { printData } from '../composables/usePrint'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { nextTick } from 'vue'

export const useQuotationStore = defineStore('quotations', () => {
  const quotations = ref<Quotation[]>([])
  const searchQuery = ref('')
  const statusFilter = ref('')
  const loading = ref(false)
  const saving = ref(false)
  const exportedPath = ref<string | null>(null)

  const emptyForm = (): Quotation => ({
    quotation_number: '', ref_number: '', client_id: null, client_name: '', client_address: '',
    quotation_date: new Date().toISOString().split('T')[0], valid_until: '',
    subtotal: 0, tax_total: 0, discount_total: 0, grand_total: 0,
    adjustment_label: 'Round Off', adjustment_amount: 0, total: 0, notes: '',
    status: 'draft', items: [],
  })

  const form = reactive<Quotation>(emptyForm())

  const filteredQuotations = computed(() => {
    let list = quotations.value
    if (statusFilter.value) list = list.filter(q => q.status === statusFilter.value)
    if (searchQuery.value) {
      const q = searchQuery.value.toLowerCase()
      list = list.filter(qu =>
        qu.client_name.toLowerCase().includes(q) || qu.quotation_number.toLowerCase().includes(q)
      )
    }
    return list
  })

  function fmt(n: number | undefined): string {
    if (n === undefined || n === null) n = 0
    return new Intl.NumberFormat('en-IN', { minimumFractionDigits: 2 }).format(n)
  }

  const itemTotal = (item: QuotationItem) => {
    return fmt(item.quantity * item.price_per_unit - item.discount_amount + item.tax_amount)
  }

  const totals = computed(() => {
    let subtotal = 0, discount = 0, tax = 0
    for (const item of form.items) {
      subtotal += item.quantity * item.price_per_unit
      discount += item.discount_amount
      tax += item.tax_amount
    }
    const grand = subtotal - discount + tax
    return {
      subtotal, discountTotal: discount, taxTotal: tax,
      grandTotal: grand, netAmount: grand - (form.adjustment_amount || 0),
    }
  })

  function recalc() {
    const t = totals.value
    form.subtotal = t.subtotal
    form.tax_total = t.taxTotal
    form.discount_total = t.discountTotal
    form.grand_total = t.grandTotal
    form.total = t.netAmount
  }

  function addItem() {
    form.items.push({
      sno: form.items.length + 1, item_name: '', quantity: 1,
      price_per_unit: 0, discount_amount: 0, tax_amount: 0, total_price: 0,
    })
  }

  function removeItem(idx: number) {
    form.items.splice(idx, 1)
    form.items.forEach((item, i) => (item.sno = i + 1))
  }

  async function loadQuotations() {
    loading.value = true
    try {
      quotations.value = await quotationService.getAllQuotations(200, 0)
    } catch (e) {
      console.error(e)
    } finally {
      loading.value = false
    }
  }

  async function openCreateModal() {
    Object.assign(form, emptyForm())
    try {
      form.ref_number = await invoiceService.generateRefNumber()
    } catch { /* fallback */ }
  }

  function editForm(q: Quotation) {
    Object.assign(form, JSON.parse(JSON.stringify(q)))
  }

  function resetForm() {
    Object.assign(form, emptyForm())
  }

  async function saveQuotation(): Promise<boolean> {
    if (!form.client_name || form.items.length === 0) {
      alert('Client name and at least one item are required.')
      return false
    }
    if (form.items.some(i => !i.item_name.trim())) {
      alert('All items must have a name.')
      return false
    }
    recalc()
    saving.value = true
    try {
      if (form.id && form.quotation_number) {
        await quotationService.updateQuotation({ ...form })
      } else {
        await quotationService.createQuotation({ ...form })
      }
      await loadQuotations()
      return true
    } catch (e) {
      alert('Error: ' + e)
      return false
    } finally {
      saving.value = false
    }
  }

  async function deleteQuotation(id: number) {
    if (!confirm('Delete this quotation?')) return
    try {
      await quotationService.deleteQuotation(id)
      await loadQuotations()
    } catch (e) {
      alert('Error: ' + e)
    }
  }

  async function convertToInvoice(q: Quotation): Promise<boolean> {
    if (!confirm(`Convert "${q.quotation_number}" to an invoice?`)) return false
    saving.value = true
    try {
      const invoice: Invoice = {
        invoice_number: '', ref_number: q.ref_number, client_id: q.client_id,
        client_name: q.client_name, client_address: q.client_address,
        invoice_date: new Date().toISOString().split('T')[0], due_date: q.valid_until,
        subtotal: q.subtotal, tax_total: q.tax_total, discount_total: q.discount_total,
        grand_total: q.grand_total, amount_paid: 0, remaining_debt: q.total,
        adjustment_label: q.adjustment_label, adjustment_amount: q.adjustment_amount,
        total: q.total, notes: q.notes + `\n(Converted from ${q.quotation_number})`,
        status: 'unpaid',
        items: q.items.map((item, i) => ({
          sno: i + 1, item_name: item.item_name, quantity: item.quantity,
          price_per_unit: item.price_per_unit, discount_amount: item.discount_amount,
          tax_amount: item.tax_amount, total_price: item.total_price,
        })),
      }
      await invoiceService.createInvoice(invoice)
      q.status = 'converted'
      await quotationService.updateQuotation({ ...q })
      await loadQuotations()
      alert('Converted to invoice successfully!')
      return true
    } catch (e) {
      alert('Error: ' + e)
      return false
    } finally {
      saving.value = false
    }
  }

  async function exportPdfRust() {
    recalc()
    try {
      const quotation = { ...form }
      const path = `${quotation.quotation_number || 'quotation'}.pdf`
      const result = await quotationService.exportQuotationPdf(quotation, path)
      exportedPath.value = result
      return result
    } catch (e) {
      alert('Export failed: ' + e)
      return null
    }
  }

  async function exportPrintTemplate() {
    recalc()
    try {
      const settings = await settingsService.getCompanySettings()
      printData.value = {
        docType: 'Quotation',
        date: form.quotation_date,
        refNumber: form.quotation_number,
        salutation: settings?.salutation || 'Respected Sir,',
        bodyText: settings?.body_text || '',
        items: form.items.map(item => ({
          sno: item.sno,
          item_name: item.item_name,
          quantity: item.quantity,
          price_per_unit: item.price_per_unit,
        })),
        subtotal: form.subtotal,
        adjustmentLabel: form.adjustment_label || '',
        adjustmentAmount: form.adjustment_amount || 0,
        total: form.total,
      }
      const title = `Quotation ${form.quotation_number}`
      document.title = title
      await getCurrentWindow().setTitle(title)
      await nextTick()
      setTimeout(() => window.print(), 500)
    } catch (e) {
      alert('Export failed: ' + e)
    }
  }

  return {
    quotations, searchQuery, statusFilter, loading, saving, exportedPath, form,
    filteredQuotations, totals, fmt, itemTotal, recalc, addItem, removeItem,
    loadQuotations, openCreateModal, editForm, resetForm,
    saveQuotation, deleteQuotation, convertToInvoice,
    exportPdfRust, exportPrintTemplate,
  }
})
