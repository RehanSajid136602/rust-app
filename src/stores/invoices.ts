import { defineStore } from 'pinia'
import { ref, computed, reactive } from 'vue'
import * as invoiceService from '../services/invoiceService'
import * as settingsService from '../services/settingsService'
import type { Invoice, InvoiceItem } from '../types'
import { printData } from '../composables/usePrint'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { nextTick } from 'vue'

export const useInvoiceStore = defineStore('invoices', () => {
  const invoices = ref<Invoice[]>([])
  const searchQuery = ref('')
  const statusFilter = ref('')
  const loading = ref(false)
  const saving = ref(false)
  const exportedPath = ref<string | null>(null)

  const emptyForm = (): Invoice => ({
    invoice_number: '', ref_number: '', client_id: null, client_name: '', client_address: '',
    invoice_date: new Date().toISOString().split('T')[0], due_date: '',
    subtotal: 0, tax_total: 0, discount_total: 0, grand_total: 0,
    amount_paid: 0, remaining_debt: 0, adjustment_label: 'Round Off',
    adjustment_amount: 0, total: 0, notes: '', status: 'unpaid', items: [],
  })

  const form = reactive<Invoice>(emptyForm())

  const filteredInvoices = computed(() => {
    let list = invoices.value
    if (statusFilter.value) list = list.filter(i => i.status === statusFilter.value)
    if (searchQuery.value) {
      const q = searchQuery.value.toLowerCase()
      list = list.filter(i =>
        i.client_name.toLowerCase().includes(q) || i.invoice_number.toLowerCase().includes(q)
      )
    }
    return list
  })

  function fmt(n: number | undefined): string {
    if (n === undefined || n === null) n = 0
    return new Intl.NumberFormat('en-IN', { minimumFractionDigits: 2 }).format(n)
  }

  const itemTotal = (item: InvoiceItem) => {
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
      subtotal,
      discountTotal: discount,
      taxTotal: tax,
      grandTotal: grand,
      netAmount: grand - (form.adjustment_amount || 0),
    }
  })

  const remainingDebt = computed(() => totals.value.netAmount - (form.amount_paid || 0))

  function recalc() {
    const t = totals.value
    form.subtotal = t.subtotal
    form.tax_total = t.taxTotal
    form.discount_total = t.discountTotal
    form.grand_total = t.grandTotal
    form.total = t.netAmount
    form.remaining_debt = remainingDebt.value
    form.status = form.remaining_debt <= 0 ? 'paid' : form.amount_paid > 0 ? 'partial' : 'unpaid'
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

  async function loadInvoices() {
    loading.value = true
    try {
      invoices.value = await invoiceService.getAllInvoices(500, 0)
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

  function editForm(inv: Invoice) {
    Object.assign(form, JSON.parse(JSON.stringify(inv)))
  }

  function resetForm() {
    Object.assign(form, emptyForm())
  }

  async function saveInvoice(): Promise<boolean> {
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
      if (form.id && form.invoice_number) {
        await invoiceService.updateInvoice({ ...form })
      } else {
        await invoiceService.createInvoice({ ...form })
      }
      await loadInvoices()
      return true
    } catch (e) {
      alert('Error: ' + e)
      return false
    } finally {
      saving.value = false
    }
  }

  async function deleteInvoice(id: number) {
    if (!confirm('Delete this invoice? This will also remove related ledger entries.')) return
    try {
      await invoiceService.deleteInvoice(id)
      await loadInvoices()
    } catch (e) {
      alert('Error: ' + e)
    }
  }

  async function recordPayment(invoiceId: number, amount: number) {
    const inv = await invoiceService.getInvoiceById(invoiceId)
    if (!inv) { alert('Invoice not found'); return }
    const newPaid = inv.amount_paid + amount
    inv.amount_paid = Math.min(newPaid, inv.total)
    inv.remaining_debt = inv.total - inv.amount_paid
    inv.status = inv.remaining_debt <= 0 ? 'paid' : 'partial'
    await invoiceService.updateInvoice(inv)
    await loadInvoices()
  }

  async function exportPdfRust() {
    recalc()
    try {
      const invoice = { ...form }
      const path = `${invoice.invoice_number || 'invoice'}.pdf`
      const result = await invoiceService.exportInvoicePdf(invoice, path)
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
        docType: 'Invoice',
        date: form.invoice_date,
        refNumber: form.invoice_number,
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
      const title = `Invoice ${form.invoice_number}`
      document.title = title
      await getCurrentWindow().setTitle(title)
      await nextTick()
      setTimeout(() => window.print(), 500)
    } catch (e) {
      alert('Export failed: ' + e)
    }
  }

  return {
    invoices,
    searchQuery,
    statusFilter,
    loading,
    saving,
    exportedPath,
    form,
    filteredInvoices,
    totals,
    remainingDebt,
    fmt,
    itemTotal,
    recalc,
    addItem,
    removeItem,
    loadInvoices,
    openCreateModal,
    editForm,
    resetForm,
    saveInvoice,
    deleteInvoice,
    recordPayment,
    exportPdfRust,
    exportPrintTemplate,
  }
})
