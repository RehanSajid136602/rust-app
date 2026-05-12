<template>
  <div v-if="data" id="invoice-print-root" class="invoice-print-root">
    <!-- HEADER IMAGE -->
    <img
      src="/header.png"
      alt="header"
      class="header-img"
      @load="onImageLoad"
      @error="onImageError"
    />

    <!-- DATE + REF TABLE -->
    <table class="info-table">
      <tbody>
        <tr>
          <td class="info-label">Dated:</td>
          <td class="info-value">{{ formatDate(data.date) }}</td>
        </tr>
        <tr>
          <td class="info-label">Ref #</td>
          <td class="info-value">{{ data.refNumber || '-' }}</td>
        </tr>
      </tbody>
    </table>

    <!-- DOC HEADING -->
    <h2 class="doc-title">{{ data.docType }}</h2>

    <!-- SALUTATION -->
    <p class="salutation">{{ data.salutation || 'Respected Sir,' }}</p>
    <p class="body-text">{{ data.bodyText || '' }}</p>

    <!-- MAIN ITEMS TABLE -->
    <table class="items-table">
      <thead>
        <tr>
          <th class="th-invoice" colspan="5">{{ data.docType }}</th>
        </tr>
        <tr class="header-row">
          <th>S. No</th>
          <th>Items Details</th>
          <th>Quantity</th>
          <th>Price/Unit</th>
          <th>Total Price</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in data.items" :key="item.sno">
          <td class="td-center">{{ item.sno }}</td>
          <td class="td-left">{{ item.item_name }}</td>
          <td class="td-center">{{ fmt(item.quantity) }}</td>
          <td class="td-center">{{ fmt(item.price_per_unit) }}</td>
          <td class="td-center td-bold">{{ fmt(item.quantity * item.price_per_unit) }}</td>
        </tr>

        <!-- TOTAL ROW -->
        <tr class="total-row">
          <td colspan="3" class="td-blank"></td>
          <td class="td-label">Total Amount</td>
          <td class="td-value td-bold">{{ formatPKR(data.subtotal) }}</td>
        </tr>

        <!-- ADJUSTMENT ROW -->
        <tr v-if="data.adjustmentAmount" class="adjust-row">
          <td colspan="3" class="td-blank"></td>
          <td class="td-label">{{ data.adjustmentLabel || 'Adjustment' }}</td>
          <td class="td-value">{{ formatPKR(data.adjustmentAmount) }}</td>
        </tr>

        <!-- NET AMOUNT ROW -->
        <tr class="net-row">
          <td colspan="4" class="td-blank"></td>
          <td class="td-value td-bold td-large">{{ formatPKR(data.total) }}</td>
        </tr>
      </tbody>
    </table>

    <!-- FOOTER IMAGE -->
    <img
      src="/footer.png"
      alt="footer"
      class="footer-img"
      @load="onImageLoad"
      @error="onImageError"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface PrintItem {
  sno: number
  item_name: string
  quantity: number
  price_per_unit: number
}

interface PrintData {
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

const props = defineProps<{
  data: PrintData | null
}>()

const imagesLoaded = ref(0)
const imagesToLoad = ref(2)
const emit = defineEmits<{
  ready: []
}>()

const onImageLoad = () => {
  imagesLoaded.value++
  if (imagesLoaded.value >= imagesToLoad.value) {
    emit('ready')
  }
}

const onImageError = () => {
  imagesLoaded.value++
  if (imagesLoaded.value >= imagesToLoad.value) {
    emit('ready')
  }
}

watch(() => props.data, (newData) => {
  if (newData) {
    imagesLoaded.value = 0
  }
})

const fmt = (n: number): string => {
  return new Intl.NumberFormat('en-IN', { minimumFractionDigits: 2, maximumFractionDigits: 2 }).format(n)
}

const formatPKR = (n: number): string => {
  return `Rs. ${fmt(n)}/-`
}

const formatDate = (dateStr: string): string => {
  if (!dateStr) return ''
  const parts = dateStr.split('-')
  if (parts.length === 3 && parts[0].length === 4) {
    // ISO format YYYY-MM-DD
    return `${parts[2]}-${parts[1]}-${parts[0]}`
  }
  return dateStr
}
</script>

<style scoped>
.invoice-print-root {
  font-family: 'Times New Roman', Times, serif;
  font-size: 11pt;
  color: #000;
  width: 210mm;
  margin: 0 auto;
  padding: 0;
  background: #fff;
  box-sizing: border-box;
}

.header-img,
.footer-img {
  width: 100%;
  display: block;
}

.header-img {
  margin-bottom: 12px;
}

.footer-img {
  margin-top: 28px;
}

.info-table {
  border: 1px solid #000;
  border-collapse: collapse;
  margin-bottom: 16px;
  font-size: 10pt;
  width: auto;
}

.info-table td {
  padding: 4px 12px;
  border: 1px solid #000;
}

.info-label {
  font-weight: bold;
}

.info-value {
  font-weight: bold;
  min-width: 120px;
}

.doc-title {
  text-align: center;
  font-weight: bold;
  font-size: 14pt;
  margin-bottom: 12px;
  font-family: 'Times New Roman', Times, serif;
}

.salutation {
  margin-bottom: 8px;
}

.body-text {
  margin-bottom: 16px;
}

.items-table {
  width: 100%;
  border-collapse: collapse;
  border: 1px solid #000;
  margin-bottom: 16px;
  font-size: 10pt;
}

.items-table th,
.items-table td {
  border: 1px solid #000;
  padding: 5px 8px;
}

.th-invoice {
  text-align: center;
  font-weight: bold;
  font-size: 12pt;
  padding: 6px;
}

.header-row {
  background: #f5f5f5;
}

.header-row th {
  font-weight: bold;
  text-align: center;
}

.td-center {
  text-align: center;
}

.td-left {
  text-align: left;
}

.td-bold {
  font-weight: bold;
}

.td-large {
  font-size: 11pt;
}

.td-blank {
  border: 1px solid #000;
}

.total-row .td-label,
.adjust-row .td-label,
.net-row .td-label {
  text-align: center;
  font-weight: bold;
}

.total-row .td-value,
.adjust-row .td-value,
.net-row .td-value {
  text-align: center;
  font-weight: bold;
}

/* Print-specific overrides */
@media print {
  .invoice-print-root {
    width: 100%;
    padding: 0;
    margin: 0;
  }
}
</style>
