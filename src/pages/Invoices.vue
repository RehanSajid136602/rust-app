<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center space-x-4">
        <input v-model="searchQuery" type="text" placeholder="Search by client or invoice #..." class="input w-72" />
        <select v-model="statusFilter" class="input w-40">
          <option value="">All Status</option>
          <option value="paid">Paid</option>
          <option value="partial">Partial</option>
          <option value="unpaid">Unpaid</option>
        </select>
      </div>
      <button @click="openCreateModal" class="btn-primary">+ New Invoice</button>
    </div>

    <!-- Invoice List -->
    <div class="card">
      <div class="table-container">
        <table class="table">
          <thead class="table-header">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Invoice #</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Client</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Date</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Total</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Paid</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Balance</th>
              <th class="px-4 py-3 text-center text-xs font-medium text-gray-500 uppercase">Status</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Actions</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="inv in filteredInvoices" :key="inv.id" class="table-row">
              <td class="px-4 py-3 text-sm font-medium text-gray-900">{{ inv.invoice_number }}</td>
              <td class="px-4 py-3 text-sm text-gray-700">{{ inv.client_name }}</td>
              <td class="px-4 py-3 text-sm text-gray-600">{{ inv.invoice_date }}</td>
              <td class="px-4 py-3 text-sm text-right font-medium">{{ fmt(inv.total) }}</td>
              <td class="px-4 py-3 text-sm text-right text-green-600">{{ fmt(inv.amount_paid) }}</td>
              <td class="px-4 py-3 text-sm text-right" :class="inv.remaining_debt > 0 ? 'text-red-600' : 'text-green-600'">
                {{ fmt(inv.remaining_debt) }}
              </td>
              <td class="px-4 py-3 text-center">
                <span :class="statusBadge(inv.status)" class="px-2 py-1 text-xs font-medium rounded-full capitalize">
                  {{ inv.status }}
                </span>
              </td>
              <td class="px-4 py-3 text-right space-x-2">
                <button @click="editInvoice(inv)" class="text-primary-600 hover:text-primary-700 text-sm">Edit</button>
                <button @click="recordPayment(inv)" class="text-green-600 hover:text-green-700 text-sm">Payment</button>
                <button @click="deleteInvoice(inv.id!)" class="text-red-600 hover:text-red-700 text-sm">Delete</button>
              </td>
            </tr>
            <tr v-if="filteredInvoices.length === 0">
              <td colspan="8" class="px-6 py-8 text-center text-gray-500">No invoices found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div v-if="showForm" class="fixed inset-0 bg-black bg-opacity-50 flex items-start justify-center z-50 overflow-y-auto py-8">
      <div class="bg-white rounded-lg w-full max-w-5xl mx-4">
        <div class="flex items-center justify-between p-6 border-b">
          <h3 class="text-xl font-semibold">{{ editing ? 'Edit Invoice' : 'New Invoice' }}</h3>
          <button @click="closeForm" class="text-gray-400 hover:text-gray-600 text-2xl">&times;</button>
        </div>

        <div class="p-6 space-y-6">
          <!-- Client & Date Row -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="md:col-span-2">
              <label class="block text-sm font-medium text-gray-700 mb-1">Client *</label>
              <select v-model="form.client_id" class="input" @change="onClientChange">
                <option :value="null">Select client...</option>
                <option v-for="c in clients" :key="c.id" :value="c.id">{{ c.name }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Ref #</label>
              <input v-model="form.ref_number" type="text" class="input bg-gray-100 text-gray-500" readonly placeholder="Auto-generated" />
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Invoice Date *</label>
              <input v-model="form.invoice_date" type="date" class="input" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Due Date</label>
              <input v-model="form.due_date" type="date" class="input" />
            </div>
          </div>

          <!-- Client Address -->
          <div v-if="form.client_name" class="bg-gray-50 rounded-lg p-4">
            <p class="font-medium">{{ form.client_name }}</p>
            <p class="text-sm text-gray-600">{{ form.client_address }}</p>
          </div>

          <!-- Line Items with Autocomplete -->
          <div>
            <div class="flex items-center justify-between mb-3">
              <h4 class="font-medium text-gray-800">Line Items</h4>
              <button @click="addItem" class="text-primary-600 hover:text-primary-700 text-sm font-medium">+ Add Item</button>
            </div>
            
            <div class="table-container">
              <table class="table">
                <thead class="table-header">
                  <tr>
                    <th class="px-2 py-2 text-left text-xs font-medium text-gray-500 uppercase w-8">#</th>
                    <th class="px-2 py-2 text-left text-xs font-medium text-gray-500 uppercase min-w-[180px]">Item</th>
                    <th class="px-2 py-2 text-right text-xs font-medium text-gray-500 uppercase w-20">Qty</th>
                    <th class="px-2 py-2 text-right text-xs font-medium text-gray-500 uppercase w-24">Price</th>
                    <th class="px-2 py-2 text-right text-xs font-medium text-gray-500 uppercase w-20">Disc</th>
                    <th class="px-2 py-2 text-right text-xs font-medium text-gray-500 uppercase w-20">Tax</th>
                    <th class="px-2 py-2 text-right text-xs font-medium text-gray-500 uppercase w-24">Total</th>
                    <th class="px-2 py-2 text-center text-xs font-medium text-gray-500 uppercase w-10"></th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr v-for="(item, idx) in form.items" :key="idx">
                    <td class="px-2 py-2 text-sm text-gray-500">{{ idx + 1 }}</td>
                    <td class="px-2 py-2 relative">
                      <AutocompleteLineEdit
                        :ref="(el: any) => setAcRef(el, idx)"
                        v-model="item.item_name"
                        placeholder="Search product..."
                        :min-chars="1"
                        @select="(s: any) => onProductSelect(idx, s)"
                        @enter="focusNext(acIdx(idx, 'qty'))"
                        @tab="focusNext(acIdx(idx, 'qty'))"
                      />
                    </td>
                    <td class="px-2 py-2">
                      <input
                        :ref="(el: any) => setFieldRef(el, acIdx(idx, 'qty'))"
                        v-model.number="item.quantity"
                        type="number"
                        step="0.01"
                        class="w-full border-0 focus:ring-0 p-0 text-sm text-right"
                        @input="recalc"
                        @keydown.enter.prevent="focusNext(acIdx(idx, 'price'))"
                        @keydown.tab="focusNext(acIdx(idx, 'price'))"
                      />
                    </td>
                    <td class="px-2 py-2">
                      <input
                        :ref="(el: any) => setFieldRef(el, acIdx(idx, 'price'))"
                        v-model.number="item.price_per_unit"
                        type="number"
                        step="0.01"
                        class="w-full border-0 focus:ring-0 p-0 text-sm text-right"
                        @input="recalc"
                        @keydown.enter.prevent="focusNext(acIdx(idx, 'disc'))"
                        @keydown.tab="focusNext(acIdx(idx, 'disc'))"
                      />
                    </td>
                    <td class="px-2 py-2">
                      <input
                        :ref="(el: any) => setFieldRef(el, acIdx(idx, 'disc'))"
                        v-model.number="item.discount_amount"
                        type="number"
                        step="0.01"
                        class="w-full border-0 focus:ring-0 p-0 text-sm text-right"
                        @input="recalc"
                        @keydown.enter.prevent="focusNext(acIdx(idx, 'tax'))"
                        @keydown.tab="focusNext(acIdx(idx, 'tax'))"
                      />
                    </td>
                    <td class="px-2 py-2">
                      <input
                        :ref="(el: any) => setFieldRef(el, acIdx(idx, 'tax'))"
                        v-model.number="item.tax_amount"
                        type="number"
                        step="0.01"
                        class="w-full border-0 focus:ring-0 p-0 text-sm text-right"
                        @input="recalc"
                        @keydown.enter.prevent="focusNextRow(idx)"
                        @keydown.tab="focusNextRow(idx)"
                      />
                    </td>
                    <td class="px-2 py-2 text-sm text-right font-medium whitespace-nowrap">{{ itemTotal(item) }}</td>
                    <td class="px-2 py-2 text-center">
                      <button @click="removeItem(idx)" class="text-red-400 hover:text-red-600 text-lg leading-none">&times;</button>
                    </td>
                  </tr>
                  <tr v-if="form.items.length === 0">
                    <td colspan="8" class="px-3 py-6 text-center text-gray-400 text-sm">
                      Click "+ Add Item" to start. Type to search your product catalog.
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Totals -->
          <div class="flex justify-end">
            <div class="w-72 space-y-2">
              <div class="flex justify-between text-sm">
                <span class="text-gray-600">Subtotal</span>
                <span class="font-medium">{{ fmt(totals.subtotal) }}</span>
              </div>
              <div class="flex justify-between text-sm">
                <span class="text-gray-600">Discount</span>
                <span class="text-red-600">{{ fmt(totals.discountTotal) }}</span>
              </div>
              <div class="flex justify-between text-sm">
                <span class="text-gray-600">Tax</span>
                <span class="font-medium">{{ fmt(totals.taxTotal) }}</span>
              </div>
              <div class="border-t pt-2">
                <div class="flex items-center justify-between mb-1">
                  <span class="text-sm text-gray-600">{{ form.adjustment_label || 'Adjustment' }}</span>
                  <input v-model.number="form.adjustment_amount" type="number" step="0.01" class="w-24 border-0 focus:ring-0 p-0 text-sm text-right" @input="recalc" />
                </div>
                <input v-model="form.adjustment_label" type="text" class="w-full border-0 focus:ring-0 p-0 text-xs text-gray-400" placeholder="Label" />
              </div>
              <div class="border-t pt-2 flex justify-between">
                <span class="font-semibold">Grand Total</span>
                <span class="font-semibold text-lg">{{ fmt(totals.grandTotal) }}</span>
              </div>
              <div class="flex justify-between text-sm">
                <span class="text-gray-600">Net Amount</span>
                <span class="font-semibold">{{ fmt(totals.netAmount) }}</span>
              </div>
              <div class="border-t pt-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">Amount Paid</label>
                <input v-model.number="form.amount_paid" type="number" step="0.01" class="input" @input="recalc" />
              </div>
              <div class="flex justify-between text-sm">
                <span class="text-gray-600">Remaining</span>
                <span :class="remainingDebt > 0 ? 'text-red-600 font-semibold' : 'text-green-600 font-semibold'">{{ fmt(remainingDebt) }}</span>
              </div>
            </div>
          </div>

          <!-- Notes -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Notes</label>
            <textarea v-model="form.notes" class="input" rows="2"></textarea>
          </div>
        </div>

        <div class="flex justify-end space-x-3 p-6 border-t">
          <button v-if="editing" @click="exportPdf" class="btn-secondary-outline">
            Export PDF
          </button>
          <button @click="closeForm" class="btn-secondary">Cancel</button>
          <button @click="saveInvoice" class="btn-primary" :disabled="saving">
            {{ saving ? 'Saving...' : (editing ? 'Update Invoice' : 'Create Invoice') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Payment Modal -->
    <div v-if="showPayment" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-lg p-6 w-full max-w-sm">
        <h3 class="text-lg font-semibold mb-4">Record Payment</h3>
        <p class="text-sm text-gray-600 mb-4">
          Invoice: {{ paymentForm.invoice_number }}<br/>
          Total: {{ fmt(paymentForm.total) }} | Paid: {{ fmt(paymentForm.amount_paid) }}<br/>
          Remaining: {{ fmt(paymentForm.total - paymentForm.amount_paid) }}
        </p>
        <div class="space-y-3">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Amount *</label>
            <input v-model.number="paymentForm.amount" type="number" step="0.01" class="input" />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">Date</label>
            <input v-model="paymentForm.date" type="date" class="input" />
          </div>
        </div>
        <div class="flex justify-end space-x-3 mt-6">
          <button @click="showPayment = false" class="btn-secondary">Cancel</button>
          <button @click="savePayment" class="btn-primary">Record Payment</button>
        </div>
      </div>
    </div>
    <!-- PDF Export Success Dialog -->
    <ExportSuccessDialog
      v-if="exportedPath"
      :file-path="exportedPath"
      @close="exportedPath = null"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import AutocompleteLineEdit from '../components/AutocompleteLineEdit.vue'
import type { Suggestion } from '../components/AutocompleteLineEdit.vue'
import { printData } from '../composables/usePrint'
import ExportSuccessDialog from '../components/ExportSuccessDialog.vue'

interface InvoiceItem {
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

interface Invoice {
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

interface Client {
  id: number
  name: string
  address: string
}

const invoices = ref<Invoice[]>([])
const clients = ref<Client[]>([])
const searchQuery = ref('')
const statusFilter = ref('')
const showForm = ref(false)
const showPayment = ref(false)
const editing = ref(false)
const saving = ref(false)
const exportedPath = ref<string | null>(null)

// Refs for field navigation
const acRefs = ref<Record<string, any>>({})
const fieldRefs = ref<Record<string, HTMLElement>>({})

const setAcRef = (el: any, idx: number) => { acRefs.value[`ac_${idx}`] = el }
const setFieldRef = (el: any, key: string) => { if (el) fieldRefs.value[key] = el }
const acIdx = (idx: number, field: string) => `item_${idx}_${field}`

const emptyForm = (): Invoice => ({
  invoice_number: '', ref_number: '', client_id: null, client_name: '', client_address: '',
  invoice_date: new Date().toISOString().split('T')[0], due_date: '',
  subtotal: 0, tax_total: 0, discount_total: 0, grand_total: 0,
  amount_paid: 0, remaining_debt: 0, adjustment_label: 'Round Off',
  adjustment_amount: 0, total: 0, notes: '', status: 'unpaid', items: [],
})

const form = reactive<Invoice>(emptyForm())

const paymentForm = reactive({
  invoice_id: 0, invoice_number: '', total: 0, amount_paid: 0,
  amount: 0, date: new Date().toISOString().split('T')[0],
})

const fmt = (n: number | undefined): string => {
  if (n === undefined || n === null) n = 0
  return new Intl.NumberFormat('en-IN', { minimumFractionDigits: 2 }).format(n)
}

const statusBadge = (s: string): string => {
  const map: Record<string, string> = {
    paid: 'bg-green-50 text-green-700', partial: 'bg-yellow-50 text-yellow-700', unpaid: 'bg-red-50 text-red-700',
  }
  return map[s] || 'bg-gray-100 text-gray-600'
}

const itemTotal = (item: any) => {
  const n = item.quantity * item.price_per_unit - item.discount_amount + item.tax_amount
  return fmt(n)
}

const totals = computed(() => {
  let subtotal = 0, discount = 0, tax = 0
  for (const item of form.items) {
    subtotal += item.quantity * item.price_per_unit
    discount += item.discount_amount
    tax += item.tax_amount
  }
  const grand = subtotal - discount + tax
  return { subtotal, discountTotal: discount, taxTotal: tax, grandTotal: grand, netAmount: grand - (form.adjustment_amount || 0) }
})

const remainingDebt = computed(() => totals.value.netAmount - (form.amount_paid || 0))

const filteredInvoices = computed(() => {
  let list = invoices.value
  if (statusFilter.value) list = list.filter(i => i.status === statusFilter.value)
  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase()
    list = list.filter(i => i.client_name.toLowerCase().includes(q) || i.invoice_number.toLowerCase().includes(q))
  }
  return list
})

// ---- Navigation helpers ----
const focusNext = (key: string) => {
  nextTick(() => {
    const el = fieldRefs.value[key]
    if (el) {
      el.focus()
      if (el instanceof HTMLInputElement) el.select()
    }
  })
}

const focusNextRow = (idx: number) => {
  addItemIfLast(idx)
  // Focus the autocomplete of the next row
  nextTick(() => {
    const nextAc = acRefs.value[`ac_${idx + 1}`]
    if (nextAc) {
      nextAc.focus()
    }
  })
}

const onProductSelect = (idx: number, s: Suggestion) => {
  const item = form.items[idx]
  item.item_name = s.name
  item.price_per_unit = s.data?.price_per_unit ?? item.price_per_unit
  recalc()
  focusNext(acIdx(idx, 'qty'))
}

const addItemIfLast = (idx: number) => {
  if (idx === form.items.length - 1) {
    addItem()
  }
}

// ---- Data loading ----
const loadInvoices = async () => {
  try { invoices.value = await invoke<Invoice[]>('get_all_invoices', { limit: 200, offset: 0 }) } catch (e) { console.error(e) }
}

const loadClients = async () => {
  try { clients.value = await invoke<Client[]>('get_all_clients', { limit: 200, offset: 0 }) } catch (e) { console.error(e) }
}

const onClientChange = () => {
  const c = clients.value.find(c => c.id === form.client_id)
  if (c) { form.client_name = c.name; form.client_address = c.address || '' }
}

const openCreateModal = async () => {
  editing.value = false
  Object.assign(form, emptyForm())
  try {
    form.ref_number = await invoke<string>('generate_ref_number')
  } catch { /* fallback to empty */ }
  showForm.value = true
}

const editInvoice = (inv: Invoice) => {
  editing.value = true
  Object.assign(form, JSON.parse(JSON.stringify(inv)))
  showForm.value = true
}

const closeForm = () => { showForm.value = false }

const addItem = () => {
  form.items.push({ sno: form.items.length + 1, item_name: '', quantity: 1, price_per_unit: 0, discount_amount: 0, tax_amount: 0, total_price: 0 })
}

const removeItem = (idx: number) => {
  form.items.splice(idx, 1)
  form.items.forEach((item, i) => item.sno = i + 1)
}

const recalc = () => {
  const t = totals.value
  form.subtotal = t.subtotal; form.tax_total = t.taxTotal; form.discount_total = t.discountTotal
  form.grand_total = t.grandTotal; form.total = t.netAmount
  form.remaining_debt = remainingDebt.value
  form.status = form.remaining_debt <= 0 ? 'paid' : form.amount_paid > 0 ? 'partial' : 'unpaid'
}

const saveInvoice = async () => {
  if (!form.client_name || form.items.length === 0) { alert('Client name and at least one item are required.'); return }
  if (form.items.some(i => !i.item_name.trim())) { alert('All items must have a name.'); return }
  recalc()
  saving.value = true
  try {
    if (editing.value) await invoke('update_invoice', { invoice: { ...form } })
    else await invoke('create_invoice', { invoice: { ...form } })
    closeForm(); await loadInvoices()
  } catch (e) { alert('Error: ' + e) }
  finally { saving.value = false }
}

const recordPayment = (inv: Invoice) => {
  paymentForm.invoice_id = inv.id!; paymentForm.invoice_number = inv.invoice_number
  paymentForm.total = inv.total; paymentForm.amount_paid = inv.amount_paid
  paymentForm.amount = inv.remaining_debt; paymentForm.date = new Date().toISOString().split('T')[0]
  showPayment.value = true
}

const savePayment = async () => {
  if (paymentForm.amount <= 0) { alert('Enter a valid payment amount.'); return }
  try {
    const inv = await invoke<Invoice>('get_invoice_by_id', { id: paymentForm.invoice_id })
    if (!inv) { alert('Invoice not found'); return }
    const newPaid = inv.amount_paid + paymentForm.amount
    inv.amount_paid = Math.min(newPaid, inv.total)
    inv.remaining_debt = inv.total - inv.amount_paid
    inv.status = inv.remaining_debt <= 0 ? 'paid' : 'partial'
    await invoke('update_invoice', { invoice: inv })
    showPayment.value = false; await loadInvoices()
  } catch (e) { alert('Error: ' + e) }
}

const deleteInvoice = async (id: number) => {
  if (!confirm('Delete this invoice? This will also remove related ledger entries.')) return
  try { await invoke('delete_invoice', { id }); await loadInvoices() } catch (e) { alert('Error: ' + e) }
}

const exportPdf = async () => {
  if (!editing.value) {
    alert('Please save the invoice before exporting.')
    return
  }
  recalc()
  try {
    const settings = await invoke<any>('get_company_settings')
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
    document.title = `Invoice ${form.invoice_number}`
    await nextTick()
    setTimeout(() => window.print(), 500)
  } catch (e) {
    alert('Export failed: ' + e)
  }
}

onMounted(() => { loadInvoices(); loadClients() })
</script>
